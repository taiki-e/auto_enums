#![crate_type = "proc-macro"]
#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/auto_enums_core/0.5.2")]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms, unreachable_pub)]
#![deny(clippy::all, clippy::pedantic)]
#![warn(single_use_lifetimes)]
#![warn(clippy::nursery)]

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
