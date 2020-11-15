# [`Sink`](https://docs.rs/futures/0.3/futures/sink/trait.Sink.html)

When deriving for enum like the following:

```rust
#[enum_derive(Sink)]
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
impl<A, B> ::futures::sink::Sink<Item> for Enum<A, B>
where
    A: ::futures::sink::Sink<Item>,
    B: ::futures::sink::Sink<Item, Error = <A as ::futures::sink::Sink>::Error>,
{
    type Error = <A as ::futures::sink::Sink>::Error;

    #[inline]
    fn poll_ready(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::core::result::Result<(), Self::Error>> {
        unsafe {
            match ::core::pin::Pin::get_unchecked_mut(self) {
                Enum::A(x) => ::futures::sink::Sink::poll_ready(::core::pin::Pin::new_unchecked(x), cx),
                Enum::B(x) => ::futures::sink::Sink::poll_ready(::core::pin::Pin::new_unchecked(x), cx),
            }
        }
    }

    #[inline]
    fn start_send(
        self: ::core::pin::Pin<&mut Self>,
        item: Item,
    ) -> ::core::result::Result<(), Self::Error> {
        unsafe {
            match ::core::pin::Pin::get_unchecked_mut(self) {
                Enum::A(x) => ::futures::sink::Sink::start_send(::core::pin::Pin::new_unchecked(x), item),
                Enum::B(x) => ::futures::sink::Sink::start_send(::core::pin::Pin::new_unchecked(x), item),
            }
        }
    }

    #[inline]
    fn poll_flush(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::core::result::Result<(), Self::Error>> {
        unsafe {
            match ::core::pin::Pin::get_unchecked_mut(self) {
                Enum::A(x) => ::futures::sink::Sink::poll_flush(::core::pin::Pin::new_unchecked(x), cx),
                Enum::B(x) => ::futures::sink::Sink::poll_flush(::core::pin::Pin::new_unchecked(x), cx),
            }
        }
    }

    #[inline]
    fn poll_close(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::core::result::Result<(), Self::Error>> {
        unsafe {
            match ::core::pin::Pin::get_unchecked_mut(self) {
                Enum::A(x) => ::futures::sink::Sink::poll_close(::core::pin::Pin::new_unchecked(x), cx),
                Enum::B(x) => ::futures::sink::Sink::poll_close(::core::pin::Pin::new_unchecked(x), cx),
            }
        }
    }
}
```
