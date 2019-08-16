use std::ops::Deref;

use proc_macro2::{Ident, TokenStream};
use syn::{punctuated::Punctuated, *};

pub(crate) use derive_utils::{derive_trait_internal as derive_trait, EnumData, Trait};
pub(crate) use quote::{format_ident, quote, ToTokens};
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

pub(crate) fn param_ident(ident: Ident) -> GenericParam {
    GenericParam::Type(TypeParam {
        attrs: Vec::new(),
        ident,
        colon_token: None,
        bounds: Punctuated::new(),
        eq_token: None,
        default: None,
    })
}

// =============================================================================
// Macros

macro_rules! param_ident {
    ($($tt:tt)*) => {
        $crate::utils::param_ident($crate::utils::format_ident!($($tt)*))
    };
}

macro_rules! parse_quote {
    ($($tt:tt)*) => {
        $crate::utils::parse2($crate::utils::quote!($($tt)*))
    };
}

macro_rules! error {
    ($span:expr, $msg:expr) => {
        syn::Error::new_spanned(&$span, $msg)
    };
    ($span:expr, $($tt:tt)*) => {
        error!($span, format!($($tt)*))
    };
}
