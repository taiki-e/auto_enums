use auto_enums::enum_derive;

#[enum_derive(rayon::ParallelExtend)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
