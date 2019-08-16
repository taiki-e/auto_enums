#![warn(unsafe_code)]
#![warn(rust_2018_idioms)]
#![allow(dead_code)]

use auto_enums::enum_derive;

#[cfg(feature = "external_libraries")]
#[test]
fn stable_external() {
    #[enum_derive(
        futures01::Future,
        futures01::Sink,
        futures01::Stream,
        quote::ToTokens,
        rayon::ParallelIterator,
        rayon::IndexedParallelIterator,
        rayon::ParallelExtend,
        serde::Serialize
    )]
    enum Enum1<A, B, C, D> {
        A(A),
        B(B),
        C(C),
        D(D),
    }
}
