use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B> ::std::io::Seek for Enum<A, B>
where
    A: ::std::io::Seek,
    B: ::std::io::Seek,
{
    #[inline]
    fn seek(&mut self, pos: ::std::io::SeekFrom) -> ::std::io::Result<u64> {
        match self {
            Enum::A(x) => <A as ::std::io::Seek>::seek(x, pos),
            Enum::B(x) => <B as ::std::io::Seek>::seek(x, pos),
        }
    }
}
fn main() {}
