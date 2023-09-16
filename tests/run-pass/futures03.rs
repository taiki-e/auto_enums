// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate futures03_crate as futures;

use auto_enums::enum_derive;

#[enum_derive(
    futures03::Stream,
    futures03::Sink,
    futures03::AsyncRead,
    futures03::AsyncWrite,
    futures03::AsyncSeek,
    futures03::AsyncBufRead
)]
enum Futures03<A, B> {
    A(A),
    B(B),
}

fn main() {}
