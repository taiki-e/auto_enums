#![cfg(all(feature = "std", feature = "type_analysis", feature = "transpose_methods"))]
#![warn(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes)]

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/auto_enum/*.rs");
    t.compile_fail("tests/ui/enum_derive/*.rs");
}
