// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{cell::RefCell, collections::hash_map::DefaultHasher, hash::Hasher, iter, mem, thread};

use proc_macro2::TokenStream;
use quote::format_ident;
#[cfg(feature = "type_analysis")]
use syn::Type;
use syn::{
    parse::{Parse, ParseStream},
    parse_quote, Attribute, Error, Expr, Ident, ItemEnum, Macro, Path, Result, Token,
};

use super::visitor::{Dummy, Visitor};
use crate::utils::{expr_call, path, replace_expr, unit, Node};

// =================================================================================================
// Context

/// Config for related to `visitor::Visitor` type.
#[derive(Clone, Copy, PartialEq)]
pub(super) enum VisitMode {
    Default,
    Return(/* count */ usize),
    Try,
}

/// Config for related to `expr::child_expr`.
#[derive(Clone, Copy, PartialEq)]
pub(super) enum VisitLastMode {
    Default,
    /*
    local: `let .. = || {};`
    or
    expr: `|| {}`
    not
    item_fn: `fn _() -> Fn*() { || {} }`
    */
    /// `Stmt::Semi(..)` - never visit last expr
    Never,
}

/// The default identifier of expression level marker.
pub(super) const DEFAULT_MARKER: &str = "marker";

pub(super) struct Context {
    builder: Builder,

    /// The identifier of the marker macro of the current scope.
    pub(super) current_marker: String,
    /// All marker macro identifiers that may have effects on the current scope.
    markers: Vec<String>,

    // TODO: we may be able to replace some fields based on depth.
    // depth: isize,
    /// Currently, this is basically the same as `self.markers.len() == 1`.
    root: bool,
    /// This is `true` if other `auto_enum` attribute exists in the current scope.
    pub(super) has_child: bool,

    pub(super) visit_mode: VisitMode,
    pub(super) visit_last_mode: VisitLastMode,

    /// Span passed to `syn::Error::new_spanned`.
    pub(super) span: TokenStream,
    // - `None`: during checking.
    // - `Some(None)`: there are no errors.
    // - `Some(Some)`: there are errors.
    #[allow(clippy::option_option)]
    error: RefCell<Option<Option<Error>>>,

    pub(super) args: Vec<Path>,
    // if "type_analysis" feature is disabled, this field is always empty.
    traits: Vec<Path>,
}

impl Context {
    fn new(
        span: TokenStream,
        args: TokenStream,
        root: bool,
        mut markers: Vec<String>,
        diagnostic: Option<Error>,
    ) -> Result<Self> {
        let Args { args, marker } = syn::parse2(args)?;

        let current_marker = if let Some(marker) = marker {
            // Currently, there is no reason to preserve the span, so convert `Ident` to `String`.
            // This should probably be more efficient than calling `to_string` for each comparison.
            // https://github.com/alexcrichton/proc-macro2/blob/1.0.1/src/wrapper.rs#L706
            let marker_string = marker.to_string();
            if markers.contains(&marker_string) {
                bail!(
                    marker,
                    "a custom marker name is specified that duplicated the name already used in the parent scope",
                );
            }
            marker_string
        } else {
            DEFAULT_MARKER.to_string()
        };

        markers.push(current_marker.clone());

        Ok(Self {
            builder: Builder::new(&span),
            current_marker,
            markers,
            root,
            has_child: false,
            visit_mode: VisitMode::Default,
            visit_last_mode: VisitLastMode::Default,
            span,
            error: RefCell::new(Some(diagnostic)),
            args,
            traits: vec![],
        })
    }

    /// Make a new `Context` as a root.
    pub(super) fn root(span: TokenStream, args: TokenStream) -> Result<Self> {
        Self::new(span, args, true, Vec::with_capacity(1), None)
    }

    /// Make a new `Context` as a child based on a parent context `self`.
    pub(super) fn make_child(&mut self, span: TokenStream, args: TokenStream) -> Result<Self> {
        debug_assert!(self.has_child);
        Self::new(
            span,
            args,
            false,
            mem::take(&mut self.markers),
            self.error.borrow_mut().as_mut().unwrap().take(),
        )
    }

    /// Merge a child `Context` into a parent context `self`.
    pub(super) fn join_child(&mut self, mut child: Self) {
        debug_assert!(self.markers.is_empty());
        child.markers.pop();
        mem::swap(&mut self.markers, &mut child.markers);

        if let Some(message) = child.error.borrow_mut().take().unwrap() {
            self.error(message);
        }
    }

    pub(super) fn error(&self, message: Error) {
        match self.error.borrow_mut().as_mut().unwrap() {
            Some(base) => base.combine(message),
            error @ None => *error = Some(message),
        }
    }

