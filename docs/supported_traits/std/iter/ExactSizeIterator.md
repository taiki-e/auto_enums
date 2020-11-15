# [`ExactSizeIterator`](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html)

When deriving for enum like the following:

```rust
#[enum_derive(ExactSizeIterator)]
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

impl<A, B> ::core::iter::ExactSizeIterator for Enum<A, B>
where
    A: ::core::iter::ExactSizeIterator,
    B: ::core::iter::ExactSizeIterator<Item = <A as ::core::iter::Iterator>::Item>,
{
    #[inline]
    fn len(&self) -> usize {
        match self {
            Enum::A(x) => ::core::iter::ExactSizeIterator::len(x),
            Enum::B(x) => ::core::iter::ExactSizeIterator::len(x),
        }
    }
}
```
