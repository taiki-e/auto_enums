use proc_macro2::{Ident, Span};
use smallvec::SmallVec;
use syn::{punctuated::Punctuated, *};

pub(crate) use derive_utils::{
    compile_err, derive_trait_internal as derive_trait, EnumData, Error, Result, Trait,
};
pub(crate) use quote::{quote, ToTokens};
pub(crate) use syn::{parse2, ItemImpl};

pub(crate) type Data = EnumData;
pub(crate) type Stack<T> = SmallVec<[T; 4]>;

pub(crate) fn ident(s: &str) -> Ident {
    Ident::new(s, Span::call_site())
}

pub(crate) fn param_ident(s: &str) -> GenericParam {
    GenericParam::Type(TypeParam {
        attrs: Vec::with_capacity(0),
        ident: ident(s),
        colon_token: None,
        bounds: Punctuated::new(),
        eq_token: None,
        default: None,
    })
}

macro_rules! parse_quote {
    ($($tt:tt)*) => {
        $crate::utils::parse2($crate::utils::quote!($($tt)*))
    };
}

macro_rules! invalid_args {
    ($expr:expr) => {
        $crate::utils::Error::Other(format!("invalid attribute arguments: {}", $expr))
    };
    ($($tt:tt)*) => {
        invalid_args!(format!($($tt)*))
    };
}
