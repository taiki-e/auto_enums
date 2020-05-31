use proc_macro2::TokenStream;
use quote::format_ident;
use std::{collections::hash_map::DefaultHasher, hash::Hasher, iter, mem};
#[cfg(feature = "type_analysis")]
use syn::Type;
use syn::{
    parse::{Parse, ParseStream},
    token, Attribute, Error, Expr, Ident, ItemEnum, Macro, Path, Result,
};

use super::visitor::{Dummy, Visitor};
use crate::utils::{expr_call, path, replace_expr, unit, VisitedNode};

// =================================================================================================
// Context

/// Config for related to `visotor::Visotor` type.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum VisitMode {
    Default,
    Return(/* count */ usize),
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
    /// `Stmt::Semi(..)` - never visit last expr
    Never,
}

#[derive(Clone, Default)]
pub(super) struct Diagnostic {
    messages: Vec<Error>,
}

impl Diagnostic {
    pub(super) fn error(&mut self, message: Error) {
        self.messages.push(message);
    }

    pub(super) fn to_compile_error(&self) -> Option<TokenStream> {
        if self.messages.is_empty() {
            None
        } else {
            Some(self.messages.iter().map(Error::to_compile_error).collect())
        }
    }
}

/// The default identifier of expression level marker.
pub(super) const DEFAULT_MARKER: &str = "marker";

pub(super) struct Context {
    builder: Builder,

    /// The identifier of the marker macro of the current scope.
    pub(super) marker: String,
    /// All marker macro identifiers that may have effects on the current scope.
    pub(super) markers: Vec<String>,

    // TODO: we may be able to replace some fields based on depth.
    // depth: isize,
    /// Currently, this is basically the same as `self.markers.len() == 1`.
    root: bool,
    /// This is `true` if other `auto_enum` attribute exists in the current scope.
    pub(super) other_attr: bool,

    pub(super) visit_mode: VisitMode,
    pub(super) visit_last_mode: VisitLastMode,

    /// Span passed to `syn::Error::new_spanned`.
    pub(super) span: TokenStream,
    pub(super) diagnostic: Diagnostic,

    pub(super) args: Vec<Path>,
    #[cfg(feature = "type_analysis")]
    traits: Vec<Path>,
}

impl Context {
    fn new(
        span: TokenStream,
        args: TokenStream,
        root: bool,
        mut markers: Vec<String>,
        diagnostic: Diagnostic,
    ) -> Result<Self> {
        let Args { args, marker } = syn::parse2(args)?;

        let marker = if let Some(marker) = marker {
            // Currently, there is no reason to preserve the span, so convert `Ident` to `String`.
            // This should probably be more efficient than calling `to_string` for each comparison.
            // https://github.com/alexcrichton/proc-macro2/blob/1.0.1/src/wrapper.rs#L706
            let marker_string = marker.to_string();
            if markers.contains(&marker_string) {
                return Err(error!(
                    marker,
                    "A custom marker name is specified that duplicated the name already used in the parent scope",
                ));
            }
            marker_string
        } else {
            DEFAULT_MARKER.to_string()
        };

        markers.push(marker.clone());

        Ok(Self {
            builder: Builder::new(&span),
            marker,
            markers,
            root,
            other_attr: false,
            visit_mode: VisitMode::Default,
            visit_last_mode: VisitLastMode::Default,
            span,
            diagnostic,
            args,
            #[cfg(feature = "type_analysis")]
            traits: Vec::new(),
        })
    }

    /// Make a new `Context` as a root.
    pub(super) fn root(span: TokenStream, args: TokenStream) -> Result<Self> {
        Self::new(span, args, true, Vec::new(), Diagnostic::default())
    }

    /// Make a new `Context` as a child based on a parent context `self`.
    pub(super) fn make_child(&mut self, span: TokenStream, args: TokenStream) -> Result<Self> {
        Self::new(
            span,
            args,
            false,
            mem::replace(&mut self.markers, Vec::new()),
            mem::replace(&mut self.diagnostic, Diagnostic::default()),
        )
    }

    /// Merge a child `Context` into a parent context `self`.
    pub(super) fn join_child(&mut self, mut child: Self) {
        debug_assert!(self.diagnostic.messages.is_empty());
        debug_assert!(self.markers.is_empty());

        child.markers.pop();
        self.markers = child.markers;
        self.diagnostic = child.diagnostic;
    }

