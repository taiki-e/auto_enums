use std::cell::RefCell;

use proc_macro2::Ident;
use quote::quote;
use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use smallvec::{smallvec, SmallVec};
use syn::{Attribute, Expr, ExprCall, ExprPath, ItemEnum, Macro};

use super::*;

fn xorshift_rng() -> XorShiftRng {
    const SEED: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    XorShiftRng::from_seed(SEED)
}

thread_local! {
    static RNG: RefCell<XorShiftRng> = RefCell::new(xorshift_rng());
}

// =============================================================================
// Context

/// Config for related to `visotor::Visotor` type.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum VisitMode {
    Default,
    Return,
    Try,
}

/// Config for related to `expr::VisitLast` trait.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum VisitLastMode {
    Default,
    /*
    local `let .. = || {};`
    or
    expr `|| {}`
    not
    item_fn `fn _() -> Fn*() { || {} }`
    */
    Closure,
    /// `Stmt::Semi(_, _)` or `never` option - never visit last expr
    Never,
}

pub(super) struct Context {
    pub(super) args: Stack<Arg>,
    builder: Builder,
    pub(super) marker: Marker,
    // pub(super) depth: usize,
    root: bool,
    pub(super) attr: bool,
    mode: VisitMode,
    visit_last: VisitLastMode,
    /// This in `true` if error occurred in visiting.
    pub(super) error: bool,
}

// This has been fixed in https://github.com/rust-lang/rust-clippy/pull/3869. Allow it temporarily until it lands.
#[allow(clippy::use_self)]
impl Context {
    fn new(args: Stack<Arg>, marker: Marker, never: bool, root: bool) -> Self {
        Self {
            args,
            builder: Builder::new(),
            marker,
            // depth: 0,
            root,
            attr: false,
            mode: VisitMode::Default,
            visit_last: if never {
                VisitLastMode::Never
            } else {
                VisitLastMode::Default
            },
            error: false,
        }
    }

    pub(super) fn root(args: Stack<Arg>, marker: Marker, never: bool) -> Self {
        Self::new(args, marker, never, true)
    }

    pub(super) fn child(args: Stack<Arg>, marker: Marker, never: bool) -> Self {
        Self::new(args, marker, never, false)
    }

    pub(super) const fn mode(&self) -> VisitMode {
        self.mode
    }

    pub(super) fn visit_mode(&mut self, mode: VisitMode) {
        self.mode = mode;
    }

    pub(super) fn visit_last_mode(&mut self, visit_last: VisitLastMode) {
        self.visit_last = visit_last;
    }

    pub(super) fn visit_last(&self) -> bool {
        self.visit_last != VisitLastMode::Never && self.mode != VisitMode::Try
    }

    pub(super) fn visitor<F: FnOnce(&mut Visitor<'_>)>(&mut self, f: F) {
        f(&mut Visitor::new(self));
    }

    pub(super) fn dummy<F: FnOnce(&mut Dummy<'_>)>(&mut self, f: F) {
        f(&mut Dummy::new(self));
    }

    pub(super) fn find_try<F: FnOnce(&mut FindTry<'_>)>(&mut self, f: F) {
        let mut find = FindTry::new(self);
        f(&mut find);
        if find.has {
            self.mode = VisitMode::Try;
        }
    }

    pub(super) fn next_expr(&mut self, expr: Expr) -> Expr {
        self.next_expr_with_attrs(Vec::with_capacity(0), expr)
    }

    pub(super) fn next_expr_with_attrs(&mut self, attrs: Vec<Attribute>, expr: Expr) -> Expr {
        self.builder.next_expr(attrs, expr)
    }

    pub(super) fn marker_macro(&self, Macro { path, .. }: &Macro) -> bool {
        match &self.marker.ident {
            None => path.is_ident(DEFAULT_MARKER),
            Some(marker) => path.is_ident(marker) || (!self.root && path.is_ident(DEFAULT_MARKER)),
        }
    }

