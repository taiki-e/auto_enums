// compile-fail

#![deny(warnings)]

use auto_enums::enum_derive;

#[enum_derive(Clone)]
enum Enum2<B> {
    A,
    B(B),
}

#[enum_derive(Clone)]
enum Enum3 {
    A = 2,
    B,
}

#[enum_derive(Clone)]
enum Enum4<A, B> {
    A { x: A },
    B(B),
}

#[enum_derive(Clone)]
enum Enum5<B> {
    A(),
    B(B),
}

#[enum_derive(Clone)]
enum Enum6<A> {
    A(A),
    B(A, B),
}

#[enum_derive(Clone)]
enum Enum7<A> {
    A(A),
}

#[enum_derive(Clone)]
enum Enum8 {}

fn main() {}
