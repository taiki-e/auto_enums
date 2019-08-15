## [`DoubleEndedIterator`](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html)

When deriving for enum like the following:

```rust
#[enum_derive(DoubleEndedIterator)]
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

impl<A, B> ::core::iter::DoubleEndedIterator for Enum<A, B>
where
    A: ::core::iter::DoubleEndedIterator,
    B: ::core::iter::DoubleEndedIterator<Item = <A as ::core::iter::Iterator>::Item>,
{
    #[inline]
    fn next_back(&mut self) -> ::core::option::Option<Self::Item> {
        match self {
            Enum::A(x) => ::core::iter::DoubleEndedIterator::next_back(x),
            Enum::B(x) => ::core::iter::DoubleEndedIterator::next_back(x),
        }
    }
}
```
