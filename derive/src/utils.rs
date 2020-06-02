pub(crate) use derive_utils::{derive_trait, EnumData as Data};
pub(crate) use proc_macro2::TokenStream;
pub(crate) use quote::{format_ident, quote};
pub(crate) use syn::{parse_quote, Result};

macro_rules! error {
    ($span:expr, $msg:expr) => {
        syn::Error::new_spanned(&$span, $msg)
    };
    ($span:expr, $($tt:tt)*) => {
        error!($span, format!($($tt)*))
    };
}
