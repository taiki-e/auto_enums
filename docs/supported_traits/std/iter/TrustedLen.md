# [`TrustedLen`](https://doc.rust-lang.org/std/iter/trait.TrustedLen.html)

When deriving for enum like the following:

```rust
#[enum_derive(TrustedLen)]
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

impl<A, B> ::core::iter::TrustedLen for Enum<A, B>
where
    A: ::core::iter::TrustedLen,
    B: ::core::iter::TrustedLen<Item = <A as ::core::iter::Iterator>::Item>,
{}
```
