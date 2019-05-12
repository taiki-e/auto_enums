<!-- TODO
## [`AsyncRead`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/io/trait.AsyncRead.html)

When deriving for enum like the following:

```rust
#[enum_derive(AsyncBufRead)]
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

impl<A, B> ::futures::io::AsyncBufRead for Enum<A, B>
where
    A: ::futures::io::AsyncBufRead,
    B: ::futures::io::AsyncBufRead,
{
    #[inline]
      fn poll_fill_buf<'__a>(
        self: Pin<&'__a mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::core::result::Result<&'__a [u8], ::futures::io::Error>>;
        match self {
            Enum::A(x) => ::futures::io::AsyncBufRead::poll_fill_buf(x, cx),
            Enum::B(x) => ::futures::io::AsyncBufRead::poll_fill_buf(x, cx),
        }
    }

    #[inline]
    fn consume(self: ::core::pin::Pin<&mut Self>, amt: usize) {
        match self {
            Enum::A(x) => ::futures::io::AsyncBufRead::consume(x, amt),
            Enum::B(x) => ::futures::io::AsyncBufRead::consume(x, amt),
        }
    }
}
``` -->
