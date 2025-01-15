// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(clippy::unnecessary_wraps)]

pub(crate) mod core;
pub(crate) mod external;
#[cfg(feature = "std")]
pub(crate) mod std;
pub(crate) mod ty_impls;

mod prelude {
    pub(super) use derive_utils::{derive_trait, EnumData as Data, EnumImpl};
    pub(super) use proc_macro2::TokenStream;
    pub(super) use quote::{format_ident, quote, ToTokens as _};
    pub(super) use syn::{parse_quote, Result};

    pub(super) use crate::enum_derive::DeriveContext as Context;
}
