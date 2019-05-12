## [`AsyncRead`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/io/trait.AsyncRead.html)

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
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
        buf: &mut [u8],
    ) -> ::core::task::Poll<::core::result::Result<usize, ::futures::io::Error>> {
        match ::core::pin::Pin::get_unchecked_mut(self) {
            Enum::A(x) => ::futures::io::AsyncRead::poll_read(::core::pin::Pin::new_unchecked(x), cx, buf),
            Enum::B(x) => ::futures::io::AsyncRead::poll_read(::core::pin::Pin::new_unchecked(x), cx, buf),
        }
    }

    #[inline]
    fn poll_read_vectored(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
        bufs: &mut [::std::io::IoSliceMut<'_>],
    ) -> ::core::task::Poll<::core::result::Result<usize, ::futures::io::Error>> {
        match ::core::pin::Pin::get_unchecked_mut(self) {
            Enum::A(x) => ::futures::io::AsyncRead::poll_read_vectored(::core::pin::Pin::new_unchecked(x), cx, bufs),
            Enum::B(x) => ::futures::io::AsyncRead::poll_read_vectored(::core::pin::Pin::new_unchecked(x), cx, bufs),
        }
    }
}
```
