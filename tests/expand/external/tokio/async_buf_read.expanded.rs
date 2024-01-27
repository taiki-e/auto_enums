extern crate tokio1_crate as tokio;
use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B> ::tokio::io::AsyncBufRead for Enum<A, B>
where
    A: ::tokio::io::AsyncBufRead,
    B: ::tokio::io::AsyncBufRead,
{
    fn poll_fill_buf(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::std::io::Result<&[u8]>> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    <A as ::tokio::io::AsyncBufRead>::poll_fill_buf(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                    )
                }
                Enum::B(x) => {
                    <B as ::tokio::io::AsyncBufRead>::poll_fill_buf(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                    )
                }
            }
        }
    }
    fn consume(self: ::core::pin::Pin<&mut Self>, amt: usize) {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    <A as ::tokio::io::AsyncBufRead>::consume(
                        ::core::pin::Pin::new_unchecked(x),
                        amt,
                    )
                }
                Enum::B(x) => {
                    <B as ::tokio::io::AsyncBufRead>::consume(
                        ::core::pin::Pin::new_unchecked(x),
                        amt,
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
