use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B> ::std::io::Read for Enum<A, B>
where
    A: ::std::io::Read,
    B: ::std::io::Read,
{
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
        match self {
            Enum::A(x) => <A as ::std::io::Read>::read(x, buf),
            Enum::B(x) => <B as ::std::io::Read>::read(x, buf),
        }
    }
    #[inline]
    fn read_vectored(
        &mut self,
        bufs: &mut [::std::io::IoSliceMut<'_>],
    ) -> ::std::io::Result<usize> {
        match self {
            Enum::A(x) => <A as ::std::io::Read>::read_vectored(x, bufs),
            Enum::B(x) => <B as ::std::io::Read>::read_vectored(x, bufs),
        }
    }
    #[inline]
    fn read_to_end(
        &mut self,
        buf: &mut ::std::vec::Vec<u8>,
    ) -> ::std::io::Result<usize> {
        match self {
            Enum::A(x) => <A as ::std::io::Read>::read_to_end(x, buf),
            Enum::B(x) => <B as ::std::io::Read>::read_to_end(x, buf),
        }
    }
    #[inline]
    fn read_to_string(
        &mut self,
        buf: &mut ::std::string::String,
    ) -> ::std::io::Result<usize> {
        match self {
            Enum::A(x) => <A as ::std::io::Read>::read_to_string(x, buf),
            Enum::B(x) => <B as ::std::io::Read>::read_to_string(x, buf),
        }
    }
    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> ::std::io::Result<()> {
        match self {
            Enum::A(x) => <A as ::std::io::Read>::read_exact(x, buf),
            Enum::B(x) => <B as ::std::io::Read>::read_exact(x, buf),
        }
    }
}
impl<A, B> ::std::io::BufRead for Enum<A, B>
where
    A: ::std::io::BufRead,
    B: ::std::io::BufRead,
{
    #[inline]
    fn fill_buf(&mut self) -> ::std::io::Result<&[u8]> {
        match self {
            Enum::A(x) => <A as ::std::io::BufRead>::fill_buf(x),
            Enum::B(x) => <B as ::std::io::BufRead>::fill_buf(x),
        }
    }
    #[inline]
    fn consume(&mut self, amt: usize) {
        match self {
            Enum::A(x) => <A as ::std::io::BufRead>::consume(x, amt),
            Enum::B(x) => <B as ::std::io::BufRead>::consume(x, amt),
        }
    }
    #[inline]
    fn read_until(
        &mut self,
        byte: u8,
        buf: &mut ::std::vec::Vec<u8>,
    ) -> ::std::io::Result<usize> {
        match self {
            Enum::A(x) => <A as ::std::io::BufRead>::read_until(x, byte, buf),
            Enum::B(x) => <B as ::std::io::BufRead>::read_until(x, byte, buf),
        }
    }
    #[inline]
    fn read_line(
        &mut self,
        buf: &mut ::std::string::String,
    ) -> ::std::io::Result<usize> {
        match self {
            Enum::A(x) => <A as ::std::io::BufRead>::read_line(x, buf),
            Enum::B(x) => <B as ::std::io::BufRead>::read_line(x, buf),
        }
    }
}
fn main() {}
