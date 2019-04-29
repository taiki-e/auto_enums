#![cfg_attr(
    feature = "unstable",
    feature(
        proc_macro_hygiene,
        stmt_expr_attributes,
        fn_traits,
        unboxed_closures,
        exact_size_is_empty,
        generator_trait,
        iovec,
        read_initializer,
        trusted_len,
        try_trait,
    )
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(not(feature = "std"), feature = "unstable"), feature(alloc))]
#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[cfg(all(not(feature = "std"), feature = "unstable"))]
extern crate alloc;

use auto_enums::enum_derive;

#[test]
fn stable_1_30() {
    #[enum_derive(
        Transpose,
        Iterator,
        DoubleEndedIterator,
        ExactSizeIterator,
        FusedIterator,
        Extend,
        RangeBounds,
        Deref,
        DerefMut,
        Index,
        IndexMut,
        AsRef,
        AsMut,
        Debug,
        Display,
        fmt::Binary,
        fmt::LowerExp,
        fmt::LowerHex,
        fmt::Octal,
        fmt::Pointer,
        fmt::UpperExp,
        fmt::UpperHex,
        fmt::Write,
        Clone,
        Copy,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash
    )]
    enum Enum1<A, B, C, D> {
        A(A),
        B(B),
        C(C),
        D(D),
    }

    #[enum_derive(Iterator)]
    enum Enum2<A, B> {
        A(A),
        B(::core::ops::Range<B>),
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
fn stable_1_30_std() {
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

#[cfg(feature = "external_libraries")]
#[test]
fn stable_1_30_external() {
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

#[cfg(feature = "unstable")]
#[test]
fn unstable() {
    #[enum_derive(Future, Fn, FnMut, FnOnce, Generator, Iterator, TrustedLen)]
    enum Enum1<A, B> {
        A(A),
        B(B),
    }
}

#[cfg(all(feature = "std", feature = "unstable"))]
#[test]
fn unstable_std() {
    #[enum_derive(Read, Write)]
    enum Enum1<A, B> {
        A(A),
        B(B),
    }
}

#[test]
fn unfmt() {
    #[cfg_attr(feature = "rustfmt", rustfmt_skip)]
    #[enum_derive(Transpose, Iterator)]
    enum Enum1<A, B,> {
        A(A),
        B(B)
    }

    #[cfg(feature = "std")]
    #[cfg_attr(feature = "rustfmt", rustfmt_skip)]
    #[enum_derive(Iterator)]
    enum Enum2<> {
        A(::core::ops::Range<i32>),
        B(::std::vec::IntoIter<i32>),
    }
}
