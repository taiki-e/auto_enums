// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(clippy::unnecessary_wraps)]

pub(crate) mod core;
pub(crate) mod external;
#[cfg(feature = "std")]
pub(crate) mod std;
pub(crate) mod ty_impls;

mod prelude {
    pub(super) use derive_utils::{EnumData as Data, EnumImpl, derive_trait};
    pub(super) use proc_macro2::TokenStream;
    pub(super) use quote::{ToTokens as _, format_ident, quote};
    pub(super) use syn::{Result, parse_quote};

    pub(super) use crate::enum_derive::DeriveContext as Context;
}
