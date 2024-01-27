extern crate futures03_crate as futures;
use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
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
                    <A as ::futures::io::AsyncSeek>::poll_seek(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                        pos,
                    )
                }
                Enum::B(x) => {
                    <B as ::futures::io::AsyncSeek>::poll_seek(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                        pos,
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
