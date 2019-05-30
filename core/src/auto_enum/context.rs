use std::{
    cell::{Cell, RefCell},
    collections::hash_map::DefaultHasher,
    hash::Hasher,
    iter,
    sync::atomic::{AtomicUsize, Ordering},
};

use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{Attribute, Expr, ItemEnum, Macro, Result};

use crate::utils::{expr_call, ident, path};

use super::{
    visitor::{Dummy, FindTry, Visitor},
    *,
};

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
    /// `Stmt::Semi(..)` or `never` option - never visit last expr
    Never,
}

pub(super) struct Context {
    /// Span passed to `syn::Error::new_spanned`.
    span: Option<TokenStream>,
    pub(super) args: Vec<Arg>,
    builder: Builder,
    pub(super) marker: Marker,
    // pub(super) depth: usize,
    root: bool,
    visit_mode: VisitMode,
    visit_last_mode: VisitLastMode,
    /// This is `true` if other `auto_enum` attribute exists in this attribute's scope.
    pub(super) other_attr: bool,
    /// This is `true` if an error occurred in visiting.
    pub(super) error: bool,
}

impl Context {
    fn new<T: ToTokens>(
        span: T,
        args: Vec<Arg>,
        marker: Option<String>,
        never: bool,
        root: bool,
    ) -> Self {
        Self {
            span: Some(span.into_token_stream()),
            args,
            builder: Builder::new(),
            marker: Marker::new(marker),
            // depth: 0,
            root,
            visit_mode: VisitMode::Default,
            visit_last_mode: if never { VisitLastMode::Never } else { VisitLastMode::Default },
            other_attr: false,
            error: false,
        }
    }

    pub(super) fn root<T: ToTokens>(
        span: T,
        (args, marker, never): (Vec<Arg>, Option<String>, bool),
    ) -> Self {
        Self::new(span, args, marker, never, true)
    }

    pub(super) fn child<T: ToTokens>(
        span: T,
        (args, marker, never): (Vec<Arg>, Option<String>, bool),
    ) -> Self {
        Self::new(span, args, marker, never, false)
    }

    pub(super) fn span(&mut self) -> TokenStream {
        // If this is called more than once, it is a bug.
        self.span.take().unwrap_or_else(|| unreachable!())
    }

    pub(super) const fn visit_mode(&self) -> VisitMode {
        self.visit_mode
    }

    pub(super) fn set_visit_mode(&mut self, mode: VisitMode) {
        self.visit_mode = mode;
    }

    pub(super) fn set_visit_last_mode(&mut self, mode: VisitLastMode) {
        self.visit_last_mode = mode;
    }

    pub(super) fn visit_last(&self) -> bool {
        self.visit_last_mode != VisitLastMode::Never && self.visit_mode != VisitMode::Try
    }

