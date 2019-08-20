use std::{
    cell::{Cell, RefCell},
    collections::hash_map::DefaultHasher,
    hash::Hasher,
    iter,
    num::Wrapping,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use syn::{Attribute, Error, Expr, Ident, ItemEnum, Macro, Path, Result};

use crate::utils::{expr_call, path, replace_expr, unit};

#[cfg(feature = "type_analysis")]
use super::traits::*;
use super::{
    visitor::{Dummy, FindTry, Visitor},
    Args,
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
    local: `let .. = || {};`
    or
    expr: `|| {}`
    not
    item_fn: `fn _() -> Fn*() { || {} }`
    */
    Closure,
    /// `Stmt::Semi(..)` - never visit last expr
    Never,
}

#[derive(Clone, Default)]
pub(super) struct Diagnostic {
    message: Rc<RefCell<Option<Error>>>,
}

impl Diagnostic {
    pub(super) fn error(&self, message: Error) {
        let mut base = self.message.borrow_mut();
        if let Some(base) = &mut *base {
            base.combine(message)
        } else {
            *base = Some(message)
        }
    }

    pub(super) fn combine(self) -> Option<Error> {
        Rc::try_unwrap(self.message)
            .expect("Called Diagnostic::combine in a non-root context")
            .into_inner()
    }
}

pub(super) struct Context {
    builder: Builder,
    pub(super) args: Vec<Path>,
    pub(super) marker: Marker,
    root: bool,
    // pub(super) depth: u32,
    pub(super) visit_mode: VisitMode,
    pub(super) visit_last_mode: VisitLastMode,
    /// This is `true` if other `auto_enum` attribute exists in this attribute's scope.
    pub(super) other_attr: bool,
    /// Span passed to `syn::Error::new_spanned`.
    pub(super) span: TokenStream,
    pub(super) diagnostic: Diagnostic,
}

impl Context {
    fn new(span: impl ToTokens, (args, marker): Args, root: bool, diagnostic: Diagnostic) -> Self {
        Self {
            builder: Builder::new(),
            args,
            marker: Marker::new(marker),
            root,
            // depth: 0,
            visit_mode: VisitMode::Default,
            visit_last_mode: VisitLastMode::Default,
            other_attr: false,
            span: span.into_token_stream(),
            diagnostic,
        }
    }

    pub(super) fn root(span: impl ToTokens, args: Args) -> Self {
        Self::new(span, args, true, Diagnostic::default())
    }

    pub(super) fn make_child(&self, span: impl ToTokens, args: Args) -> Self {
        Self::new(span, args, false, self.diagnostic.clone())
    }

    /// This is `true` if errors occurred.
    pub(super) fn failed(&self) -> bool {
        self.diagnostic.message.borrow().is_some()
    }

    pub(super) fn visit_last(&self) -> bool {
        self.visit_last_mode != VisitLastMode::Never && self.visit_mode != VisitMode::Try
    }

    // visitors

    pub(super) fn visitor(&mut self, f: impl FnOnce(&mut Visitor<'_>)) {
        f(&mut Visitor::new(self));
    }

    pub(super) fn dummy(&mut self, f: impl FnOnce(&mut Dummy<'_>)) {
        f(&mut Dummy::new(self));
    }

    pub(super) fn find_try(&mut self, f: impl FnOnce(&mut FindTry<'_>)) {
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
            Expr::Macro(expr) => self.is_marker_macro(&expr.mac, false),
            _ => false,
        }
    }

    pub(super) fn is_marker_macro(&self, Macro { path, .. }: &Macro, exact: bool) -> bool {
        match &self.marker.ident {
            None => path.is_ident(Marker::DEFAULT),
            Some(marker) => {
                path.is_ident(marker) || (!exact && !self.root && path.is_ident(Marker::DEFAULT))
            }
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

    // build

    pub(super) fn build(&mut self, f: impl FnOnce(ItemEnum)) -> Result<()> {
        fn err(cx: &Context) -> Error {
            let (msg1, msg2) = match cx.visit_last_mode {
                VisitLastMode::Default | VisitLastMode::Closure => {
                    ("branches or marker macros in total", "branch or marker macro")
                }
                VisitLastMode::Never => ("marker macros", "marker macro"),
            };

            error!(
                cx.span,
                "`#[auto_enum]` is required two or more {}, there is {} {} in this statement",
                msg1,
                if cx.builder.variants.is_empty() { "no" } else { "only one" },
                msg2
            )
        }

        // As we know that an error will occur, it does not matter if there are not enough variants.
        if !self.failed() {
            match self.builder.variants.len() {
                1 => return Err(err(self)),
                0 if !self.other_attr => return Err(err(self)),
                _ => {}
            }
        }

        if !self.builder.variants.is_empty() {
            f(self.builder.build(&self.args))
        }
        Ok(())
    }

    // type_analysis feature

    #[cfg(feature = "type_analysis")]
    pub(super) fn collect_trait(&mut self, ty: &mut Type) {
        collect_impl_trait(&mut self.args, ty);
    }
}

// =============================================================================
// Expression level marker

pub(super) struct Marker {
    ident: Option<String>,
}

impl Marker {
    const DEFAULT: &'static str = "marker";

    fn new(ident: Option<String>) -> Self {
        Self { ident }
    }

    pub(super) fn is_unique(&self) -> bool {
        self.ident.is_some()
    }
}

// =============================================================================
// Enum builder

struct Builder {
    ident: Ident,
    variants: Vec<Ident>,
}

impl Builder {
    fn new() -> Self {
        Self { ident: format_ident!("___Enum{}", random()), variants: Vec::new() }
    }

    fn iter(&self) -> impl Iterator<Item = &Ident> + '_ {
        self.variants.iter()
    }

    fn next_expr(&mut self, attrs: Vec<Attribute>, expr: Expr) -> Expr {
        let variant = format_ident!("___Variant{}", self.variants.len());

        let path =
            path(iter::once(self.ident.clone().into()).chain(iter::once(variant.clone().into())));

        self.variants.push(variant);

        expr_call(attrs, path, expr)
    }

    fn build(&self, args: &[Path]) -> ItemEnum {
        let ident = &self.ident;
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

/// Pseudorandom number generator based on [xorshift*].
///
/// [xorshift*]: https://en.wikipedia.org/wiki/Xorshift#xorshift*
fn random() -> u64 {
    thread_local! {
        static RNG: Cell<Wrapping<u64>> = Cell::new(Wrapping(prng_seed()));
    }

    fn prng_seed() -> u64 {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);

        // Any non-zero seed will do -- this uses the hash of a global counter.
        // Refs: https://github.com/rayon-rs/rayon/pull/571
        let mut seed = 0;
        while seed == 0 {
            let mut hasher = DefaultHasher::new();
            hasher.write_usize(COUNTER.fetch_add(1, Ordering::Relaxed));
            seed = hasher.finish();
        }
        seed
    }

    RNG.with(|rng| {
        let mut x = rng.get();
        debug_assert_ne!(x.0, 0);
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        rng.set(x);
        x.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    })
}
