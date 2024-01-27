use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B> ::core::fmt::Display for Enum<A, B>
where
    A: ::core::fmt::Display,
    B: ::core::fmt::Display,
{
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Enum::A(x) => <A as ::core::fmt::Display>::fmt(x, f),
            Enum::B(x) => <B as ::core::fmt::Display>::fmt(x, f),
        }
    }
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
impl<A, B> ::std::error::Error for Enum<A, B>
where
    A: ::std::error::Error,
    B: ::std::error::Error,
    A: 'static,
    B: 'static,
{
    #[allow(deprecated)]
    fn description(&self) -> &str {
        match self {
            Enum::A(x) => <A as ::std::error::Error>::description(x),
            Enum::B(x) => <B as ::std::error::Error>::description(x),
        }
    }
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Enum::A(x) => ::std::option::Option::Some(x),
            Enum::B(x) => ::std::option::Option::Some(x),
        }
    }
}
fn main() {}
