// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate tokio02_crate as tokio;

use auto_enums::enum_derive;

#[enum_derive(tokio02::AsyncRead, tokio02::AsyncWrite, tokio02::AsyncSeek, tokio02::AsyncBufRead)]
enum Tokio02<A, B> {
    A(A),
    B(B),
}

fn main() {}
