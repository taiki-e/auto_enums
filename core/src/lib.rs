//! An internal crate to support auto_enums - **do not use directly**

#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/auto_enums_core/0.7.1")]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms, single_use_lifetimes), allow(dead_code))
))]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, unreachable_pub)]
// It cannot be included in the published code because these lints have false positives in the minimum required version.
#![cfg_attr(test, warn(single_use_lifetimes))]
#![warn(clippy::all)]
// `auto_enum` uses the hash value of the input AST to prevent access to the generated enum.
// This works well for common use cases, but is inconvenient when testing error messages that contain enum names.
// When this feature is enabled, `auto_enum` uses the enum name is based on the function name,
// and access to the enum is prevented by using `Span::def_site()`.
// Currently, this feature only affects when `auto_enum` is used in function position.
//
// This is disabled by default and can be enabled using
// `--cfg auto_enums_def_site_enum_ident` in RUSTFLAGS.
#![cfg_attr(auto_enums_def_site_enum_ident, feature(proc_macro_def_site))]

extern crate proc_macro;

#[macro_use]
mod utils;

mod auto_enum;

use proc_macro::TokenStream;

/// An attribute macro for to allow multiple return types by automatically generated enum.
#[proc_macro_attribute]
pub fn auto_enum(args: TokenStream, input: TokenStream) -> TokenStream {
    crate::auto_enum::attribute(args.into(), input.into()).into()
}
