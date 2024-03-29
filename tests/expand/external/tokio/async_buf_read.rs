// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate tokio1_crate as tokio;

use auto_enums::enum_derive;

#[enum_derive(tokio1::AsyncBufRead)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
