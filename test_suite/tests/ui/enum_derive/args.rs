// compile-fail

#![deny(warnings)]

use auto_enums::enum_derive;

#[enum_derive(Clone, =>)] //~ ERROR expected one of `,`, `::`, or identifier, found `=`
enum Enum1<A, B> {
    A(A),
    B(B),
}

#[enum_derive(foo::bar::!)] //~ ERROR expected identifier
enum Enum1<A, B> {
    A(A),
    B(B),
}

fn main() {}
