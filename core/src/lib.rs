#![crate_type = "proc-macro"]
#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/auto_enums_core/0.5.2")]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms, unreachable_pub)]
#![deny(clippy::all, clippy::pedantic)]

extern crate proc_macro;

#[macro_use]
mod utils;

mod attribute;

use proc_macro::TokenStream;

/// An attribute macro for to allow multiple return types by automatically generated enum.
#[proc_macro_attribute]
pub fn auto_enum(args: TokenStream, input: TokenStream) -> TokenStream {
    TokenStream::from(self::attribute::attribute(args.into(), input.into()))
}
