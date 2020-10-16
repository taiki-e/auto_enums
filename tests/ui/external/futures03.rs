extern crate futures03_crate as futures;

use auto_enums::enum_derive;

#[enum_derive(
    futures::Stream,
    futures::Sink,
    futures::AsyncRead,
    futures::AsyncWrite,
    futures::AsyncSeek,
    futures::AsyncBufRead
)]
enum Futures03<A, B> {
    A(A),
    B(B),
}

fn main() {}
