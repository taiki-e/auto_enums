#![allow(clippy::unnecessary_wraps)]

pub(crate) mod core;
pub(crate) mod external;
#[cfg(feature = "std")]
pub(crate) mod std;
pub(crate) mod ty_impls;

use derive_utils::{derive_trait, EnumData as Data};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, Result};
