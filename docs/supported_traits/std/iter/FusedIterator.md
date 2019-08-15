## [`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html)

When deriving for enum like the following:

```rust
#[enum_derive(FusedIterator)]
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

impl<A, B> ::core::iter::FusedIterator for Enum<A, B>
where
    A: ::core::iter::FusedIterator,
    B: ::core::iter::FusedIterator<Item = <A as ::core::iter::Iterator>::Item>,
{}
```
