extern crate http_body1_crate as http_body;
use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B> ::http_body::Body for Enum<A, B>
where
    A: ::http_body::Body,
    B: ::http_body::Body<
        Data = <A as ::http_body::Body>::Data,
        Error = <A as ::http_body::Body>::Error,
    >,
{
    type Data = <A as ::http_body::Body>::Data;
    type Error = <A as ::http_body::Body>::Error;
    #[inline]
    fn is_end_stream(&self) -> bool {
        match self {
            Enum::A(x) => <A as ::http_body::Body>::is_end_stream(x),
            Enum::B(x) => <B as ::http_body::Body>::is_end_stream(x),
        }
    }
    #[inline]
    fn size_hint(&self) -> ::http_body::SizeHint {
        match self {
            Enum::A(x) => <A as ::http_body::Body>::size_hint(x),
            Enum::B(x) => <B as ::http_body::Body>::size_hint(x),
        }
    }
    fn poll_frame(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<
        ::core::option::Option<
            ::core::result::Result<::http_body::Frame<Self::Data>, Self::Error>,
        >,
    > {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    <A as ::http_body::Body>::poll_frame(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                    )
                }
                Enum::B(x) => {
                    <B as ::http_body::Body>::poll_frame(
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
