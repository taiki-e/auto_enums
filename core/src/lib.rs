#![crate_type = "proc-macro"]
#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/auto_enums_core/0.3.3")]

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate rand;
extern crate smallvec;
extern crate syn;

#[macro_use]
mod utils;

mod attribute;

use proc_macro::TokenStream;

/// An attribute macro for to allow multiple return types by automatically generated enum.
#[proc_macro_attribute]
pub fn auto_enum(args: TokenStream, input: TokenStream) -> TokenStream {
    crate::attribute::attribute(args, input)
}
