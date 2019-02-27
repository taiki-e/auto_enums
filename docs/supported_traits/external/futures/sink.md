## [`Sink`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/sink/trait.Sink.html)

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
impl<A, B> ::futures::sink::Sink for Enum<A, B>
where
    A: ::futures::sink::Sink,
    B: ::futures::sink::Sink<SinkItem = <A as ::futures::sink::Sink>::SinkItem, SinkError = <A as ::futures::sink::Sink>::SinkError>,
{
    type SinkItem = <A as ::futures::sink::Sink>::SinkItem;
    type SinkError = <A as ::futures::sink::Sink>::SinkError;

    #[inline]
    fn poll_ready(
        self: ::core::pin::Pin<&mut Self>,
        waker: &::core::task::Waker,
    ) -> ::core::task::Poll<::core::result::Result<(), Self::SinkError>> {
        unsafe {
            match ::core::pin::Pin::get_unchecked_mut(self) {
                Enum::A(x) => ::futures::sink::Sink::poll_ready(::core::pin::Pin::new_unchecked(x), waker),
                Enum::B(x) => ::futures::sink::Sink::poll_ready(::core::pin::Pin::new_unchecked(x), waker),
            }
        }
    }

    #[inline]
    fn start_send(
        self: ::core::pin::Pin<&mut Self>,
        item: Self::SinkItem,
    ) -> ::core::result::Result<(), Self::SinkError> {
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
        waker: &::core::task::Waker,
    ) -> ::core::task::Poll<::core::result::Result<(), Self::SinkError>> {
        unsafe {
            match ::core::pin::Pin::get_unchecked_mut(self) {
                Enum::A(x) => ::futures::sink::Sink::poll_flush(::core::pin::Pin::new_unchecked(x), waker),
                Enum::B(x) => ::futures::sink::Sink::poll_flush(::core::pin::Pin::new_unchecked(x), waker),
            }
        }
    }

    #[inline]
    fn poll_close(
        self: ::core::pin::Pin<&mut Self>,
        waker: &::core::task::Waker,
    ) -> ::core::task::Poll<::core::result::Result<(), Self::SinkError>> {
        unsafe {
            match ::core::pin::Pin::get_unchecked_mut(self) {
                Enum::A(x) => ::futures::sink::Sink::poll_close(::core::pin::Pin::new_unchecked(x), waker),
                Enum::B(x) => ::futures::sink::Sink::poll_close(::core::pin::Pin::new_unchecked(x), waker),
            }
        }
    }
}
```
