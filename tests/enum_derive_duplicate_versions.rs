#![cfg_attr(not(feature = "std"), no_std)]
#![warn(rust_2018_idioms, single_use_lifetimes)]
#![allow(dead_code)]

#[cfg(feature = "futures01")]
extern crate futures01_crate as futures;
#[cfg(feature = "tokio02")]
extern crate tokio01_crate as tokio;

#[allow(unused_imports)]
use auto_enums::enum_derive;

#[cfg(feature = "std")]
#[test]
fn stable_external() {
    #[cfg(feature = "futures01")]
    #[enum_derive(futures01::Future, futures01::Sink, futures01::Stream)]
    enum Futures01<A, B> {
        A(A),
        B(B),
    }

    #[cfg(feature = "tokio01")]
    #[enum_derive(tokio01::AsyncRead, tokio01::AsyncWrite, Read, Write)]
    enum Tokio01<A, B> {
        A(A),
        B(B),
    }
}
