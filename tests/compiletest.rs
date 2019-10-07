#![cfg(all(feature = "std", feature = "type_analysis", feature = "transpose_methods",))]
#![warn(rust_2018_idioms)]

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/auto_enum/*.rs");
    t.compile_fail("tests/ui/enum_derive/*.rs");
}
