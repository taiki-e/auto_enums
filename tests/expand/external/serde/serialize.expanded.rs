extern crate serde_crate as serde;
use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B> ::serde::ser::Serialize for Enum<A, B>
where
    A: ::serde::ser::Serialize,
    B: ::serde::ser::Serialize,
{
    #[inline]
    fn serialize<__S>(
        &self,
        serializer: __S,
    ) -> ::core::result::Result<__S::Ok, __S::Error>
    where
        __S: ::serde::ser::Serializer,
    {
        match self {
            Enum::A(x) => <A as ::serde::ser::Serialize>::serialize(x, serializer),
            Enum::B(x) => <B as ::serde::ser::Serialize>::serialize(x, serializer),
        }
    }
}
fn main() {}
