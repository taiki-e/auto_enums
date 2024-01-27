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
fn main() {}
