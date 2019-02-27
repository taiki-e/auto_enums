## [`AsyncRead`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/io/trait.AsyncRead.html)

When deriving for enum like the following:

```rust
#[enum_derive(AsyncRead)]
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
impl<A, B> ::futures::io::AsyncRead for Enum<A, B>
where
    A: ::futures::io::AsyncRead,
    B: ::futures::io::AsyncRead,
{
    #[inline]
    unsafe fn initializer(&self) -> ::futures::io::Initializer {
        match self {
            Enum::A(x) => ::futures::io::AsyncRead::initializer(x),
            Enum::B(x) => ::futures::io::AsyncRead::initializer(x),
        }
    }

    #[inline]
    fn poll_read(
        &mut self,
        waker: &::core::task::Waker,
        buf: &mut [u8],
    ) -> ::core::task::Poll<::core::result::Result<usize, ::futures::io::Error>> {
        match self {
            Enum::A(x) => ::futures::io::AsyncRead::poll_read(x, waker, buf),
            Enum::B(x) => ::futures::io::AsyncRead::poll_read(x, waker, buf),
        }
    }

    #[inline]
    fn poll_vectored_read(
        &mut self,
        waker: &::core::task::Waker,
        vec: &mut [&mut ::futures::io::IoVec],
    ) -> ::core::task::Poll<::core::result::Result<usize, ::futures::io::Error>> {
        match self {
            Enum::A(x) => ::futures::io::AsyncRead::poll_vectored_read(x, waker, vec),
            Enum::B(x) => ::futures::io::AsyncRead::poll_vectored_read(x, waker, vec),
        }
    }
}
```
