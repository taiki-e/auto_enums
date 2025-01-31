// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate tokio01_crate as tokio;

use auto_enums::enum_derive;

#[enum_derive(tokio01::AsyncRead, tokio01::AsyncWrite, Read, Write)]
enum Tokio01<A, B> {
    A(A),
    B(B),
}

fn main() {}
