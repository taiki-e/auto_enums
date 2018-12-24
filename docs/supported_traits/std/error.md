# [`Error`](https://doc.rust-lang.org/std/error/trait.Error.html)

When deriving for enum like the following:

```rust
#[enum_derive(Error)]
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

impl<A, B> ::std::error::Error for Enum<A, B>
where
    A: ::std::error::Error + 'static,
    B: ::std::error::Error + 'static,
{
    fn description(&self) -> &str {
        match self {
            Enum::A(x) => ::std::error::Error::description(x),
            Enum::B(x) => ::std::error::Error::description(x),
        }
    }
    fn source(&self) -> ::std::option::Option<&(dyn (::std::error::Error) + 'static)> {
        match self {
            Enum::A(x) => ::std::option::Option::Some(x),
            Enum::B(x) => ::std::option::Option::Some(x),
        }
    }
}
```
