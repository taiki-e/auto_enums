use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B> ::core::future::Future for Enum<A, B>
where
    A: ::core::future::Future,
    B: ::core::future::Future<Output = <A as ::core::future::Future>::Output>,
{
    type Output = <A as ::core::future::Future>::Output;
    #[inline]
    fn poll(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<Self::Output> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    <A as ::core::future::Future>::poll(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                    )
                }
                Enum::B(x) => {
                    <B as ::core::future::Future>::poll(
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
