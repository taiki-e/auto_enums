use std::{fmt, result};

use proc_macro2::TokenStream;
use quote::quote;

use crate::auto_enum::NAME;

use self::Error::{InvalidArgs, InvalidExpr, Other};

pub(crate) type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub(crate) enum Error {
    InvalidArgs(String),
    // FIXME: This may not be necessary.
    /// An expression that is invalid also as expression of Rust.
    InvalidExpr(String),
    Other(String),
}

impl Error {
    #[inline(never)]
    pub(crate) fn to_compile_err(&self) -> TokenStream {
        let msg = &format!("{}", self);
        quote!(compile_error!(#msg);)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidArgs(msg) => write!(f, "invalid attribute arguments: `{}` {}", NAME, msg),
            InvalidExpr(msg) => write!(f, "invalid expression: `{}` {}", NAME, msg),
            Other(msg) => write!(f, "`{}` {}", NAME, msg),
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

pub(crate) fn invalid_expr<S: Into<String>>(s: S) -> Error {
    InvalidExpr(s.into())
}
