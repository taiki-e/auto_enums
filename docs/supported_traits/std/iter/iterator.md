## [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)

When deriving for enum like the following:

```rust
#[enum_derive(Iterator)]
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

impl<A, B> ::core::iter::Iterator for Enum<A, B>
where
    A: ::core::iter::Iterator,
    B: ::core::iter::Iterator<Item = <A as ::core::iter::Iterator>::Item>,
{
    type Item = <A as ::core::iter::Iterator>::Item;

    #[inline]
    fn next(&mut self) -> ::core::option::Option<Self::Item> {
        match self {
            Enum::A(x) => ::core::iter::Iterator::next(x),
            Enum::B(x) => ::core::iter::Iterator::next(x),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, ::core::option::Option<usize>) {
        match self {
            Enum::A(x) => ::core::iter::Iterator::size_hint(x),
            Enum::B(x) => ::core::iter::Iterator::size_hint(x),
        }
    }

    #[inline]
    fn count(self) -> usize;
        match self {
            Enum::A(x) => ::core::iter::Iterator::count(x),
            Enum::B(x) => ::core::iter::Iterator::count(x),
        }
    }

    #[inline]
    fn last(self) -> ::core::option::Option<Self::Item> {
        match self {
            Enum::A(x) => ::core::iter::Iterator::last(x, fmt),
            Enum::B(x) => ::core::iter::Iterator::last(x, fmt),
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> ::core::option::Option<Self::Item> {
        match self {
            Enum::A(x) => ::core::iter::Iterator::nth(x, n),
            Enum::B(x) => ::core::iter::Iterator::nth(x, n),
        }
    }

    #[inline]
    #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
    fn collect<__U: ::core::iter::FromIterator<Self::Item>>(self) -> __U {
        match self {
            Enum::A(x) => ::core::iter::Iterator::collect(x),
            Enum::B(x) => ::core::iter::Iterator::collect(x),
        }
    }

    #[inline]
    fn partition<__U, __F>(self, f: __F) -> (__U, __U)
    where
        __U: ::core::default::Default + ::core::iter::Extend<Self::Item>,
        __F: ::core::ops::FnMut(&Self::Item) -> bool
    {
        match self {
            Enum::A(x) => ::core::iter::Iterator::partition(x, f),
            Enum::B(x) => ::core::iter::Iterator::partition(x, f),
        }
    }
}
```
