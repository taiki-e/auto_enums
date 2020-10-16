//! An internal crate to support auto_enums - **do not use directly**

#![doc(html_root_url = "https://docs.rs/auto_enums_core/0.7.8")]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms, single_use_lifetimes), allow(dead_code))
))]
#![forbid(unsafe_code)]
#![warn(future_incompatible, rust_2018_idioms, unreachable_pub)]
// It cannot be included in the published code because these lints have false positives in the minimum required version.
#![cfg_attr(test, warn(single_use_lifetimes))]
#![warn(clippy::all, clippy::default_trait_access)]
// mem::take and #[non_exhaustive] requires Rust 1.40, matches! requires Rust 1.42
#![allow(
    clippy::mem_replace_with_default,
    clippy::manual_non_exhaustive,
    clippy::match_like_matches_macro
)]

// older compilers require explicit `extern crate`.
#[allow(unused_extern_crates)]
extern crate proc_macro;

#[macro_use]
mod utils;

mod auto_enum;

use proc_macro::TokenStream;

/// An attribute macro for to allow multiple return types by automatically generated enum.
/// See crate level documentation for details.
#[proc_macro_attribute]
pub fn auto_enum(args: TokenStream, input: TokenStream) -> TokenStream {
    crate::auto_enum::attribute(args.into(), input.into()).into()
}
