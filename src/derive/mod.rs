// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(clippy::unnecessary_wraps, clippy::wildcard_imports)]

pub(crate) mod core;
pub(crate) mod external;
#[cfg(feature = "std")]
pub(crate) mod std;
pub(crate) mod ty_impls;

use derive_utils::{derive_trait, EnumData as Data, EnumImpl};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, Result};

use crate::enum_derive::DeriveContext as Context;
