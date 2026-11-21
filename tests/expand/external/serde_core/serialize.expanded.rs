extern crate serde_core_crate as serde_core;
use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
#[automatically_derived]
impl<A, B> ::serde_core::ser::Serialize for Enum<A, B>
where
    A: ::serde_core::ser::Serialize,
    B: ::serde_core::ser::Serialize,
{
    #[inline]
    fn serialize<__S>(
        &self,
        serializer: __S,
    ) -> ::core::result::Result<__S::Ok, __S::Error>
    where
        __S: ::serde_core::ser::Serializer,
    {
        match self {
            Enum::A(x) => <A as ::serde_core::ser::Serialize>::serialize(x, serializer),
            Enum::B(x) => <B as ::serde_core::ser::Serialize>::serialize(x, serializer),
        }
    }
}
fn main() {}
