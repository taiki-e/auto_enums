## [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html)

When deriving for enum like the following:

```rust
#[enum_derive(BufRead)]
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

impl<A, B> ::std::io::BufRead for Enum<A, B>
where
    A: ::std::io::BufRead,
    B: ::std::io::BufRead,
{
    fn fill_buf(&mut self) -> ::std::io::Result<&[u8]> {
        match self {
            Enum::A(x) => ::std::io::BufRead::fill_buf(x),
            Enum::B(x) => ::std::io::BufRead::fill_buf(x),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            Enum::A(x) => ::std::io::BufRead::consume(x, amt),
            Enum::B(x) => ::std::io::BufRead::consume(x, amt),
        }
    }

    fn read_until(&mut self, byte: u8, buf: &mut ::std::vec::Vec<u8>) -> ::std::io::Result<usize> {
        match self {
            Enum::A(x) => ::std::io::BufRead::read_until(x, byte, buf),
            Enum::B(x) => ::std::io::BufRead::read_until(x, byte, buf),
        }
    }

    fn read_line(&mut self, buf: &mut ::std::string::String) -> ::std::io::Result<usize> {
        match self {
            Enum::A(x) => ::std::io::BufRead::read_line(x, buf),
            Enum::B(x) => ::std::io::BufRead::read_line(x, buf),
        }
    }
}
```
