#![cfg(all(
    feature = "std",
    feature = "type_analysis",
    feature = "transpose_methods",
    feature = "futures01",
    feature = "futures03",
    feature = "rayon",
    feature = "serde",
    feature = "tokio01",
    feature = "tokio02",
    feature = "tokio03",
))]
#![warn(rust_2018_idioms, single_use_lifetimes)]

use std::env;

#[rustversion::attr(not(nightly), ignore)]
#[test]
fn ui() {
    if !env::var_os("CI").map_or(false, |v| v == "true") {
        env::set_var("TRYBUILD", "overwrite");
    }

    let t = trybuild::TestCases::new();
    t.pass("tests/ui/external/*.rs");
    t.compile_fail("tests/ui/auto_enum/*.rs");
    t.compile_fail("tests/ui/enum_derive/*.rs");
}
