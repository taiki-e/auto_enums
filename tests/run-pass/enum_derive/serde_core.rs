// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate serde_core_crate as serde_core;

use auto_enums::enum_derive;

#[enum_derive(serde_core::Serialize)]
enum SerdeCore<A, B> {
    A(A),
    B(B),
}

fn main() {}
