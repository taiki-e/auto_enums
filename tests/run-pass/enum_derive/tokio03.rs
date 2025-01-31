// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate tokio03_crate as tokio;

use auto_enums::enum_derive;

#[enum_derive(tokio03::AsyncRead, tokio03::AsyncWrite, tokio03::AsyncSeek, tokio03::AsyncBufRead)]
enum Tokio03<A, B> {
    A(A),
    B(B),
}

fn main() {}
