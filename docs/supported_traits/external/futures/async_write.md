## [`AsyncWrite`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/io/trait.AsyncWrite.html)

When deriving for enum like the following:

```rust
#[enum_derive(AsyncWrite)]
enum Enum<A, B> {
    A(A),
    B(B),
}
```

Code like this will be generated:

```rust
enum Enum<A, B> {
    A(A),
    B(B),
}

#[allow(unsafe_code)]
impl<A, B> ::futures::io::AsyncWrite for Enum<A, B>
where
    A: ::futures::io::AsyncWrite,
    B: ::futures::io::AsyncWrite,
{
    #[inline]
    fn poll_write(
        &mut self,
        waker: &::core::task::Waker,
        buf: &[u8],
    ) -> ::core::task::Poll<::core::result::Result<usize, ::futures::io::Error>> {
        match self {
            Enum::A(x) => ::futures::io::AsyncWrite::poll_write(x, waker, buf),
            Enum::B(x) => ::futures::io::AsyncWrite::poll_write(x, waker, buf),
        }
    }

    #[inline]
    fn poll_vectored_write(
        &mut self,
        waker: &::core::task::Waker,
        vec: &[&::futures::io::IoVec],
    ) -> ::core::task::Poll<::core::result::Result<usize, ::futures::io::Error>> {
        match self {
            Enum::A(x) => ::futures::io::AsyncWrite::poll_vectored_write(x, waker, vec),
            Enum::B(x) => ::futures::io::AsyncWrite::poll_vectored_write(x, waker, vec),
        }
    }

    #[inline]
    fn poll_flush(
        &mut self,
        waker: &::core::task::Waker,
    ) -> ::core::task::Poll<::core::result::Result<(), ::futures::io::Error>> {
        match self {
            Enum::A(x) => ::futures::io::AsyncWrite::poll_flush(x, waker),
            Enum::B(x) => ::futures::io::AsyncWrite::poll_flush(x, waker),
        }
    }

    #[inline]
    fn poll_close(
        &mut self,
        waker: &::core::task::Waker,
    ) -> ::core::task::Poll<::core::result::Result<(), ::futures::io::Error>> {
        match self {
            Enum::A(x) => ::futures::io::AsyncWrite::poll_close(x, waker),
            Enum::B(x) => ::futures::io::AsyncWrite::poll_close(x, waker),
        }
    }
}
```
