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
    feature = "tokio1",
))]
#![warn(rust_2018_idioms, single_use_lifetimes)]

use std::env;

#[rustversion::attr(before(2021-08-23), ignore)] // Note: This date is commit-date and the day before the toolchain date.
#[test]
fn ui() {
    if env::var_os("CI").is_none() {
        env::set_var("TRYBUILD", "overwrite");
    }

    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/**/*.rs");
    t.pass("tests/run-pass/**/*.rs");
}
