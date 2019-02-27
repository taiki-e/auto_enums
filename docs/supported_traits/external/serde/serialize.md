## [`serde::Serialize`](https://docs.serde.rs/serde/trait.Serialize.html)

When deriving for enum like the following:

```rust
#[enum_derive(serde::Serialize)]
enum Enum<A, B> {
    A(A),
    B(B),
}
```

Code like this will be generated:

Note that it is a different implementation from `#[derive(Serialize)]`.

```rust
enum Enum<A, B> {
    A(A),
    B(B),
}

impl<A, B> ::serde::ser::Serialize for Enum<A, B>
where
    A: ::serde::ser::Serialize,
    B: ::serde::ser::Serialize,
{
    #[inline]
    fn serialize<__S>(&self, serializer: __S) -> ::std::result::Result<__S::Ok, __S::Error>
    where
        __S: ::serde::ser::Serializer
    {
        match self {
            Enum::A(x) => ::serde::ser::Serialize::serialize(x, serializer),
            Enum::B(x) => ::serde::ser::Serialize::serialize(x, serializer),
        }
    }
}
```
