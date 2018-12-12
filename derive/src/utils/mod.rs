use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use smallvec::SmallVec;
use syn::{punctuated::Punctuated, *};

#[macro_use]
mod error;

mod parse;

pub(crate) use self::error::{Error, Result, *};
pub(crate) use self::parse::{build, Data, Trait};

pub(crate) type Stack<T> = SmallVec<[T; 8]>;

pub(crate) fn default<T: Default>() -> T {
    T::default()
}

pub(crate) fn ident_call_site(s: &str) -> Ident {
    Ident::new(s, Span::call_site())
}

pub(crate) fn block(stmts: Vec<Stmt>) -> Block {
    Block {
        brace_token: default(),
        stmts,
    }
}

pub(crate) fn param_ident(ident: &str) -> GenericParam {
    GenericParam::Type(TypeParam {
        attrs: Vec::with_capacity(0),
        ident: ident_call_site(ident),
        colon_token: None,
        bounds: Punctuated::new(),
        eq_token: None,
        default: None,
    })
}

pub(crate) fn std_root() -> TokenStream {
    #[cfg(feature = "std")]
    let root = quote!(::std);
    #[cfg(not(feature = "std"))]
    let root = quote!(::core);
    root
}