    pub(super) fn check(self) -> Result<()> {
        match self.error.borrow_mut().take().unwrap() {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }

    /// Returns `true` if one or more errors occurred.
    pub(super) fn has_error(&self) -> bool {
        self.error.borrow().as_ref().unwrap().is_some()
    }

    pub(super) fn visit_last(&self) -> bool {
        self.visit_last_mode != VisitLastMode::Never && self.visit_mode != VisitMode::Try
    }

    /// Even if this is `false`, there are cases where this `auto_enum` attribute is handled as a
    /// dummy. e.g., If `self.has_child && self.builder.variants.is_empty()` is true, this
    /// `auto_enum` attribute is handled as a dummy.
    pub(super) fn is_dummy(&self) -> bool {
        // `auto_enum` attribute with no argument is handled as a dummy.
        // if "type_analysis" feature is disabled, `self.traits` field is always empty.
        self.args.is_empty() && self.traits.is_empty()
    }

    #[cfg(feature = "type_analysis")]
    pub(super) fn variant_is_empty(&self) -> bool {
        self.builder.variants.is_empty()
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

        self.markers.iter().any(|marker| mac.path.is_ident(marker))
    }

    /// Returns `true` if `mac` is the marker macro of the current scope.
    pub(super) fn is_marker_macro_exact(&self, mac: &Macro) -> bool {
        mac.path.is_ident(&self.current_marker)
    }

    /// from `<expr>` into `Enum::VariantN(<expr>)`
    pub(super) fn next_expr(&mut self, expr: Expr) -> Expr {
        self.next_expr_with_attrs(vec![], expr)
    }

    /// from `<expr>` into `<attrs> Enum::VariantN(<expr>)`
    pub(super) fn next_expr_with_attrs(&mut self, attrs: Vec<Attribute>, expr: Expr) -> Expr {
        self.builder.next_expr(attrs, expr)
    }

    pub(super) fn replace_boxed_expr(&mut self, expr: &mut Option<Box<Expr>>) {
        replace_expr(expr.get_or_insert_with(|| Box::new(unit())), |expr| {
            if self.is_marker_expr(&expr) {
                // Skip if `<expr>` is a marker macro.
                expr
            } else {
                self.next_expr(expr)
            }
        });
    }

    // visitors

    pub(super) fn visitor(&mut self, node: &mut impl Node) {
        node.visited(&mut Visitor::new(self));
    }

    pub(super) fn dummy(&mut self, node: &mut impl Node) {
        debug_assert!(self.args.is_empty());

        node.visited(&mut Dummy::new(self));
    }

    // build

    pub(super) fn build(&mut self, f: impl FnOnce(ItemEnum)) {
        // As we know that an error will occur, it does not matter if there are not enough variants.
        if !self.has_error() {
            match self.builder.variants.len() {
                1 => {}
                0 if !self.has_child => {}
                _ => {
                    if !self.builder.variants.is_empty() {
                        f(self.builder.build(&self.args, &self.traits));
                    }
                    return;
                }
            }

            let (msg1, msg2) = match self.visit_last_mode {
                VisitLastMode::Default => {
                    ("branches or marker macros in total", "branch or marker macro")
                }
                VisitLastMode::Never => ("marker macros", "marker macro"),
            };
            self.error(format_err!(
                self.span,
                "`#[auto_enum]` is required two or more {}, there is {} {} in this statement",
                msg1,
                if self.builder.variants.is_empty() { "no" } else { "only one" },
                msg2
            ));
        }
    }

    // type_analysis feature

    #[cfg(feature = "type_analysis")]
    pub(super) fn collect_impl_trait(&mut self, ty: &mut Type) -> bool {
        super::type_analysis::collect_impl_trait(&self.args, &mut self.traits, ty)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if !thread::panicking() && self.error.borrow().is_some() {
            panic!("context need to be checked");
        }
    }
}

// =================================================================================================
// Args

mod kw {
    syn::custom_keyword!(marker);
}

struct Args {
    args: Vec<Path>,
    marker: Option<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut args = Vec::with_capacity(usize::from(!input.is_empty()));
        let mut marker = None;
        while !input.is_empty() {
            if input.peek(kw::marker) && input.peek2(Token![=]) {
                let i: kw::marker = input.parse()?;
                let _: Token![=] = input.parse()?;
                let ident: Ident = input.parse()?;
                if marker.replace(ident).is_some() {
                    bail!(i, "duplicate `marker` argument");
                }
            } else {
                args.push(input.parse()?);
            }

            if input.is_empty() {
                break;
            }
            let _: Token![,] = input.parse()?;
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
    fn new(input: &TokenStream) -> Self {
        Self { ident: format_ident!("__Enum{}", hash(input)), variants: Vec::with_capacity(2) }
    }

    fn next_expr(&mut self, attrs: Vec<Attribute>, expr: Expr) -> Expr {
        let variant = format_ident!("__Variant{}", self.variants.len());

        let path =
            path(iter::once(self.ident.clone().into()).chain(iter::once(variant.clone().into())));

        self.variants.push(variant);

        expr_call(attrs, path, expr)
    }

    fn build(&self, args: &[Path], traits: &[Path]) -> ItemEnum {
        let derive = args.iter().chain(traits);
        let ident = &self.ident;
        let ty_generics = &self.variants;
        let variants = &self.variants;
        let fields = &self.variants;

        parse_quote! {
            #[allow(non_camel_case_types)]
            #[::auto_enums::enum_derive(#(#derive),*)]
            enum #ident<#(#ty_generics),*> {
                #(#variants(#fields),)*
            }
        }
    }
}

/// Returns the hash value of the input AST.
fn hash(input: &TokenStream) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write(input.to_string().as_bytes());
    hasher.finish()
}
