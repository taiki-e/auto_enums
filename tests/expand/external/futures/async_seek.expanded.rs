use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
#[allow(unsafe_code)]
impl<A, B> ::futures::io::AsyncSeek for Enum<A, B>
where
    A: ::futures::io::AsyncSeek,
    B: ::futures::io::AsyncSeek,
{
    #[inline]
    fn poll_seek(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
        pos: ::std::io::SeekFrom,
    ) -> ::core::task::Poll<::std::io::Result<u64>> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    ::futures::io::AsyncSeek::poll_seek(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                        pos,
                    )
                }
                Enum::B(x) => {
                    ::futures::io::AsyncSeek::poll_seek(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                        pos,
                    )
                }
            }
        }
    }
}
fn main() {}
