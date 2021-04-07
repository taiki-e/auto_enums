use auto_enums::enum_derive;

#[enum_derive(futures03::Sink)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
