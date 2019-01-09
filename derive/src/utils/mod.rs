use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
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

/// Returns standard library's root.
///
/// In default returns `::std`.
/// if disabled default crate feature, returned `::core`.
pub(crate) fn std_root() -> TokenStream {
    #[cfg(feature = "std")]
    let root = quote!(::std);
    #[cfg(not(feature = "std"))]
    let root = quote!(::core);
    root
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
