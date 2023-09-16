// SPDX-License-Identifier: Apache-2.0 OR MIT

use auto_enums::enum_derive;

#[enum_derive(Extend)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
