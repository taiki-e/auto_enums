// compile-fail

#![deny(warnings)]

use auto_enums::enum_derive;

#[enum_derive(Clone)]
enum Enum2<B> {
    A, //~ ERROR an enum with units variant is not supported
    B(B),
}

#[enum_derive(Clone)]
enum Enum3 {
    A = 2, //~ ERROR an enum with discriminants is not supported
    B,
}

#[enum_derive(Clone)]
enum Enum4<A, B> {
    A { x: A }, //~ ERROR an enum with named fields variant is not supported
    B(B),
}

#[enum_derive(Clone)]
enum Enum5<B> {
    A(), //~ ERROR a variant with zero fields is not supported
    B(B),
}

#[enum_derive(Clone)]
enum Enum6<A> {
    A(A),
    B(A, B), //~ ERROR a variant with multiple fields is not supported
}

#[enum_derive(Clone)]
enum Enum7<A> {
    //~^ ERROR cannot be implemented for enums with less than two variants
    A(A),
}

#[enum_derive(Clone)]
enum Enum8 {} //~ ERROR cannot be implemented for enums with less than two variants

fn main() {}
