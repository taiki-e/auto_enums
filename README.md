# auto_enums

[![Build Status][azure-badge]][azure-url]
[![Crates.io][crates-version-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![License][crates-license-badge]][crates-url]
[![Minimum supported Rust version][rustc-badge]][rustc-url]

[azure-badge]: https://dev.azure.com/taiki-e/taiki-e/_apis/build/status/taiki-e.auto_enums?branchName=master
[azure-url]: https://dev.azure.com/taiki-e/taiki-e/_build/latest?definitionId=3&branchName=master
[crates-version-badge]: https://img.shields.io/crates/v/auto_enums.svg
[crates-license-badge]: https://img.shields.io/crates/l/auto_enums.svg
[crates-badge]: https://img.shields.io/crates/v/auto_enums.svg
[crates-url]: https://crates.io/crates/auto_enums/
[docs-badge]: https://docs.rs/auto_enums/badge.svg
[docs-url]: https://docs.rs/auto_enums/
[rustc-badge]: https://img.shields.io/badge/rustc-1.31+-lightgray.svg
[rustc-url]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html

A library for to allow multiple return types by automatically generated enum.

This crate is a procedural macro implementation of the features discussions in <https://github.com/rust-lang/rfcs/issues/2414>.

This library provides the following attribute macros:

* `#[auto_enum]`

  Parses syntax, creates the enum, inserts variants, and passes specified traits to `#[enum_derive]`.

* `#[enum_derive]`

  Implements specified traits to the enum.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
auto_enums = "0.6.0-alpha.3"
```

The current auto_enums requires Rust 1.31 or later.

## Examples

`#[auto_enum]`'s basic feature is to wrap the value returned by the obvious branches (`match`, `if`, `return`, etc..) by an enum that implemented the specified traits.

```rust
use auto_enums::auto_enum;

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
    #[::auto_enums::enum_derive(Iterator)]
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
use auto_enums::enum_derive;

// `#[enum_derive]` implements `Iterator`, and `#[derive]` implements `Clone`.
#[enum_derive(Iterator, Clone)]
enum Foo<A, B> {
    A(A),
    B(B),
}
```

[derive_utils]: https://github.com/taiki-e/derive_utils

### [std|core] libraries

Some traits support is disabled by default.
Note that some traits have aliases.

*When using features that depend on unstable APIs, the `"unstable"` feature must be explicitly enabled*

`[std|core]::iter`

* [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) - [generated code](docs/supported_traits/std/iter/Iterator.md)
* [`DoubleEndedIterator`](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html) - [generated code](docs/supported_traits/std/iter/DoubleEndedIterator.md)
* [`ExactSizeIterator`](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html) - [generated code](docs/supported_traits/std/iter/ExactSizeIterator.md)
* [`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html) - [generated code](docs/supported_traits/std/iter/FusedIterator.md)
* [`Extend`](https://doc.rust-lang.org/std/iter/trait.Extend.html) - [generated code](docs/supported_traits/std/iter/Extend.md)
* [`TrustedLen`](https://doc.rust-lang.org/std/iter/trait.TrustedLen.html) - [generated code](docs/supported_traits/std/iter/TrustedLen.md) *(requires `"trusted_len"` and `"unstable"` crate features)*

`[std|core]::future`

* [`Future`](https://doc.rust-lang.org/nightly/std/future/trait.Future.html) - [generated code](docs/supported_traits/std/Future.md)

`std::io`

* [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) (alias: `io::Read`) - [generated code](docs/supported_traits/std/io/Read.md)
* [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html) (alias: `io::BufRead`) - [generated code](docs/supported_traits/std/io/BufRead.md)
* [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) (alias: `io::Write`) - [generated code](docs/supported_traits/std/io/Write.md)
* [`Seek`](https://doc.rust-lang.org/std/io/trait.Seek.html) (alias: `io::Seek`) - [generated code](docs/supported_traits/std/io/Seek.md)

`[std|core]::ops`

* [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html) *(requires `"ops"` crate feature)*
* [`DerefMut`](https://doc.rust-lang.org/std/ops/trait.DerefMut.html) *(requires `"ops"` crate feature)*
* [`Index`](https://doc.rust-lang.org/std/ops/trait.Index.html) *(requires `"ops"` crate feature)*
* [`IndexMut`](https://doc.rust-lang.org/std/ops/trait.IndexMut.html) *(requires `"ops"` crate feature)*
* [`RangeBounds`](https://doc.rust-lang.org/std/ops/trait.RangeBounds.html) *(requires `"ops"` crate feature)*
* [`Fn`](https://doc.rust-lang.org/std/ops/trait.Fn.html) *(requires `"fn_traits"` and `"unstable"` crate features)*
* [`FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html) *(requires `"fn_traits"` and `"unstable"` crate features)*
* [`FnOnce`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html) *(requires `"fn_traits"` and `"unstable"` crate features)*
* [`Generator`](https://doc.rust-lang.org/nightly/std/ops/trait.Generator.html) *(requires `"generator_trait"` and `"unstable"` crate features)*

`[std|core]::convert`

* [`AsRef`](https://doc.rust-lang.org/std/convert/trait.AsRef.html) *(requires `"convert"` crate feature)*
* [`AsMut`](https://doc.rust-lang.org/std/convert/trait.AsMut.html) *(requires `"convert"` crate feature)*

`[std|core]::fmt`

* [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) (alias: `fmt::Debug`) - [generated code](docs/supported_traits/std/Debug.md)
* [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) (alias: `fmt::Display`)
* [`fmt::Binary`](https://doc.rust-lang.org/std/fmt/trait.Binary.html) *(requires `"fmt"` crate feature)*
* [`fmt::LowerExp`](https://doc.rust-lang.org/std/fmt/trait.LowerExp.html) *(requires `"fmt"` crate feature)*
* [`fmt::LowerHex`](https://doc.rust-lang.org/std/fmt/trait.LowerHex.html) *(requires `"fmt"` crate feature)*
* [`fmt::Octal`](https://doc.rust-lang.org/std/fmt/trait.Octal.html) *(requires `"fmt"` crate feature)*
* [`fmt::Pointer`](https://doc.rust-lang.org/std/fmt/trait.Pointer.html) *(requires `"fmt"` crate feature)*
* [`fmt::UpperExp`](https://doc.rust-lang.org/std/fmt/trait.UpperExp.html) *(requires `"fmt"` crate feature)*
* [`fmt::UpperHex`](https://doc.rust-lang.org/std/fmt/trait.UpperHex.html) *(requires `"fmt"` crate feature)*
* [`fmt::Write`](https://doc.rust-lang.org/std/fmt/trait.Write.html)

`std::error`

* [`Error`](https://doc.rust-lang.org/std/error/trait.Error.html) - [generated code](docs/supported_traits/std/Error.md)

### External libraries

You can add support for external library by activating the each crate feature.

[`futures(v0.3)`](https://github.com/rust-lang-nursery/futures-rs) *(requires `"futures"` and `"unstable"` crate features)*

* [`futures::Stream`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/stream/trait.Stream.html) - [generated code](docs/supported_traits/external/futures/Stream.md)
* [`futures::Sink`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/sink/trait.Sink.html) - [generated code](docs/supported_traits/external/futures/Sink.md)
* [`futures::AsyncRead`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/io/trait.AsyncRead.html) - [generated code](docs/supported_traits/external/futures/AsyncRead.md)
* [`futures::AsyncWrite`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/io/trait.AsyncWrite.html) - [generated code](docs/supported_traits/external/futures/AsyncWrite.md)
* [`futures::AsyncSeek`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/io/trait.AsyncSeek.html) - [generated code](docs/supported_traits/external/futures/AsyncSeek.md)
* [`futures::AsyncBufRead`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/io/trait.AsyncBufRead.html) - [generated code](docs/supported_traits/external/futures/AsyncBufRead.md)

[`futures(v0.1)`](https://github.com/rust-lang-nursery/futures-rs) *(requires `"futures01"` crate feature)*

* [`futures01::Future`](https://docs.rs/futures/0.1/futures/future/trait.Future.html)
* [`futures01::Stream`](https://docs.rs/futures/0.1/futures/stream/trait.Stream.html)
* [`futures01::Sink`](https://docs.rs/futures/0.1/futures/sink/trait.Sink.html)

[`rayon`](https://github.com/rayon-rs/rayon) *(requires `"rayon"` crate feature)*

* [`rayon::ParallelIterator`](https://docs.rs/rayon/1.0/rayon/iter/trait.ParallelIterator.html) - [generated code](docs/supported_traits/external/rayon/ParallelIterator.md)
* [`rayon::IndexedParallelIterator`](https://docs.rs/rayon/1.0/rayon/iter/trait.IndexedParallelIterator.html) - [generated code](docs/supported_traits/external/rayon/IndexedParallelIterator.md)
* [`rayon::ParallelExtend`](https://docs.rs/rayon/1.0/rayon/iter/trait.ParallelExtend.html) - [generated code](docs/supported_traits/external/rayon/ParallelExtend.md)

[`serde`](https://github.com/serde-rs/serde) *(requires `"serde"` crate feature)*

* [`serde::Serialize`](https://docs.serde.rs/serde/trait.Serialize.html) - [generated code](docs/supported_traits/external/serde/Serialize.md)

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
