## [`Future`](https://doc.rust-lang.org/nightly/std/future/trait.Future.html)

When deriving for enum like the following:

```rust
#[enum_derive(Future)]
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
impl<A, B> ::core::future::Future for Enum<A, B>
where
    A: ::core::future::Future,
    B: ::core::future::Future<Output = <A as ::core::future::Future>::Output>,
{
    type Output = <A as ::core::future::Future>::Output;

    #[inline]
    fn poll(
        self: ::core::pin::Pin<&mut Self>,
        waker: &::core::task::Waker
    ) -> ::core::task::Poll<Self::Output> {
        unsafe {
            match ::core::pin::Pin::get_unchecked_mut(self) {
                Enum::A(x) => ::core::future::Future::poll(::core::pin::Pin::new_unchecked(x), waker),
                Enum::B(x) => ::core::future::Future::poll(::core::pin::Pin::new_unchecked(x), waker),
            }
        }
    }
}
```
