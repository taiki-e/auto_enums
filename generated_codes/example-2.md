### Original code:

```rust
use std::{fs, io, path::Path};

#[auto_enum]
fn output_stream(file: Option<&Path>) -> io::Result<impl io::Write> {
    #[auto_enum(io::Write)]
    let writer = match file {
        Some(f) => fs::File::create(f)?,
        None => io::stdout(),
    };

    Ok(writer)
}
```

### Generated code:

```rust
use std::{fs, io, path::Path};

fn output_stream(file: Option<&Path>) -> io::Result<impl io::Write> {
    let writer = {
        enum __Enum1<__T1, __T2> {
            __T1(__T1),
            __T2(__T2),
        }

        impl<__T1, __T2> ::std::io::Write for __Enum1<__T1, __T2>
        where
            __T1: ::std::io::Write,
            __T2: ::std::io::Write,
        {
            #[inline]
            fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
                match self {
                    __Enum1::__T1(x) => x.write(buf),
                    __Enum1::__T2(x) => x.write(buf),
                }
            }
            #[inline]
            fn flush(&mut self) -> ::std::io::Result<()> {
                match self {
                    __Enum1::__T1(x) => x.flush(),
                    __Enum1::__T2(x) => x.flush(),
                }
            }
            #[inline]
            fn write_all(&mut self, buf: &[u8]) -> ::std::io::Result<()> {
                match self {
                    __Enum1::__T1(x) => x.write_all(buf),
                    __Enum1::__T2(x) => x.write_all(buf),
                }
            }
            #[inline]
            fn write_fmt(&mut self, fmt: ::std::fmt::Arguments) -> ::std::Result<()> {
                match self {
                    __Enum1::__T1(x) => x.write_fmt(fmt),
                    __Enum1::__T2(x) => x.write_fmt(fmt),
                }
            }
        }

        match file {
            Some(f) => __Enum1::__T1(fs::File::create(f)?),
            None => __Enum1::__T2(io::stdout()),
        }
    };

    Ok(writer)
}
```
