// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(coroutine_trait)]
#![feature(fn_traits, unboxed_closures)]
#![feature(trusted_len)]

use auto_enums::enum_derive;

#[enum_derive(Coroutine, Fn, FnMut, FnOnce, TrustedLen)]
enum Enum1<A, B> {
    A(A),
    B(B),
}

fn main() {}
