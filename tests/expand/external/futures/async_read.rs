use auto_enums::enum_derive;

#[enum_derive(futures03::AsyncRead)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
