## [`Stream`](https://docs.rs/futures/0.3/futures/stream/trait.Stream.html)

When deriving for enum like the following:

```rust
#[enum_derive(Stream)]
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
impl<A, B> ::futures::stream::Stream for Enum<A, B>
where
    A: ::futures::stream::Stream,
    B: ::futures::stream::Stream<Item = <A as ::futures::stream::Stream>::Item>,
{
    type Item = <A as ::futures::stream::Stream>::Item;

    fn poll_next(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::core::option::Option<Self::Item>> {
        unsafe {
            match ::core::pin::Pin::get_unchecked_mut(self) {
                Enum::A(x) => ::futures::stream::Stream::poll_next(::core::pin::Pin::new_unchecked(x), cx),
                Enum::B(x) => ::futures::stream::Stream::poll_next(::core::pin::Pin::new_unchecked(x), cx),
            }
        }
    }

    fn size_hint(&self) -> (usize, ::core::option::Option<usize>) {
        match self {
            Enum::A(x) => ::futures::stream::Stream::size_hint(x),
            Enum::B(x) => ::futures::stream::Stream::size_hint(x),
        }
    }
}
```
