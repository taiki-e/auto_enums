// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate futures01_crate as futures;

use auto_enums::enum_derive;

#[enum_derive(futures01::Future, futures01::Sink, futures01::Stream)]
enum Futures01<A, B> {
    A(A),
    B(B),
}

fn main() {}
