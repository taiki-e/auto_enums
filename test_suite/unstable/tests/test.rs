#![cfg_attr(
    feature = "unstable",
    feature(
        proc_macro_hygiene,
        stmt_expr_attributes,
        fn_traits,
        unboxed_closures,
        read_initializer,
        trusted_len,
        exact_size_is_empty,
        try_trait,
    )
)]
#![warn(rust_2018_idioms)]
#![allow(unused_imports)]

#[cfg(feature = "unstable")]
mod test_futures {
    use auto_enums::auto_enum;

    #[test]
    fn stream() {
        use futures::{
            executor::block_on,
            stream::{self, StreamExt},
        };

        let x = 0;

        #[auto_enum(futures::Stream)]
        let mut stream = match x {
            0 => stream::iter(1..=3),
            _ => stream::iter(1..3),
        };

        assert_eq!(block_on(stream.next()), Some(1));
        assert_eq!(block_on(stream.next()), Some(2));
        assert_eq!(block_on(stream.next()), Some(3));
        assert_eq!(block_on(stream.next()), None);
    }

    #[test]
    fn sink() {
        use futures::{channel::mpsc, executor::block_on, sink::SinkExt, stream::StreamExt};
        use std::collections::VecDeque;

        let x = 0;
        let (tx, rx) = mpsc::channel(5);

        #[auto_enum(futures::Sink)]
        let mut tx = match x {
            0 => tx.with_flat_map(|x| VecDeque::from(vec![Ok(42); x])),
            _ => tx.with_flat_map(|x| VecDeque::from(vec![Ok(84); x])),
        };

        block_on(tx.send(5)).unwrap();
        drop(tx);
        let received: Vec<i32> = block_on(rx.collect());
        assert_eq!(received, vec![42, 42, 42, 42, 42]);
    }
}

mod enum_derive {
    #![allow(dead_code)]

    use auto_enums::enum_derive;

    #[cfg(feature = "unstable")]
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
