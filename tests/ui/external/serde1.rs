extern crate serde1_crate as serde;

use auto_enums::enum_derive;

#[enum_derive(serde::Serialize)]
enum Serde<A, B> {
    A(A),
    B(B),
}

fn main() {}
