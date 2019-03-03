use std::{fmt, result};

use proc_macro2::TokenStream;
use quote::quote;

use crate::attribute::NAME;

use self::Error::*;

pub(crate) type Result<T> = result::Result<T, Error>;

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
            UnsupportedExpr(msg) => write!(f, "unsupported expression: `{}` {}", NAME, msg),
            UnsupportedStmt(msg) => write!(f, "unsupported statement: `{}` {}", NAME, msg),
            UnsupportedItem(msg) => write!(f, "unsupported item: `{}` {}", NAME, msg),
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

pub(crate) fn unsupported_expr<S: Into<String>>(s: S) -> Error {
    UnsupportedExpr(s.into())
}
pub(crate) fn unsupported_stmt<S: Into<String>>(s: S) -> Error {
    UnsupportedStmt(s.into())
}
pub(crate) fn unsupported_item<S: Into<String>>(s: S) -> Error {
    UnsupportedItem(s.into())
}
