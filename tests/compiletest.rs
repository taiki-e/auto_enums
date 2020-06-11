#![cfg(all(feature = "std", feature = "type_analysis", feature = "transpose_methods"))]
#![warn(rust_2018_idioms, single_use_lifetimes)]

#[cfg_attr(not(auto_enums_def_site_enum_ident), ignore)]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/auto_enum/*.rs");
    t.compile_fail("tests/ui/enum_derive/*.rs");
}
