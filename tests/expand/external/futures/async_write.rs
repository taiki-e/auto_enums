use auto_enums::enum_derive;

#[enum_derive(futures03::AsyncWrite)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