    /* FIXME: This may not be necessary.
    pub(super) fn assigned_enum(&self, ExprCall { args, func, .. }: &ExprCall) -> bool {
        args.len() == 1
            && match &**func {
                Expr::Path(ExprPath {
                    path, qself: None, ..
                }) => {
                    path.leading_colon.is_none()
                        && path.segments.len() == 2
                        && path.segments[0].arguments.is_empty()
                        && path.segments[1].arguments.is_empty()
                        && path.segments[0].ident == self.builder.ident
                }
                _ => false,
            }
    }
    */

    pub(super) fn buildable(&self) -> Result<bool> {
        fn err(cx: &Context, len: usize) -> Result<bool> {
            let (msg1, msg2) = match cx.visit_last {
                VisitLastMode::Default | VisitLastMode::Closure => (
                    "branches or marker macros in total",
                    "branch or marker macro",
                ),
                VisitLastMode::Never => ("marker macros", "marker macro"),
            };

            Err(format!(
                "the `#[auto_enum]` attribute is required two or more {}, there is {} {} in this statement",
                msg1,
                if len == 0 { "no" } else { "only one" },
                msg2
            ))?
        }

        if self.error {
            Ok(false)
        } else {
            match self.builder.len() {
                1 => err(self, 1),
                0 if !self.attr => err(self, 0),
                0 => Ok(false),
                _ => Ok(true),
            }
        }
    }

    pub(super) fn build(&self) -> ItemEnum {
        self.builder.build(&self.args)
    }

    #[cfg(feature = "type_analysis")]
    pub(super) fn impl_traits(&mut self, ty: &mut Type) {
        collect_impl_traits(&mut self.args, ty);
    }
}

// =============================================================================
// Expression level marker

pub(super) const DEFAULT_MARKER: &str = "marker";

pub(super) struct Marker {
    ident: Option<String>,
}

impl Marker {
    pub(super) const fn new(ident: Option<String>) -> Self {
        Self { ident }
    }

    pub(super) fn is_unique(&self) -> bool {
        self.ident.is_some()
    }

    pub(super) fn ident(&self) -> &str {
        self.ident.as_ref().map_or(DEFAULT_MARKER, |s| s)
    }
}

// =============================================================================
// Enum builder

struct Builder {
    ident: String,
    variants: Stack<String>,
}

impl Builder {
    fn new() -> Self {
        Self {
            ident: format!("___Enum{}", RNG.with(|rng| rng.borrow_mut().next_u32())),
            variants: Stack::new(),
        }
    }

    fn len(&self) -> usize {
        self.variants.len()
    }

    fn iter(&self) -> impl Iterator<Item = Ident> + '_ {
        self.variants.iter().map(ident)
    }

    fn push_variant(&mut self) {
        let variant = format!("___Variant{}", self.len());
        self.variants.push(variant);
    }

    fn last_expr(&self, attrs: Vec<Attribute>, expr: Expr) -> Expr {
        assert!(!self.variants.is_empty());

        let segments: SmallVec<[_; 2]> = smallvec![
            ident(&self.ident).into(),
            ident(self.variants.last().unwrap()).into()
        ];

        Expr::Call(ExprCall {
            attrs,
            func: Box::new(Expr::Path(ExprPath {
                attrs: Vec::with_capacity(0),
                qself: None,
                path: path(segments),
            })),
            paren_token: default(),
            args: Some(expr).into_iter().collect(),
        })
    }

    fn next_expr(&mut self, attrs: Vec<Attribute>, expr: Expr) -> Expr {
        self.push_variant();
        self.last_expr(attrs, expr)
    }

    fn build(&self, args: &[Arg]) -> ItemEnum {
        assert!(self.len() >= 2);

        let ident = ident(&self.ident);
        let ty_generics = self.iter();
        let variants = self.iter();
        let fields = self.iter();

        syn::parse2(quote! {
            #[::auto_enums::enum_derive(#(#args),*)]
            enum #ident<#(#ty_generics),*> {
                #(#variants(#fields),)*
            }
        })
        .unwrap_or_else(|_| unreachable!()) // If this fails, it's definitely a bug.
    }
}
