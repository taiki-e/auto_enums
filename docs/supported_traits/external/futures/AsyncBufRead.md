## [`AsyncBufRead`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/io/trait.AsyncBufRead.html)

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

#[allow(unsafe_code)]
impl<A, B> ::futures::io::AsyncBufRead for Enum<A, B>
where
    A: ::futures::io::AsyncBufRead,
    B: ::futures::io::AsyncBufRead,
{
    #[inline]
      fn poll_fill_buf<'__a>(
        self: ::core::pin::Pin<&'__a mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::std::io::Result<&'__a [u8]>>;
        match ::core::pin::Pin::get_unchecked_mut(self) {
            Enum::A(x) => ::futures::io::AsyncBufRead::poll_fill_buf(::core::pin::Pin::new_unchecked(x), cx),
            Enum::B(x) => ::futures::io::AsyncBufRead::poll_fill_buf(::core::pin::Pin::new_unchecked(x), cx),
        }
    }

    #[inline]
    fn consume(self: ::core::pin::Pin<&mut Self>, amt: usize) {
        match ::core::pin::Pin::get_unchecked_mut(self) {
            Enum::A(x) => ::futures::io::AsyncBufRead::consume(::core::pin::Pin::new_unchecked(x), amt),
            Enum::B(x) => ::futures::io::AsyncBufRead::consume(::core::pin::Pin::new_unchecked(x), amt),
        }
    }
}
```
