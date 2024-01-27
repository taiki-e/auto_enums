extern crate tokio1_crate as tokio;
use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B> ::tokio::io::AsyncSeek for Enum<A, B>
where
    A: ::tokio::io::AsyncSeek,
    B: ::tokio::io::AsyncSeek,
{
    fn start_seek(
        self: ::core::pin::Pin<&mut Self>,
        pos: ::std::io::SeekFrom,
    ) -> ::std::io::Result<()> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    <A as ::tokio::io::AsyncSeek>::start_seek(
                        ::core::pin::Pin::new_unchecked(x),
                        pos,
                    )
                }
                Enum::B(x) => {
                    <B as ::tokio::io::AsyncSeek>::start_seek(
                        ::core::pin::Pin::new_unchecked(x),
                        pos,
                    )
                }
            }
        }
    }
    fn poll_complete(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::std::io::Result<u64>> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    <A as ::tokio::io::AsyncSeek>::poll_complete(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                    )
                }
                Enum::B(x) => {
                    <B as ::tokio::io::AsyncSeek>::poll_complete(
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
