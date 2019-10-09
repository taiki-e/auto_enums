#![cfg_attr(feature = "generator_trait", feature(generator_trait))]
#![cfg_attr(feature = "fn_traits", feature(fn_traits, unboxed_closures))]
#![cfg_attr(feature = "trusted_len", feature(trusted_len))]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(unsafe_code)]
#![warn(rust_2018_idioms)]
#![allow(dead_code)]

#[cfg(feature = "rayon")]
extern crate rayon_crate as rayon;
#[cfg(feature = "serde")]
extern crate serde_crate as serde;

use auto_enums::enum_derive;

#[test]
fn unfmt() {
    #[rustfmt::skip]
    #[enum_derive(Debug, Iterator)]
    enum Enum1<A, B,> {
        A(A),
        B(B)
    }

    #[rustfmt::skip]
    #[enum_derive(Iterator)]
    enum Enum2<> {
        A(::core::ops::Range<i32>),
        B(::core::ops::RangeInclusive<i32>),
    }
}

#[test]
fn stable() {
    #[enum_derive(
        Iterator,
        DoubleEndedIterator,
        ExactSizeIterator,
        FusedIterator,
        Extend,
        Debug,
        Display,
        fmt::Write,
        Clone,
        Copy,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        Future
    )]
    enum Stable<A, B> {
        A(A),
        B(B),
    }

    #[cfg(feature = "ops")]
    #[enum_derive(Deref, DerefMut, Index, IndexMut, RangeBounds)]
    enum Ops<A, B> {
        A(A),
        B(B),
    }

    #[cfg(feature = "convert")]
    #[enum_derive(AsRef, AsMut)]
    enum Convert<A, B> {
        A(A),
        B(B),
    }

    #[cfg(feature = "fmt")]
    #[enum_derive(
        fmt::Binary,
        fmt::LowerExp,
        fmt::LowerHex,
        fmt::Octal,
        fmt::Pointer,
        fmt::UpperExp,
        fmt::UpperHex
    )]
    enum Fmt<A, B> {
        A(A),
        B(B),
    }

    #[cfg(feature = "transpose_methods")]
    #[enum_derive(Transpose)]
    enum Transpose<A, B> {
        A(A),
        B(B),
    }

    #[enum_derive(Iterator, Clone)]
    #[enum_derive(Extend, Copy)]
    enum Enum3<A, B> {
        A(A),
        B(B),
    }
}

#[cfg(feature = "std")]
#[test]
fn stable_std() {
    #[enum_derive(
        BufRead, Read, Seek, Write, Display, Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd,
        Ord, Hash
    )]
    enum Stable<A, B> {
        A(A),
        B(B),
    }
}

#[cfg(feature = "std")]
#[test]
fn stable_external() {
    #[cfg(feature = "rayon")]
    #[enum_derive(rayon::ParallelIterator, rayon::IndexedParallelIterator, rayon::ParallelExtend)]
    enum Rayon<A, B> {
        A(A),
        B(B),
    }

    #[cfg(feature = "serde")]
    #[enum_derive(serde::Serialize)]
    enum Serde<A, B> {
        A(A),
        B(B),
    }

    #[cfg(feature = "futures")]
    #[enum_derive(
        Future,
        futures::Stream,
        futures::Sink,
        futures::AsyncRead,
        futures::AsyncWrite,
        futures::AsyncSeek,
        futures::AsyncBufRead
    )]
    enum Futures<A, B> {
        A(A),
        B(B),
    }

    // #[enum_derive(futures01::Future, futures01::Sink, futures01::Stream)]
    // enum Futures01<A, B> {
    //     A(A),
    //     B(B),
    // }
}

// nightly

#[cfg(feature = "generator_trait")]
#[test]
fn generator_trait() {
    #[enum_derive(Generator)]
    enum Enum1<A, B> {
        A(A),
        B(B),
    }
}

#[cfg(feature = "fn_traits")]
#[test]
fn fn_traits() {
    #[enum_derive(Fn, FnMut, FnOnce)]
    enum Enum1<A, B> {
        A(A),
        B(B),
    }
}

#[cfg(feature = "trusted_len")]
#[test]
fn trusted_len() {
    #[enum_derive(TrustedLen)]
    enum Enum1<A, B> {
        A(A),
        B(B),
    }
}
