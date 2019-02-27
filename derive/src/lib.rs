#![crate_type = "proc-macro"]
#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/auto_enums_derive/0.4.1")]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unreachable_pub)]

extern crate proc_macro;

#[macro_use]
mod utils;

mod attribute;
mod derive;

use proc_macro::TokenStream;

/// An attribute macro like a wrapper of `#[derive]`, implementing
/// the supported traits and passing unsupported traits to `#[derive]`.
#[proc_macro_attribute]
pub fn enum_derive(args: TokenStream, input: TokenStream) -> TokenStream {
    crate::attribute::attribute(args, input)
}
