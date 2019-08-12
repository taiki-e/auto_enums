#![feature(
    proc_macro_hygiene,
    stmt_expr_attributes,
    fn_traits,
    unboxed_closures,
    exact_size_is_empty,
    generator_trait,
    read_initializer,
    trusted_len,
    try_trait
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(unsafe_code)]
#![warn(rust_2018_idioms)]
#![allow(dead_code)]

use auto_enums::enum_derive;

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
    enum Enum1<A, B, C, D> {
        A(A),
        B(B),
        C(C),
        D(D),
    }

    #[cfg(feature = "ops")]
    #[enum_derive(Deref, DerefMut, Index, IndexMut, RangeBounds)]
    enum Ops<A, B, C, D> {
        A(A),
        B(B),
        C(C),
        D(D),
    }

    #[cfg(feature = "convert")]
    #[enum_derive(AsRef, AsMut)]
    enum Convert<A, B, C, D> {
        A(A),
        B(B),
        C(C),
        D(D),
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
    enum Fmt<A, B, C, D> {
        A(A),
        B(B),
        C(C),
        D(D),
    }

    #[cfg(feature = "transpose_methods")]
    #[enum_derive(Transpose)]
    enum Transpose<A, B, C, D> {
        A(A),
        B(B),
        C(C),
        D(D),
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
    enum Enum<A, B, C, D> {
        A(A),
        B(B),
        C(C),
        D(D),
    }
}

#[test]
fn unstable() {
    #[enum_derive(Fn, FnMut, FnOnce, Generator, Iterator, TrustedLen)]
    enum Enum1<A, B> {
        A(A),
        B(B),
    }
}

#[cfg(feature = "std")]
#[test]
fn unstable_std() {
    #[enum_derive(Read)]
    enum Enum1<A, B> {
        A(A),
        B(B),
    }
}

#[test]
fn unfmt() {
    #[rustfmt::skip]
    #[enum_derive(Debug, Iterator)]
    enum Enum1<A, B,> {
        A(A),
        B(B)
    }

    #[cfg(feature = "std")]
    #[rustfmt::skip]
    #[enum_derive(Iterator)]
    enum Enum2<> {
        A(::core::ops::Range<i32>),
        B(::std::vec::IntoIter<i32>),
    }
}
