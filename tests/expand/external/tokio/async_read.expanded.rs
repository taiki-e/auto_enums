extern crate tokio1_crate as tokio;
use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
#[allow(unsafe_code)]
impl<A, B> ::tokio::io::AsyncRead for Enum<A, B>
where
    A: ::tokio::io::AsyncRead,
    B: ::tokio::io::AsyncRead,
{
    fn poll_read(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
        buf: &mut ::tokio::io::ReadBuf<'_>,
    ) -> ::core::task::Poll<::std::io::Result<()>> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    ::tokio::io::AsyncRead::poll_read(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                        buf,
                    )
                }
                Enum::B(x) => {
                    ::tokio::io::AsyncRead::poll_read(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                        buf,
                    )
                }
            }
        }
    }
}
fn main() {}
