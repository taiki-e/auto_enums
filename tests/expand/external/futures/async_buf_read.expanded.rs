use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
#[allow(unsafe_code)]
impl<A, B> ::futures::io::AsyncBufRead for Enum<A, B>
where
    A: ::futures::io::AsyncBufRead,
    B: ::futures::io::AsyncBufRead,
{
    #[inline]
    fn poll_fill_buf<'__a>(
        self: ::core::pin::Pin<&'__a mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::std::io::Result<&'__a [u8]>> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => ::futures::io::AsyncBufRead::poll_fill_buf(
                    ::core::pin::Pin::new_unchecked(x),
                    cx,
                ),
                Enum::B(x) => ::futures::io::AsyncBufRead::poll_fill_buf(
                    ::core::pin::Pin::new_unchecked(x),
                    cx,
                ),
            }
        }
    }
    #[inline]
    fn consume(self: ::core::pin::Pin<&mut Self>, amt: usize) {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    ::futures::io::AsyncBufRead::consume(::core::pin::Pin::new_unchecked(x), amt)
                }
                Enum::B(x) => {
                    ::futures::io::AsyncBufRead::consume(::core::pin::Pin::new_unchecked(x), amt)
                }
            }
        }
    }
}
fn main() {}
