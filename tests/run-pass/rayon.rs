// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate rayon_crate as rayon;

use auto_enums::enum_derive;

#[enum_derive(rayon::ParallelIterator, rayon::IndexedParallelIterator, rayon::ParallelExtend)]
enum Rayon<A, B> {
    A(A),
    B(B),
}

fn main() {}
