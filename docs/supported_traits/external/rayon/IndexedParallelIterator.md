## [`rayon::IndexedParallelIterator`](https://docs.rs/rayon/1/rayon/iter/trait.IndexedParallelIterator.html)

When deriving for enum like the following:

```rust
#[enum_derive(rayon::IndexedParallelIterator)]
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

impl<A, B> ::rayon::iter::IndexedParallelIterator for Enum<A, B>
where
    A: ::rayon::iter::IndexedParallelIterator,
    B: ::rayon::iter::IndexedParallelIterator<Item = <A as ::rayon::iter::ParallelIterator>::Item>,
{
    #[inline]
    fn drive<__C>(self, consumer: __C) -> __C::Result
    where
        __C: ::rayon::iter::plumbing::Consumer<Self::Item>,
    {
        match self {
            Enum::A(x) => ::rayon::iter::IndexedParallelIterator::drive(x, consumer),
            Enum::B(x) => ::rayon::iter::IndexedParallelIterator::drive(x, consumer),
        }
    }

    #[inline]
    fn len(&self) -> usize {
        match self {
            Enum::A(x) => ::rayon::iter::IndexedParallelIterator::len(x),
            Enum::B(x) => ::rayon::iter::IndexedParallelIterator::len(x),
        }
    }

    #[inline]
    fn with_producer<__CB>(self, callback: __CB) -> __CB::Output
    where
        __CB: ::rayon::iter::plumbing::ProducerCallback<Self::Item>,
    {
        match self {
            Enum::A(x) => ::rayon::iter::IndexedParallelIterator::opt_len(x),
            Enum::B(x) => ::rayon::iter::IndexedParallelIterator::opt_len(x),
        }
    }
}
```
