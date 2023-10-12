// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate futures03_crate as futures;

use auto_enums::enum_derive;

#[enum_derive(futures03::AsyncRead)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
