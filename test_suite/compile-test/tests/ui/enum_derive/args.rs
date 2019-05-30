// compile-fail

#![deny(warnings)]

use auto_enums::enum_derive;

#[enum_derive(Clone, =>)]
enum Enum1<A, B> {
    A(A),
    B(B),
}

#[enum_derive(foo::bar::!)]
enum Enum1<A, B> {
    A(A),
    B(B),
}

fn main() {}
