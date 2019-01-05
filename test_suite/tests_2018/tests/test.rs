#![cfg_attr(feature = "unstable", feature(proc_macro_hygiene, stmt_expr_attributes))]
#![cfg_attr(feature = "unstable", feature(futures_api))]
#![cfg_attr(feature = "unstable", feature(fn_traits, unboxed_closures))]
#![cfg_attr(feature = "unstable", feature(read_initializer))]
#![cfg_attr(feature = "unstable", feature(trusted_len))]
#![cfg_attr(feature = "unstable", feature(exact_size_is_empty))]
#![cfg_attr(feature = "unstable", feature(try_trait))]
#![cfg_attr(feature = "unstable", feature(unsized_locals))]
#![deny(warnings)]
#![allow(unused_imports)]
#![cfg(test)]

#[macro_use]
extern crate auto_enums;

#[cfg(feature = "unstable")]
mod test_futures {
    #[test]
    fn stream() {
        use futures::executor::block_on;
        use futures::stream::{self, StreamExt};

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
        use futures::channel::mpsc;
        use futures::executor::block_on;
        use futures::sink::SinkExt;
        use futures::stream::StreamExt;
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

#[cfg(test)]
mod enum_derive {
    #![allow(dead_code)]

    #[cfg(feature = "unstable")]
    #[test]
    fn unstable() {
        #[enum_derive(
            Future,
            futures::Stream,
            futures::Sink,
            futures::AsyncRead,
            futures::AsyncWrite
        )]
        enum Enum1<A, B> {
            A(A),
            B(B),
        }
    }
}