    #[cfg(auto_enums_def_site_enum_ident)]
    pub(super) fn update_enum_ident(&mut self, ident: &Ident) {
        self.builder.update_enum_ident(ident)
    }

    /// Returns `true` if one or more errors occurred.
    pub(super) fn failed(&self) -> bool {
        !self.diagnostic.messages.is_empty()
    }

    pub(super) fn visit_last(&self) -> bool {
        self.visit_last_mode != VisitLastMode::Never && self.visit_mode != VisitMode::Try
    }

    /// Even if this is `false`, there are cases where this `auto_enum` attribute is handled as a
    /// dummy. e.g., If `self.other_attr && self.builder.variants.is_empty()` is true, this
    /// `auto_enum` attribute is handled as a dummy.
    pub(super) fn is_dummy(&self) -> bool {
        // `auto_enum` attribute with no argument is handled as a dummy.
        #[cfg(not(feature = "type_analysis"))]
        {
            self.args.is_empty()
        }
        #[cfg(feature = "type_analysis")]
        {
            self.args.is_empty() && self.traits.is_empty()
        }
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
        mac.path.is_ident(&self.marker)
    }

    /// from `<expr>` into `Enum::VariantN(<expr>)`
    pub(super) fn next_expr(&mut self, expr: Expr) -> Expr {
        self.next_expr_with_attrs(Vec::new(), expr)
    }

    /// from `<expr>` into `<attrs> Enum::VariantN(<expr>)`
    pub(super) fn next_expr_with_attrs(&mut self, attrs: Vec<Attribute>, expr: Expr) -> Expr {
        self.builder.next_expr(attrs, expr)
    }

    pub(super) fn replace_boxed_expr(&mut self, expr: &mut Option<Box<Expr>>) {
        replace_expr(&mut **expr.get_or_insert_with(|| Box::new(unit())), |expr| {
            if self.is_marker_expr(&expr) {
                // Skip if `<expr>` is a marker macro.
                expr
            } else {
                self.next_expr(expr)
            }
        });
    }

    // visitors

    pub(super) fn visitor(&mut self, node: &mut impl VisitedNode) {
        node.visited(&mut Visitor::new(self));
    }

    pub(super) fn dummy(&mut self, node: &mut impl VisitedNode) {
        #[cfg(not(feature = "type_analysis"))]
        debug_assert!(self.is_dummy());
        #[cfg(feature = "type_analysis")]
        debug_assert!(self.args.is_empty());

        node.visited(&mut Dummy::new(self));
    }

    // build

    pub(super) fn build(&mut self, f: impl FnOnce(ItemEnum)) -> Result<()> {
        fn err(cx: &Context) -> Error {
            let (msg1, msg2) = match cx.visit_last_mode {
                VisitLastMode::Default => {
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
            #[cfg(not(feature = "type_analysis"))]
            {
                f(self.builder.build(&self.args, &[]));
            }
            #[cfg(feature = "type_analysis")]
            {
                f(self.builder.build(&self.args, &self.traits));
            }
        }
        Ok(())
    }

    // type_analysis feature

    #[cfg(feature = "type_analysis")]
    pub(super) fn collect_trait(&mut self, ty: &mut Type) {
        super::type_analysis::collect_impl_trait(&self.args, &mut self.traits, ty);
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
            if input.peek(kw::marker) && input.peek2(token::Eq) {
                let i: kw::marker = input.parse()?;
                let _: token::Eq = input.parse()?;
                let ident: Ident = input.parse()?;
                if marker.replace(ident).is_some() {
                    return Err(error!(i, "duplicate `marker` argument"));
                }
            } else {
                args.push(input.parse()?);
            }

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
    fn new(input: &TokenStream) -> Self {
        Self { ident: format_ident!("__Enum{}", hash(input)), variants: Vec::new() }
    }

    #[cfg(auto_enums_def_site_enum_ident)]
    fn update_enum_ident(&mut self, ident: &Ident) {
        debug_assert!(self.variants.is_empty());
        self.ident = format_ident!("__Enum{}", ident, span = proc_macro::Span::def_site().into());
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

        syn::parse_quote! {
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
