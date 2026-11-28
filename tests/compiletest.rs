// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(not(miri))]
#![cfg(all(
    feature = "std",
    feature = "coroutine_trait",
    feature = "fn_traits",
    feature = "trusted_len",
    feature = "type_analysis",
    feature = "transpose_methods",
    feature = "futures01",
    feature = "futures03",
    feature = "rayon",
    feature = "serde",
    feature = "serde_core",
    feature = "tokio01",
    feature = "tokio02",
    feature = "tokio03",
    feature = "tokio1",
    feature = "http_body1",
))]

#[test]
fn ui() {
    if !build_context::RUSTC_NIGHTLY {
        return;
    }
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/**/*.rs");
    t.pass("tests/run-pass/**/*.rs");
}
