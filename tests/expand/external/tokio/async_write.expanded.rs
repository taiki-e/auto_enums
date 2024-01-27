extern crate tokio1_crate as tokio;
use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B> ::tokio::io::AsyncWrite for Enum<A, B>
where
    A: ::tokio::io::AsyncWrite,
    B: ::tokio::io::AsyncWrite,
{
    fn is_write_vectored(&self) -> bool {
        match self {
            Enum::A(x) => <A as ::tokio::io::AsyncWrite>::is_write_vectored(x),
            Enum::B(x) => <B as ::tokio::io::AsyncWrite>::is_write_vectored(x),
        }
    }
    fn poll_write(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
        buf: &[u8],
    ) -> ::core::task::Poll<::std::io::Result<usize>> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    <A as ::tokio::io::AsyncWrite>::poll_write(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                        buf,
                    )
                }
                Enum::B(x) => {
                    <B as ::tokio::io::AsyncWrite>::poll_write(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                        buf,
                    )
                }
            }
        }
    }
    fn poll_flush(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::std::io::Result<()>> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    <A as ::tokio::io::AsyncWrite>::poll_flush(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                    )
                }
                Enum::B(x) => {
                    <B as ::tokio::io::AsyncWrite>::poll_flush(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                    )
                }
            }
        }
    }
    fn poll_shutdown(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::std::io::Result<()>> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    <A as ::tokio::io::AsyncWrite>::poll_shutdown(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                    )
                }
                Enum::B(x) => {
                    <B as ::tokio::io::AsyncWrite>::poll_shutdown(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                    )
                }
            }
        }
    }
    fn poll_write_vectored(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
        bufs: &[::std::io::IoSlice<'_>],
    ) -> ::core::task::Poll<::std::io::Result<usize>> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    <A as ::tokio::io::AsyncWrite>::poll_write_vectored(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                        bufs,
                    )
                }
                Enum::B(x) => {
                    <B as ::tokio::io::AsyncWrite>::poll_write_vectored(
                        ::core::pin::Pin::new_unchecked(x),
                        cx,
                        bufs,
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
