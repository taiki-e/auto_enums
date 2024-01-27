use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B> ::core::fmt::Debug for Enum<A, B>
where
    A: ::core::fmt::Debug,
    B: ::core::fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Enum::A(x) => <A as ::core::fmt::Debug>::fmt(x, f),
            Enum::B(x) => <B as ::core::fmt::Debug>::fmt(x, f),
        }
    }
}
fn main() {}
