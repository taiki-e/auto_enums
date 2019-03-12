use std::{fmt, result};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub(crate) type Result<T> = result::Result<T, Error>;

pub(crate) enum Error {
    /// `syn::Error`.
    Syn(syn::Error),
    /// Other error.
    Other(String),
}

impl Error {
    pub(crate) fn set_span<T: ToTokens>(self, tokens: T) -> Self {
        match self {
            Error::Syn(_) => self,
            Error::Other(msg) => Error::Syn(syn::Error::new_spanned(tokens, msg)),
        }
    }

    /// Render the error as an invocation of [`compile_error!`].
    ///
    /// [`compile_error!`]: https://doc.rust-lang.org/std/macro.compile_error.html
    pub(crate) fn to_compile_error(&self) -> TokenStream {
        if let Error::Syn(err) = self {
            err.to_compile_error()
        } else {
            // TODO: use `unreachable!()` in this branch
            let msg = &format!("{}", self);
            quote!(compile_error!(#msg);)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Syn(err) => err.fmt(f),
            Error::Other(msg) => msg.fmt(f),
        }
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Other(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Other(s.into())
    }
}

impl From<syn::Error> for Error {
    fn from(e: syn::Error) -> Self {
        Error::Syn(e)
    }
}

macro_rules! span {
    ($expr:expr) => {
        $expr.clone()
    };
}

macro_rules! err {
    ($msg:expr) => {{
        $crate::utils::Error::from(
            syn::Error::new_spanned(span!($msg), $msg)
        )
    }};
    ($span:expr, $msg:expr) => {
        $crate::utils::Error::from(
            syn::Error::new_spanned(span!($span), $msg)
        )
    };
    ($span:expr, $($tt:tt)*) => {
        err!($span, format!($($tt)*))
    };
}

macro_rules! arg_err {
    ($msg:expr) => {{
        #[allow(unused_imports)]
        use syn::spanned::Spanned as _Spanned;
        $crate::utils::Error::from(
            syn::Error::new(
                $msg.span(),
                format!("invalid arguments: {}", $msg),
            )
        )
    }};
    ($span:expr, $msg:expr) => {
        $crate::utils::Error::from(
            syn::Error::new(
                syn::spanned::Spanned::span(&$span),
                format!("invalid arguments: {}", $msg),
            )
        )
    };
    ($span:expr, $($tt:tt)*) => {
        arg_err!($span, format!($($tt)*))
    };
}
