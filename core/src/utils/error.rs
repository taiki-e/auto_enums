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

    /// An expression that is invalid also as expression of Rust.
    InvalidExpr(String),

    UnsupportedExpr(String),
    UnsupportedStmt(String),
    UnsupportedItem(String),

    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Error::*;
        match self {
            InvalidArgs(s) => write!(f, "invalid attribute arguments: {}", s),
            InvalidExpr(s) => write!(f, "invalid expression: {}", s),
            UnsupportedExpr(s) => write!(f, "unsupported expression: {}", s),
            UnsupportedStmt(s) => write!(f, "unsupported statement: {}", s),
            UnsupportedItem(s) => write!(f, "unsupported item: {}", s),
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

pub(crate) fn invalid_expr<S: Into<String>>(s: S) -> Error {
    Error::InvalidExpr(s.into())
}

pub(crate) fn unsupported_expr<S: Into<String>>(s: S) -> Error {
    Error::UnsupportedExpr(s.into())
}
pub(crate) fn unsupported_stmt<S: Into<String>>(s: S) -> Error {
    Error::UnsupportedStmt(s.into())
}
pub(crate) fn unsupported_item<S: Into<String>>(s: S) -> Error {
    Error::UnsupportedItem(s.into())
}
