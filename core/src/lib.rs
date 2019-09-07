#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/auto_enums_core/0.6.0")]
#![doc(test(attr(deny(warnings), allow(dead_code, unused_assignments, unused_variables))))]
#![warn(unsafe_code)]
#![warn(rust_2018_idioms, unreachable_pub)]
// It cannot be included in the published code because these lints have false positives in the minimum required version.
#![cfg_attr(test, warn(single_use_lifetimes))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::use_self)]

#[cfg(all(feature = "try_trait", not(feature = "unstable")))]
compile_error!(
    "The `try_trait` feature requires the `unstable` feature as an explicit opt-in to unstable features"
);

extern crate proc_macro;

#[macro_use]
mod utils;

mod auto_enum;

use proc_macro::TokenStream;

/// An attribute macro for to allow multiple return types by automatically generated enum.
#[proc_macro_attribute]
pub fn auto_enum(args: TokenStream, input: TokenStream) -> TokenStream {
    TokenStream::from(self::auto_enum::attribute(args.into(), input.into()))
}
