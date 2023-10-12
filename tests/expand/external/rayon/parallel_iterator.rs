// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate rayon_crate as rayon;

use auto_enums::enum_derive;

#[enum_derive(rayon::ParallelIterator)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
