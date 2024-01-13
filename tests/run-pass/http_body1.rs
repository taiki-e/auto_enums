// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate http_body1_crate as http_body;

use auto_enums::enum_derive;

#[enum_derive(http_body1::Body)]
enum HttpBody1<A, B> {
    A(A),
    B(B),
}

fn main() {}
