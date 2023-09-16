// SPDX-License-Identifier: Apache-2.0 OR MIT

use auto_enums::enum_derive;

#[enum_derive(futures03::AsyncBufRead)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
