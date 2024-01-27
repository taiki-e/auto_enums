extern crate futures03_crate as futures;
use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B, __Item> ::futures::sink::Sink<__Item> for Enum<A, B>
where
    A: ::futures::sink::Sink<__Item>,
    B: ::futures::sink::Sink<
        __Item,
        Error = <A as ::futures::sink::Sink<__Item>>::Error,
    >,
{
    type Error = <A as ::futures::sink::Sink<__Item>>::Error;
    #[inline]
    fn poll_ready(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::core::result::Result<(), Self::Error>> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    <A as ::futures::sink::Sink<
                        __Item,
                    >>::poll_ready(::core::pin::Pin::new_unchecked(x), cx)
                }
                Enum::B(x) => {
                    <B as ::futures::sink::Sink<
                        __Item,
                    >>::poll_ready(::core::pin::Pin::new_unchecked(x), cx)
                }
            }
        }
    }
    #[inline]
    fn start_send(
        self: ::core::pin::Pin<&mut Self>,
        item: __Item,
    ) -> ::core::result::Result<(), Self::Error> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    <A as ::futures::sink::Sink<
                        __Item,
                    >>::start_send(::core::pin::Pin::new_unchecked(x), item)
                }
                Enum::B(x) => {
                    <B as ::futures::sink::Sink<
                        __Item,
                    >>::start_send(::core::pin::Pin::new_unchecked(x), item)
                }
            }
        }
    }
    #[inline]
    fn poll_flush(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::core::result::Result<(), Self::Error>> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    <A as ::futures::sink::Sink<
                        __Item,
                    >>::poll_flush(::core::pin::Pin::new_unchecked(x), cx)
                }
                Enum::B(x) => {
                    <B as ::futures::sink::Sink<
                        __Item,
                    >>::poll_flush(::core::pin::Pin::new_unchecked(x), cx)
                }
            }
        }
    }
    #[inline]
    fn poll_close(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::core::result::Result<(), Self::Error>> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    <A as ::futures::sink::Sink<
                        __Item,
                    >>::poll_close(::core::pin::Pin::new_unchecked(x), cx)
                }
                Enum::B(x) => {
                    <B as ::futures::sink::Sink<
                        __Item,
                    >>::poll_close(::core::pin::Pin::new_unchecked(x), cx)
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
