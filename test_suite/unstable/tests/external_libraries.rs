#![warn(unsafe_code)]
#![warn(rust_2018_idioms)]
#![allow(dead_code)]

mod enum_derive {

    use auto_enums::enum_derive;

    #[test]
    fn unstable() {
        #[enum_derive(
            Future,
            futures::Stream,
            futures::Sink,
            futures::AsyncRead,
            futures::AsyncWrite,
            futures::AsyncSeek,
            futures::AsyncBufRead
        )]
        enum Enum1<A, B> {
            A(A),
            B(B),
        }
    }
}
