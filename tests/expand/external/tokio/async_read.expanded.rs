extern crate tokio1_crate as tokio;
use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
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
                    <A as ::tokio::io::AsyncRead>::poll_read(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                        buf,
                    )
                }
                Enum::B(x) => {
                    <B as ::tokio::io::AsyncRead>::poll_read(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                        buf,
                    )
                }
            }
        }
    }
}
impl<A, B> ::core::marker::Unpin for Enum<A, B>
where
    A: ::core::marker::Unpin,
    B: ::core::marker::Unpin,
{}
const _: () = {
    trait MustNotImplDrop {}
    #[allow(clippy::drop_bounds, drop_bounds)]
    impl<T: ::core::ops::Drop> MustNotImplDrop for T {}
    impl<A, B> MustNotImplDrop for Enum<A, B> {}
};
fn main() {}
