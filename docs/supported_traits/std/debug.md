# [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) (alias: `fmt::Debug`)

When deriving for enum like the following:

```rust
#[enum_derive(Debug)]
enum Enum<A, B> {
    A(A),
    B(B),
}
```

Code like this will be generated:

*If `std` crate feature is disabled, `::std` is replaced with `::core`.*

Note that it is a different implementation from `#[derive(Debug)]`.

```rust
enum Enum<A, B> {
    A(A),
    B(B),
}

impl<A, B> ::std::fmt::Debug for Enum<A, B>
where
    A: ::std::fmt::Debug,
    B: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Enum::A(x) => ::std::fmt::Debug::fmt(x, f),
            Enum::B(x) => ::std::fmt::Debug::fmt(x, f),
        }
    }
}
```
