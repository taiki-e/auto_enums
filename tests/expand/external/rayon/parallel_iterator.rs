use auto_enums::enum_derive;

#[enum_derive(rayon::ParallelIterator)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
