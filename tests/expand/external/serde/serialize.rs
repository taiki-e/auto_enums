use auto_enums::enum_derive;

#[enum_derive(serde::Serialize)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
