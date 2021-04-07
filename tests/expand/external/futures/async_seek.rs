use auto_enums::enum_derive;

#[enum_derive(futures03::AsyncSeek)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
