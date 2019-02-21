# auto_enums

[![Build Status](https://travis-ci.org/taiki-e/auto_enums.svg?branch=master)](https://travis-ci.org/taiki-e/auto_enums)
[![version](https://img.shields.io/crates/v/auto_enums.svg)](https://crates.io/crates/auto_enums/)
[![documentation](https://docs.rs/auto_enums/badge.svg)](https://docs.rs/auto_enums/)
[![license](https://img.shields.io/crates/l/auto_enums.svg)](https://crates.io/crates/auto_enums/)
[![Rustc Version](https://img.shields.io/badge/rustc-1.30+-lightgray.svg)](https://blog.rust-lang.org/2018/10/25/Rust-1.30.0.html)

A library for to allow multiple return types by automatically generated enum.

This library provides the following attribute macros:

* `#[auto_enum]`

  Parses syntax, creates the enum, inserts variants, and passes specified traits to `#[enum_derive]`.

* `#[enum_derive]`

  Implements specified traits to the enum.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
auto_enums = "0.4"
```

and this to your crate root:

```rust
#[macro_use]
extern crate auto_enums;
```

## Examples

`#[auto_enum]`'s basic feature is to wrap the value returned by the last if or match expression by an enum that implemented the specified traits.

```rust
#[auto_enum(Iterator)]
fn foo(x: i32) -> impl Iterator<Item = i32> {
    match x {
        0 => 1..10,
        _ => vec![5, 10].into_iter(),
    }
}
```

`#[auto_enum]` generates code in two stages.

First, `#[auto_enum]` will do the following.

* parses syntax
* creates the enum
* inserts variants

Code like this will be generated:

```rust
fn foo(x: i32) -> impl Iterator<Item = i32> {
    #[enum_derive(Iterator)]
    enum __Enum1<__T1, __T2> {
        __T1(__T1),
        __T2(__T2),
    }

    match x {
        0 => __Enum1::__T1(1..10),
        _ => __Enum1::__T2(vec![5, 10].into_iter()),
    }
}
```

Next, `#[enum_derive]` implements the specified traits.

[Code like this will be generated](docs/example-1.md)

See [API Documentation](https://docs.rs/auto_enums/) for more details.

## Supported traits

`#[enum_derive]` implements the supported traits and passes unsupported traits to `#[derive]`.

If you want to use traits that are not supported by `#[enum_derive]`, you can use another crate that provides `proc_macro_derive`, or you can define `proc_macro_derive` yourself ([derive_utils] probably can help it).

Basic usage of `#[enum_derive]`

```rust
// `#[enum_derive]` implements `Iterator`, and `#[derive]` implements `Clone`.
#[enum_derive(Iterator, Clone)]
enum Foo<A, B> {
    A(A),
    B(B),
}
```

[derive_utils]: https://crates.io/crates/derive_utils

### [std|core] libraries

Note that some traits have aliases.

`[std|core]::ops`

* [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html)
* [`DerefMut`](https://doc.rust-lang.org/std/ops/trait.DerefMut.html)
* [`Index`](https://doc.rust-lang.org/std/ops/trait.Index.html)
* [`IndexMut`](https://doc.rust-lang.org/std/ops/trait.IndexMut.html)
* [`RangeBounds`](https://doc.rust-lang.org/std/ops/trait.RangeBounds.html)
* [`Fn`](https://doc.rust-lang.org/std/ops/trait.Fn.html) (*nightly-only*)
* [`FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html) (*nightly-only*)
* [`FnOnce`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html) (*nightly-only*)
* [`Generator`](https://doc.rust-lang.org/nightly/std/ops/trait.Generator.html) (*nightly-only*)

`[std|core]::convert`

* [`AsRef`](https://doc.rust-lang.org/std/convert/trait.AsRef.html)
* [`AsMut`](https://doc.rust-lang.org/std/convert/trait.AsMut.html)

`[std|core]::iter`

* [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
* [`DoubleEndedIterator`](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html)
* [`ExactSizeIterator`](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html)
* [`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html)
* [`TrustedLen`](https://doc.rust-lang.org/std/iter/trait.TrustedLen.html) (*nightly-only*)
* [`Extend`](https://doc.rust-lang.org/std/iter/trait.Extend.html)

`[std|core]::fmt`

* [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) (alias: `fmt::Debug`) - [generated code](docs/supported_traits/std/debug.md)
* [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) (alias: `fmt::Display`)
* [`fmt::Binary`](https://doc.rust-lang.org/std/fmt/trait.Binary.html) (*requires `"fmt"` crate feature*)
* [`fmt::LowerExp`](https://doc.rust-lang.org/std/fmt/trait.LowerExp.html) (*requires `"fmt"` crate feature*)
* [`fmt::LowerHex`](https://doc.rust-lang.org/std/fmt/trait.LowerHex.html) (*requires `"fmt"` crate feature*)
* [`fmt::Octal`](https://doc.rust-lang.org/std/fmt/trait.Octal.html) (*requires `"fmt"` crate feature*)
* [`fmt::Pointer`](https://doc.rust-lang.org/std/fmt/trait.Pointer.html) (*requires `"fmt"` crate feature*)
* [`fmt::UpperExp`](https://doc.rust-lang.org/std/fmt/trait.UpperExp.html) (*requires `"fmt"` crate feature*)
* [`fmt::UpperHex`](https://doc.rust-lang.org/std/fmt/trait.UpperHex.html) (*requires `"fmt"` crate feature*)
* [`fmt::Write`](https://doc.rust-lang.org/std/fmt/trait.Write.html)

`[std|core]::future`

* [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html) - *nightly-only* - [generated code](docs/supported_traits/std/future.md)

`std::io`

* [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) (alias: `io::Read`)
* [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html) (alias: `io::BufRead`)
* [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) (alias: `io::Write`)
* [`Seek`](https://doc.rust-lang.org/std/io/trait.Seek.html) (alias: `io::Seek`)

`std::error`

* [`Error`](https://doc.rust-lang.org/std/error/trait.Error.html) - [generated code](docs/supported_traits/std/error.md)

### External libraries

You can add support for external library by activating the each crate feature.

[`futures(v0.3)`](https://github.com/rust-lang-nursery/futures-rs) (*requires `"futures"` crate feature*)

* [`futures::Stream`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.12/futures/stream/trait.Stream.html) - [generated code](docs/supported_traits/external/futures/stream.md)
* [`futures::Sink`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.12/futures/sink/trait.Sink.html) - [generated code](docs/supported_traits/external/futures/sink.md)
* [`futures::AsyncRead`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.12/futures/io/trait.AsyncRead.html) - [generated code](docs/supported_traits/external/futures/async_read.md)
* [`futures::AsyncWrite`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.12/futures/io/trait.AsyncWrite.html) - [generated code](docs/supported_traits/external/futures/async_write.md)

[`futures(v0.1)`](https://github.com/rust-lang-nursery/futures-rs) (*requires `"futures01"` crate feature*)

* [`futures01::Future`](https://docs.rs/futures/0.1/futures/future/trait.Future.html)
* [`futures01::Stream`](https://docs.rs/futures/0.1/futures/stream/trait.Stream.html)
* [`futures01::Sink`](https://docs.rs/futures/0.1/futures/sink/trait.Sink.html)

[`quote`](https://github.com/dtolnay/quote) (*requires `"proc_macro"` crate feature*)

* [`quote::ToTokens`](https://docs.rs/quote/0.6/quote/trait.ToTokens.html)

[`rayon`](https://github.com/rayon-rs/rayon) (*requires `"rayon"` crate feature*)

* [`rayon::ParallelIterator`](https://docs.rs/rayon/1.0/rayon/iter/trait.ParallelIterator.html)
* [`rayon::IndexedParallelIterator`](https://docs.rs/rayon/1.0/rayon/iter/trait.IndexedParallelIterator.html)
* [`rayon::ParallelExtend`](https://docs.rs/rayon/1.0/rayon/iter/trait.ParallelExtend.html)

[`serde`](https://github.com/serde-rs/serde) (*requires `"serde"` crate feature*)

* [`serde::Serialize`](https://docs.serde.rs/serde/trait.Serialize.html) - [generated code](docs/supported_traits/external/serde/serialize.md)

## Rust Version

The current minimum required Rust version is 1.30.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
