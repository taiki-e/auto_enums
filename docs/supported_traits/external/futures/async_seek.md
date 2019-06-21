## [`AsyncSeek`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/io/trait.AsyncSeek.html)

When deriving for enum like the following:

```rust
#[enum_derive(AsyncSeek)]
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
            match ::core::pin::Pin::get_unchecked_mut(self) {
                Enum::A(x) => ::futures::io::AsyncSeek::poll_seek(x, cx, pos),
                Enum::B(x) => ::futures::io::AsyncSeek::poll_seek(x, cx, pos),
            }
        }
    }
}
```
