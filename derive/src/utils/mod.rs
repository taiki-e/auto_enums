use proc_macro2::{Ident, Span};
use smallvec::SmallVec;
use syn::{punctuated::Punctuated, *};

pub(crate) use derive_utils::{Error, Result, *};

pub(crate) type Data = EnumData;
pub(crate) type Stack<T> = SmallVec<[T; 4]>;

pub(crate) fn ident_call_site(s: &str) -> Ident {
    Ident::new(s, Span::call_site())
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

macro_rules! parse_quote {
    ($($tt:tt)*) => {
        $crate::syn::parse2($crate::quote::quote!($($tt)*))
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
