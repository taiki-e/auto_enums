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
use quote::format_ident;
use syn::{
    parse::{Parse, ParseStream},
    *,
};

use crate::utils::{expr_call, path, replace_expr, unit};

use super::visitor::{Dummy, FindTry, Visitor};

// =================================================================================================
// Context

/// Config for related to `visotor::Visotor` type.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum VisitMode {
    Default,
    Return,
    Try,
}

/// Config for related to `expr::child_expr`.
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
    /// Currently, this is handled as the same as `Default`.
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

    pub(super) fn get_inner(&self) -> Option<Error> {
        self.message.borrow().clone()
    }
}

/// The default identifier of expression level marker.
pub(super) const DEFAULT_MARKER: &str = "marker";

pub(super) struct Context {
    args: Vec<Path>,
    builder: Builder,

    /// The identifier of the marker macro of the current scope.
    pub(super) marker: Ident,
    /// All marker macro identifiers that may have effects on the current scope.
    pub(super) markers: Rc<RefCell<Vec<String>>>,

    // depth: i32,
    /// Currently, this is basically the same as `self.markers.borrow().len() == 1`.
    root: bool,
    /// This is `true` if other `auto_enum` attribute exists in the current scope.
    pub(super) other_attr: bool,

    pub(super) visit_mode: VisitMode,
    pub(super) visit_last_mode: VisitLastMode,

    /// Span passed to `syn::Error::new_spanned`.
    pub(super) span: TokenStream,
    pub(super) diagnostic: Diagnostic,
}

impl Drop for Context {
    fn drop(&mut self) {
        self.markers.borrow_mut().pop();
    }
}

impl Context {
    fn new(
        span: TokenStream,
        args: TokenStream,
        root: bool,
        markers: Rc<RefCell<Vec<String>>>,
        diagnostic: Diagnostic,
    ) -> Result<Self> {
        let Args { args, marker } = syn::parse2(args)?;

        let (marker_string, marker) = if let Some(marker) = marker {
            // Currently, there is no reason to preserve the span, so convert `Ident` to `String`.
            // This should probably be more efficient than calling `to_string` for each comparison.
            // https://github.com/alexcrichton/proc-macro2/blob/1.0.1/src/wrapper.rs#L706
            let marker_string = marker.to_string();
            if markers.borrow().contains(&marker_string) {
                return Err(error!(
                    marker,
                    "A custom marker name is specified that duplicated the name already used in the parent scope",
                ));
            }
            (marker_string, marker)
        } else {
            (DEFAULT_MARKER.to_owned(), format_ident!("{}", DEFAULT_MARKER))
        };

        markers.borrow_mut().push(marker_string);

        Ok(Self {
            builder: Builder::new(),
            args,
            marker,
            markers,
            root,
            other_attr: false,
            visit_mode: VisitMode::Default,
            visit_last_mode: VisitLastMode::Default,
            span,
            diagnostic,
        })
    }

    pub(super) fn root(span: TokenStream, args: TokenStream) -> Result<Self> {
        Self::new(span, args, true, Rc::default(), Diagnostic::default())
    }

    pub(super) fn make_child(&self, span: TokenStream, args: TokenStream) -> Result<Self> {
        Self::new(span, args, false, self.markers.clone(), self.diagnostic.clone())
    }

    /// Returns `true` if one or more errors occurred.
    pub(super) fn failed(&self) -> bool {
        self.diagnostic.message.borrow().is_some()
    }

    pub(super) fn visit_last(&self) -> bool {
        self.visit_last_mode != VisitLastMode::Never && self.visit_mode != VisitMode::Try
    }

    /// Even if this is `false`, there are cases where this `auto_enum` attribute is handled as a
    /// dummy. e.g., If `self.other_attr && self.builder.variants.is_empty()` is true, this
    /// `auto_enum` attribute is handled as a dummy.
    pub(super) fn is_dummy(&self) -> bool {
        // `auto_enum` attribute with no argument is handled as a dummy.
        self.args.is_empty()
    }

    /// Returns `true` if `expr` is the marker macro that may have effects on the current scope.
    pub(super) fn is_marker_expr(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Macro(expr) => self.is_marker_macro(&expr.mac),
            _ => false,
        }
    }

    /// Returns `true` if `mac` is the marker macro that may have effects on the current scope.
    pub(super) fn is_marker_macro(&self, mac: &Macro) -> bool {
        let exact = self.is_marker_macro_exact(mac);
        if exact || self.root {
            return exact;
        }

        self.markers.borrow().iter().any(|marker| mac.path.is_ident(marker))
    }

    /// Returns `true` if `mac` is the marker macro of the current scope.
    pub(super) fn is_marker_macro_exact(&self, mac: &Macro) -> bool {
        mac.path.is_ident(&self.marker)
    }

    pub(super) fn next_expr(&mut self, expr: Expr) -> Expr {
        self.next_expr_with_attrs(Vec::new(), expr)
    }

    pub(super) fn next_expr_with_attrs(&mut self, attrs: Vec<Attribute>, expr: Expr) -> Expr {
        self.builder.next_expr(attrs, expr)
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

    // visitors

    pub(super) fn visitor(&mut self, f: impl FnOnce(&mut Visitor<'_>)) {
        f(&mut Visitor::new(self));
    }

    pub(super) fn dummy(&mut self, f: impl FnOnce(&mut Dummy<'_>)) {
        debug_assert!(self.is_dummy());

        f(&mut Dummy::new(self));
    }

    pub(super) fn find_try(&mut self, f: impl FnOnce(&mut FindTry<'_>)) {
        let mut find = FindTry::new(self);
        f(&mut find);
        if find.has {
            self.visit_mode = VisitMode::Try;
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
        super::type_analysis::collect_impl_trait(&mut self.args, ty);
    }
}

// =================================================================================================
// Args

mod kw {
    syn::custom_keyword!(marker);
}

#[allow(dead_code)] // https://github.com/rust-lang/rust/issues/56750
struct Args {
    args: Vec<Path>,
    marker: Option<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut args = Vec::new();
        let mut marker = None;

        while !input.is_empty() {
            if input.peek(kw::marker) && input.peek2(Token![=]) {
                let _: kw::marker = input.parse()?;
                let _: Token![=] = input.parse()?;
                let i: Ident = input.parse()?;
                if marker.is_some() {
                    return Err(error!(i, "duplicate `marker` argument"));
                } else {
                    marker = Some(i);
                    if !input.is_empty() {
                        let _: token::Comma = input.parse()?;
                    }
                    continue;
                }
            }

            args.push(input.parse()?);

            if !input.is_empty() {
                let _: token::Comma = input.parse()?;
            }
        }

        Ok(Self { args, marker })
    }
}

// =================================================================================================
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

// =================================================================================================
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
