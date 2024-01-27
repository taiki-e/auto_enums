extern crate futures03_crate as futures;
use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B> ::futures::stream::Stream for Enum<A, B>
where
    A: ::futures::stream::Stream,
    B: ::futures::stream::Stream<Item = <A as ::futures::stream::Stream>::Item>,
{
    type Item = <A as ::futures::stream::Stream>::Item;
    #[inline]
    fn size_hint(&self) -> (usize, ::core::option::Option<usize>) {
        match self {
            Enum::A(x) => <A as ::futures::stream::Stream>::size_hint(x),
            Enum::B(x) => <B as ::futures::stream::Stream>::size_hint(x),
        }
    }
    #[inline]
    fn poll_next(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::core::option::Option<Self::Item>> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    <A as ::futures::stream::Stream>::poll_next(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                    )
                }
                Enum::B(x) => {
                    <B as ::futures::stream::Stream>::poll_next(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
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
