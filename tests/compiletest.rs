#![cfg(all(
    feature = "std",
    feature = "type_analysis",
    feature = "transpose_methods",
    feature = "futures01",
    feature = "futures03",
    feature = "tokio01",
    feature = "tokio02",
    feature = "tokio03",
))]
#![warn(rust_2018_idioms, single_use_lifetimes)]

#[rustversion::attr(not(nightly), ignore)]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/run-pass/*.rs");
    t.compile_fail("tests/ui/auto_enum/*.rs");
    t.compile_fail("tests/ui/enum_derive/*.rs");
}
