use std::{fmt, result};

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub(crate) type StdResult<T, E> = result::Result<T, E>;
pub(crate) type Result<T> = StdResult<T, Error>;

pub(crate) fn compile_err(msg: &str) -> TokenStream2 {
    quote!(compile_error!(#msg);)
}

#[derive(Debug)]
pub(crate) enum Error {
    InvalidArgs(String),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Error::*;
        match self {
            InvalidArgs(s) => write!(f, "invalid attribute arguments: {}", s),
            Other(s) => write!(f, "{}", s),
        }
    }
}

impl<S: Into<String>> From<S> for Error {
    fn from(s: S) -> Self {
        Error::Other(s.into())
    }
}

macro_rules! invalid_args {
    ($expr:expr) => {
        $crate::utils::Error::InvalidArgs($expr.to_string())
    };
    ($($tt:tt)*) => {
        $crate::utils::Error::InvalidArgs(format!($($tt)*))
    };
}