    // visitors

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
            self.visit_mode = VisitMode::Try;
        }
    }

    // utils

    pub(super) fn next_expr(&mut self, expr: Expr) -> Expr {
        self.next_expr_with_attrs(Vec::new(), expr)
    }

    pub(super) fn next_expr_with_attrs(&mut self, attrs: Vec<Attribute>, expr: Expr) -> Expr {
        self.builder.next_expr(attrs, expr)
    }

    pub(super) fn is_marker_expr(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Macro(expr) => self.is_marker_macro(&expr.mac),
            _ => false,
        }
    }

    pub(super) fn is_marker_macro(&self, Macro { path, .. }: &Macro) -> bool {
        match &self.marker.ident {
            None => path.is_ident(Marker::DEFAULT),
            Some(marker) => path.is_ident(marker) || (!self.root && path.is_ident(Marker::DEFAULT)),
        }
    }

    pub(super) fn replace_boxed_expr(&mut self, expr: &mut Option<Box<Expr>>) {
        if expr.is_none() {
            expr.replace(Box::new(unit()));
        }

        if let Some(expr) = expr {
            replace_expr(&mut **expr, |expr| {
                if self.is_marker_expr(&expr) {
                    // Skip if `<expr>` is a marker macro.
                    expr
                } else {
                    self.next_expr(expr)
                }
            });
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

    // build

    fn buildable(&mut self) -> Result<bool> {
        fn err(cx: &mut Context, len: usize) -> Result<bool> {
            let (msg1, msg2) = match cx.visit_last_mode {
                VisitLastMode::Default | VisitLastMode::Closure => {
                    ("branches or marker macros in total", "branch or marker macro")
                }
                VisitLastMode::Never => ("marker macros", "marker macro"),
            };

            Err(error!(cx.span(),
                "the `#[auto_enum]` attribute is required two or more {}, there is {} {} in this statement",
                msg1,
                if len == 0 { "no" } else { "only one" },
                msg2
            ))?
        }

        if self.error {
            // As we know that an error will occur, it does not matter if there are not enough variants.
            Ok(true)
        } else {
            match self.builder.variants.len() {
                1 => err(self, 1),
                0 if !self.other_attr => err(self, 0),
                0 => Ok(false),
                _ => Ok(true),
            }
        }
    }

    pub(super) fn build<F: FnOnce(ItemEnum)>(&mut self, f: F) -> Result<()> {
        self.buildable().map(|buildable| {
            if buildable {
                f(self.builder.build(&self.args))
            }
        })
    }

    // type_analysis feature

    #[cfg(feature = "type_analysis")]
    pub(super) fn impl_traits(&mut self, ty: &mut Type) {
        collect_impl_traits(&mut self.args, ty);
    }
}

// =============================================================================
// Expression level marker

pub(super) struct Marker {
    ident: Option<String>,
}

impl Marker {
    const DEFAULT: &'static str = "marker";

    const fn new(ident: Option<String>) -> Self {
        Self { ident }
    }

    pub(super) fn is_unique(&self) -> bool {
        self.ident.is_some()
    }

    pub(super) fn ident(&self) -> &str {
        self.ident.as_ref().map_or(Self::DEFAULT, |s| s)
    }
}

// =============================================================================
// Enum builder

struct Builder {
    ident: String,
    variants: Vec<String>,
}

impl Builder {
    fn new() -> Self {
        Self {
            ident: format!("___Enum{}", RNG.with(|rng| rng.borrow_mut().next())),
            variants: Vec::new(),
        }
    }

    fn iter(&self) -> impl Iterator<Item = Ident> + '_ {
        self.variants.iter().map(ident)
    }

    fn next_expr(&mut self, attrs: Vec<Attribute>, expr: Expr) -> Expr {
        let variant = format!("___Variant{}", self.variants.len());

        let path =
            path(iter::once(ident(&self.ident).into()).chain(iter::once(ident(&variant).into())));

        self.variants.push(variant);

        expr_call(attrs, path, expr)
    }

    fn build(&self, args: &[Arg]) -> ItemEnum {
        let ident = ident(&self.ident);
        let ty_generics = self.iter();
        let variants = self.iter();
        let fields = self.iter();

        syn::parse_quote! {
            #[::auto_enums::enum_derive(#(#args),*)]
            enum #ident<#(#ty_generics),*> {
                #(#variants(#fields),)*
            }
        }
    }
}

// =============================================================================
// RNG

thread_local! {
    static RNG: RefCell<XorShift64Star> = RefCell::new(XorShift64Star::new());
}

// https://github.com/rayon-rs/rayon/blob/rayon-core-v1.4.1/rayon-core/src/registry.rs#L712-L750

/// [xorshift*] is a fast pseudorandom number generator which will
/// even tolerate weak seeding, as long as it's not zero.
///
/// [xorshift*]: https://en.wikipedia.org/wiki/Xorshift#xorshift*
struct XorShift64Star {
    state: Cell<u64>,
}

impl XorShift64Star {
    fn new() -> Self {
        // Any non-zero seed will do -- this uses the hash of a global counter.
        let mut seed = 0;
        while seed == 0 {
            static COUNTER: AtomicUsize = AtomicUsize::new(0);
            let mut hasher = DefaultHasher::new();
            hasher.write_usize(COUNTER.fetch_add(1, Ordering::Relaxed));
            seed = hasher.finish();
        }

        Self { state: Cell::new(seed) }
    }

    fn next(&self) -> u64 {
        let mut x = self.state.get();
        debug_assert_ne!(x, 0);
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state.set(x);
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }
}
