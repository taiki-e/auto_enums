use proc_macro2::TokenStream;
use std::ops::Deref;

#[cfg(feature = "fn_traits")]
pub(crate) use derive_utils::Trait;
pub(crate) use derive_utils::{derive_trait_internal as derive_trait, EnumData};
pub(crate) use quote::{format_ident, quote, ToTokens};
#[cfg(any(feature = "fn_traits", feature = "transpose_methods"))]
pub(crate) use syn::TypeParam;
pub(crate) use syn::{parse2, ItemImpl, Result};

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
