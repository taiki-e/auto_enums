use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
#[allow(unsafe_code)]
impl<A, B> ::futures::io::AsyncRead for Enum<A, B>
where
    A: ::futures::io::AsyncRead,
    B: ::futures::io::AsyncRead,
{
    #[inline]
    fn poll_read(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
        buf: &mut [u8],
    ) -> ::core::task::Poll<::std::io::Result<usize>> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    ::futures::io::AsyncRead::poll_read(::core::pin::Pin::new_unchecked(x), cx, buf)
                }
                Enum::B(x) => {
                    ::futures::io::AsyncRead::poll_read(::core::pin::Pin::new_unchecked(x), cx, buf)
                }
            }
        }
    }
    #[inline]
    fn poll_read_vectored(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
        bufs: &mut [::std::io::IoSliceMut<'_>],
    ) -> ::core::task::Poll<::std::io::Result<usize>> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => ::futures::io::AsyncRead::poll_read_vectored(
                    ::core::pin::Pin::new_unchecked(x),
                    cx,
                    bufs,
                ),
                Enum::B(x) => ::futures::io::AsyncRead::poll_read_vectored(
                    ::core::pin::Pin::new_unchecked(x),
                    cx,
                    bufs,
                ),
            }
        }
    }
}
fn main() {}
