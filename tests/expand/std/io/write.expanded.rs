use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B> ::std::io::Write for Enum<A, B>
where
    A: ::std::io::Write,
    B: ::std::io::Write,
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
        match self {
            Enum::A(x) => <A as ::std::io::Write>::write(x, buf),
            Enum::B(x) => <B as ::std::io::Write>::write(x, buf),
        }
    }
    #[inline]
    fn write_vectored(
        &mut self,
        bufs: &[::std::io::IoSlice<'_>],
    ) -> ::std::io::Result<usize> {
        match self {
            Enum::A(x) => <A as ::std::io::Write>::write_vectored(x, bufs),
            Enum::B(x) => <B as ::std::io::Write>::write_vectored(x, bufs),
        }
    }
    #[inline]
    fn flush(&mut self) -> ::std::io::Result<()> {
        match self {
            Enum::A(x) => <A as ::std::io::Write>::flush(x),
            Enum::B(x) => <B as ::std::io::Write>::flush(x),
        }
    }
    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> ::std::io::Result<()> {
        match self {
            Enum::A(x) => <A as ::std::io::Write>::write_all(x, buf),
            Enum::B(x) => <B as ::std::io::Write>::write_all(x, buf),
        }
    }
    #[inline]
    fn write_fmt(&mut self, fmt: ::std::fmt::Arguments<'_>) -> ::std::io::Result<()> {
        match self {
            Enum::A(x) => <A as ::std::io::Write>::write_fmt(x, fmt),
            Enum::B(x) => <B as ::std::io::Write>::write_fmt(x, fmt),
        }
    }
}
fn main() {}
