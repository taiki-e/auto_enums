## [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html)

When deriving for enum like the following:

```rust
#[enum_derive(Write)]
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

impl<A, B> ::std::io::Write for Enum<A, B>
where
    A: ::std::io::Write,
    B: ::std::io::Write,
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
        match self {
            Enum::A(x) => ::std::io::Write::write(x, buf),
            Enum::B(x) => ::std::io::Write::write(x, buf),
        }
    }

    #[inline]
    fn flush(&mut self) -> ::std::io::Result<()> {
        match self {
            Enum::A(x) => ::std::io::Write::flush(x),
            Enum::B(x) => ::std::io::Write::flush(x),
        }
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> ::std::io::Result<()> {
        match self {
            Enum::A(x) => ::std::io::Write::write_all(x, buf),
            Enum::B(x) => ::std::io::Write::write_all(x, buf),
        }
    }

    #[inline]
    fn write_fmt(&mut self, fmt: ::std::fmt::Arguments<'_>) -> ::std::io::Result<()> {
        match self {
            Enum::A(x) => ::std::io::Write::write_fmt(x, fmt),
            Enum::B(x) => ::std::io::Write::write_fmt(x, fmt),
        }
    }
}
```
