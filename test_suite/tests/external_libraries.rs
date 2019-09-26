#![warn(unsafe_code)]
#![warn(rust_2018_idioms)]
#![allow(dead_code)]

use auto_enums::enum_derive;

#[test]
fn stable_external() {
    #[enum_derive(
        rayon::ParallelIterator,
        rayon::IndexedParallelIterator,
        rayon::ParallelExtend,
        serde::Serialize
    )]
    enum Enum1<A, B> {
        A(A),
        B(B),
    }

    #[cfg(feature = "unstable")]
    #[enum_derive(
        Future,
        futures::Stream,
        futures::Sink,
        futures::AsyncRead,
        futures::AsyncWrite,
        futures::AsyncSeek,
        futures::AsyncBufRead
    )]
    enum Future<A, B> {
        A(A),
        B(B),
    }

    // #[enum_derive(futures01::Future, futures01::Sink, futures01::Stream)]
    // enum Future01<A, B> {
    //     A(A),
    //     B(B),
    // }
}
