// SPDX-License-Identifier: Apache-2.0 OR MIT

use auto_enums::enum_derive;

#[enum_derive(Clone, =>)] //~ ERROR expected identifier
enum Enum1<A, B> {
    A(A),
    B(B),
}

#[enum_derive(foo::bar::!)] //~ ERROR expected identifier
enum Enum2<A, B> {
    A(A),
    B(B),
}

#[enum_derive(Clone, Foo:)] //~ ERROR expected `,`
enum Enum3<A, B> {
    A(A),
    B(B),
}

#[enum_derive(Clone Foo)] //~ ERROR expected `,`
enum Enum4<A, B> {
    A(A),
    B(B),
}

fn main() {}
