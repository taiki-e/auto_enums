// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(not(miri))]
#![cfg(not(careful))]
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
    feature = "http_body1",
))]

#[rustversion::attr(not(nightly), ignore)]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/**/*.rs");
    t.pass("tests/run-pass/**/*.rs");
}
