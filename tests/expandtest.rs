// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(not(miri))]

#[test]
fn expandtest() {
    if !build_context::RUSTC_NIGHTLY {
        return;
    }
    let args = &["--all-features"];
    macrotest::expand_args("tests/expand/**/*.rs", args);
}
