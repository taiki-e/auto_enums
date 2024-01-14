# auto_enums

[![crates.io](https://img.shields.io/crates/v/auto_enums?style=flat-square&logo=rust)](https://crates.io/crates/auto_enums)
[![docs.rs](https://img.shields.io/badge/docs.rs-auto__enums-blue?style=flat-square&logo=docs.rs)](https://docs.rs/auto_enums)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![rust version](https://img.shields.io/badge/rustc-1.56+-blue?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![github actions](https://img.shields.io/github/actions/workflow/status/taiki-e/auto_enums/ci.yml?branch=main&style=flat-square&logo=github)](https://github.com/taiki-e/auto_enums/actions)

A library for to allow multiple return types by automatically generated enum.

This crate is a procedural macro implementation of the features discussions
in [rust-lang/rfcs#2414]. This idea is also known as
["Anonymous sum types"][rust-lang/rfcs#294].

This library provides the following attribute macros:

- `#[auto_enum]`

  Parses syntax, creates the enum, inserts variants, and passes specified
  traits to `#[enum_derive]`.

- `#[enum_derive]`

  Implements specified traits to the enum.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
auto_enums = "0.8"
```

*Compiler support: requires rustc 1.56+*

## Examples

`#[auto_enum]`'s basic feature is to wrap the value returned by the obvious
branches (`match`, `if`, `return`, etc..) by an enum that implemented the
specified traits.

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

- parses syntax
- creates the enum
- inserts variants

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

[Code like this will be generated](tests/expand/enum_derive/example-1.expanded.rs)

`#[auto_enum]` can also parse nested arms/branches by using the `#[nested]`
attribute.

```rust
use auto_enums::auto_enum;

#[auto_enum(Iterator)]
fn foo(x: i32) -> impl Iterator<Item = i32> {
    match x {
        0 => 1..10,
        #[nested]
        _ => match x {
            1 => vec![5, 10].into_iter(),
            _ => 0..=x,
        },
    }
}
```

See [documentation](https://docs.rs/auto_enums) for more details.

## Supported traits

`#[enum_derive]` implements the supported traits and passes unsupported
traits to `#[derive]`.

`#[enum_derive]` supports many of the standard library traits and some popular
third-party libraries traits such as [rayon], [futures][futures03],
[tokio][tokio1], [http_body][http_body1]. **See [documentation](https://docs.rs/auto_enums/latest/auto_enums/#supported-traits) for a complete list of supported traits.**

If you want to use traits that are not supported by `#[enum_derive]`, you
can use another crate that provides [derives macros][proc-macro-derive], or
you can define derives macros yourself ([derive_utils] probably can help it).

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

## Optional features

- **`std`** *(enabled by default)*
  - Enable to use `std` library's traits.
- **`ops`**
  - Enable to use `[std|core]::ops`'s `Deref`, `DerefMut`, `Index`, `IndexMut`, and `RangeBounds` traits.
- **`convert`**
  - Enable to use `[std|core]::convert`'s `AsRef` and `AsMut` traits.
- **`fmt`**
  - Enable to use `[std|core]::fmt`'s traits other than `Debug`, `Display` and `Write`.
- **`transpose_methods`**
  - Enable to use `transpose*` methods.
- **`futures03`**
  - Enable to use [futures v0.3][futures03] traits.
- **`futures01`**
  - Enable to use [futures v0.1][futures01] traits.
- **`rayon`**
  - Enable to use [rayon] traits.
- **`serde`**
  - Enable to use [serde] traits.
- **`tokio1`**
  - Enable to use [tokio v1][tokio1] traits.
- **`tokio03`**
  - Enable to use [tokio v0.3][tokio03] traits.
- **`tokio02`**
  - Enable to use [tokio v0.2][tokio02] traits.
- **`tokio01`**
  - Enable to use [tokio v0.1][tokio01] traits.
- **`http_body1`**
  - Enable to use [http_body v1][http_body1] traits.
- **`coroutine_trait`**
  - Enable to use `[std|core]::ops::Coroutine` trait.
  - Note that this feature is unstable and may cause incompatible changes between patch versions.
- **`fn_traits`**
  - Enable to use `[std|core]::ops`'s `Fn`, `FnMut`, and `FnOnce` traits.
  - Note that this feature is unstable and may cause incompatible changes between patch versions.
- **`trusted_len`**
  - Enable to use `[std|core]::iter::TrustedLen` trait.
  - Note that this feature is unstable and may cause incompatible changes between patch versions.

### `type_analysis` feature

Analyze return type of function and `let` binding.

*Note that this feature is still experimental.*

Examples:

```rust
use auto_enums::auto_enum;

#[auto_enum] // there is no need to specify std library's traits
fn func1(x: i32) -> impl Iterator<Item = i32> {
    match x {
        0 => 1..10,
        _ => vec![5, 10].into_iter(),
    }
}

#[auto_enum]
fn func2(x: i32) {
    // Unlike `feature(impl_trait_in_bindings)`, this works on stable compilers.
    #[auto_enum]
    let iter: impl Iterator<Item = i32> = match x {
        0 => Some(0).into_iter(),
        _ => 0..x,
    };
}
```

Please be careful if you return another traits with the same name.

## Related Projects

- [derive_utils]: A procedural macro helper for easily writing [derives macros][proc-macro-derive] for enums.
- [io-enum]: \#\[derive(Read, Write, Seek, BufRead)\] for enums.
- [iter-enum]: \#\[derive(Iterator, DoubleEndedIterator, ExactSizeIterator, Extend)\] for enums.

[derive_utils]: https://github.com/taiki-e/derive_utils
[futures01]: https://docs.rs/futures/0.1
[futures03]: https://docs.rs/futures/0.3
[io-enum]: https://github.com/taiki-e/io-enum
[iter-enum]: https://github.com/taiki-e/iter-enum
[proc-macro-derive]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros
[rayon]: https://docs.rs/rayon/1
[rust-lang/rfcs#294]: https://github.com/rust-lang/rfcs/issues/294
[rust-lang/rfcs#2414]: https://github.com/rust-lang/rfcs/issues/2414
[serde]: https://docs.rs/serde/1
[tokio01]: https://docs.rs/tokio/0.1
[tokio02]: https://docs.rs/tokio/0.2
[tokio03]: https://docs.rs/tokio/0.3
[tokio1]: https://docs.rs/tokio/1
[http_body1]: https://docs.rs/http_body/1

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
