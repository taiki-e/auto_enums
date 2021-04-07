use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
#[allow(unsafe_code)]
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
                Enum::A(x) => ::core::future::Future::poll(::core::pin::Pin::new_unchecked(x), cx),
                Enum::B(x) => ::core::future::Future::poll(::core::pin::Pin::new_unchecked(x), cx),
            }
        }
    }
}
fn main() {}
