// compile-fail

#![deny(warnings)]

use auto_enums::enum_derive;

struct Foo<A>(A);

#[enum_derive(Transpose)]
enum Enum1<A, B> {
    A(Foo<A>),
    B(B),
}

fn main() {}
