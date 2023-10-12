// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate serde_crate as serde;

use auto_enums::enum_derive;

#[enum_derive(serde::Serialize)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
