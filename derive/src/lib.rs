#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/auto_enums_derive/0.6.2")]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms, single_use_lifetimes), allow(dead_code))
))]
#![warn(unsafe_code)]
#![warn(rust_2018_idioms, unreachable_pub)]
#![warn(single_use_lifetimes)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::use_self)]

#[cfg(all(feature = "try_trait", not(feature = "unstable")))]
compile_error!(
    "The `try_trait` feature requires the `unstable` feature as an explicit opt-in to unstable features"
);

#[cfg(all(feature = "futures", not(feature = "unstable")))]
compile_error!(
    "The `futures` feature requires the `unstable` feature as an explicit opt-in to unstable features"
);

#[cfg(all(feature = "generator_trait", not(feature = "unstable")))]
compile_error!(
    "The `generator_trait` feature requires the `unstable` feature as an explicit opt-in to unstable features"
);

#[cfg(all(feature = "fn_traits", not(feature = "unstable")))]
compile_error!(
    "The `fn_traits` feature requires the `unstable` feature as an explicit opt-in to unstable features"
);

#[cfg(all(feature = "trusted_len", not(feature = "unstable")))]
compile_error!(
    "The `trusted_len` feature requires the `unstable` feature as an explicit opt-in to unstable features"
);

#[cfg(all(feature = "exact_size_is_empty", not(feature = "unstable")))]
compile_error!(
    "The `exact_size_is_empty` feature requires the `unstable` feature as an explicit opt-in to unstable features"
);

#[cfg(all(feature = "read_initializer", not(feature = "unstable")))]
compile_error!(
    "The `read_initializer` feature requires the `unstable` feature as an explicit opt-in to unstable features"
);

extern crate proc_macro;

#[macro_use]
mod utils;

mod derive;
mod enum_derive;

use proc_macro::TokenStream;

/// An attribute macro like a wrapper of `#[derive]`, implementing
/// the supported traits and passing unsupported traits to `#[derive]`.
#[proc_macro_attribute]
pub fn enum_derive(args: TokenStream, input: TokenStream) -> TokenStream {
    TokenStream::from(self::enum_derive::attribute(args.into(), input.into()))
}
