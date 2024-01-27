use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B, __A> ::core::iter::Extend<__A> for Enum<A, B>
where
    A: ::core::iter::Extend<__A>,
    B: ::core::iter::Extend<__A>,
{
    #[inline]
    fn extend<__T: ::core::iter::IntoIterator<Item = __A>>(&mut self, iter: __T) {
        match self {
            Enum::A(x) => <A as ::core::iter::Extend<__A>>::extend(x, iter),
            Enum::B(x) => <B as ::core::iter::Extend<__A>>::extend(x, iter),
        }
    }
}
fn main() {}
