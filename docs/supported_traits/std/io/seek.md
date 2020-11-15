# [`Seek`](https://doc.rust-lang.org/std/io/trait.Seek.html)

When deriving for enum like the following:

```rust
#[enum_derive(Seek)]
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

impl<A, B> ::std::io::Seek for Enum<A, B>
where
    A: ::std::io::Seek,
    B: ::std::io::Seek,
{
    #[inline]
    fn seek(&mut self, pos: ::std::io::SeekFrom) -> ::std::io::Result<u64> {
        match self {
            Enum::A(x) => ::std::io::Seek::seek(x, pos),
            Enum::B(x) => ::std::io::Seek::seek(x, pos),
        }
    }
}
```
