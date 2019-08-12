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
