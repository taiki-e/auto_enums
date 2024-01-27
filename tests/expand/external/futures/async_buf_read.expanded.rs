extern crate futures03_crate as futures;
use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B> ::futures::io::AsyncBufRead for Enum<A, B>
where
    A: ::futures::io::AsyncBufRead,
    B: ::futures::io::AsyncBufRead,
{
    #[inline]
    fn poll_fill_buf(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::std::io::Result<&[u8]>> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    <A as ::futures::io::AsyncBufRead>::poll_fill_buf(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                    )
                }
                Enum::B(x) => {
                    <B as ::futures::io::AsyncBufRead>::poll_fill_buf(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                    )
                }
            }
        }
    }
    #[inline]
    fn consume(self: ::core::pin::Pin<&mut Self>, amt: usize) {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    <A as ::futures::io::AsyncBufRead>::consume(
                        ::core::pin::Pin::new_unchecked(x),
                        amt,
                    )
                }
                Enum::B(x) => {
                    <B as ::futures::io::AsyncBufRead>::consume(
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
