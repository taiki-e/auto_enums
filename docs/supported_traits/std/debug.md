## [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) (alias: `fmt::Debug`)

When deriving for enum like the following:

```rust
#[enum_derive(Debug)]
enum Enum<A, B> {
    A(A),
    B(B),
}
```

Code like this will be generated:

Note that it is a different implementation from `#[derive(Debug)]`.

```rust
enum Enum<A, B> {
    A(A),
    B(B),
}

impl<A, B> ::core::fmt::Debug for Enum<A, B>
where
    A: ::core::fmt::Debug,
    B: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Enum::A(x) => ::core::fmt::Debug::fmt(x, f),
            Enum::B(x) => ::core::fmt::Debug::fmt(x, f),
        }
    }
}
```
