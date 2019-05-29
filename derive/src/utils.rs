use std::ops::Deref;

use proc_macro2::{Ident, Span, TokenStream};
use syn::{punctuated::Punctuated, *};

pub(crate) use derive_utils::{derive_trait_internal as derive_trait, EnumData, Trait};
pub(crate) use quote::{quote, ToTokens};
pub(crate) use syn::{parse2, ItemImpl, Result};

pub(crate) struct Data {
    pub(crate) data: EnumData,
    pub(crate) span: TokenStream,
}

impl Deref for Data {
    type Target = EnumData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub(crate) fn ident<S: AsRef<str>>(s: S) -> Ident {
    Ident::new(s.as_ref(), Span::call_site())
}

pub(crate) fn param_ident(s: &str) -> GenericParam {
    GenericParam::Type(TypeParam {
        attrs: Vec::new(),
        ident: ident(s),
        colon_token: None,
        bounds: Punctuated::new(),
        eq_token: None,
        default: None,
    })
}

// =============================================================================
// Macros

macro_rules! parse_quote {
    ($($tt:tt)*) => {
        $crate::utils::parse2($crate::utils::quote!($($tt)*))
    };
}

macro_rules! span {
    ($expr:expr) => {
        $expr.clone()
    };
}

macro_rules! err {
    ($msg:expr) => {
        syn::Error::new_spanned(span!($msg), $msg)
    };
    ($span:expr, $msg:expr) => {
        syn::Error::new_spanned(span!($span), $msg)
    };
    ($span:expr, $($tt:tt)*) => {
        err!($span, format!($($tt)*))
    };
}
