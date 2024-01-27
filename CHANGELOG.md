# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

## [0.8.5] - 2024-01-27

- Update `derive_utils` to 0.14. This fixes "overflow evaluating the requirement" error with `#[enum_derive]` in two situations.

## [0.8.4] - 2024-01-14

- Add support for `http_body1::Body`. ([#161](https://github.com/taiki-e/auto_enums/pull/161), thanks @nwtgck)

## [0.8.3] - 2023-10-21

- Update to [new coroutine API since nightly-2023-10-21](https://github.com/rust-lang/rust/pull/116958). This renames unstable `generator_trait` feature to `coroutine_trait`. The old cargo feature name is kept as a deprecated alias and will be removed in the next breaking release. ([daf9165](https://github.com/taiki-e/auto_enums/commit/daf91653b925d53cde57b598f0d884fe35a53c60))

- Recognize full path to std types/functions. ([28507ca](https://github.com/taiki-e/auto_enums/commit/28507ca34bdce58a371e9bc671495975f3a34d1d))

## [0.8.2] - 2023-08-10

- Update `syn` to 2. ([#158](https://github.com/taiki-e/auto_enums/pull/158), thanks @cuviper)

## [0.8.1] - 2023-06-29

- Increase the minimum supported Rust version from Rust 1.31 to Rust 1.56.

- Update minimal version of `proc-macro2` to 1.0.60.

## [0.8.0] - 2022-12-10

- Remove `futures` feature. Use `futures03` feature instead. ([#124](https://github.com/taiki-e/auto_enums/pull/124))

- Merge `auto_enums_core` and `auto_enums_derive` crates into main `auto_enums` crate. ([#123](https://github.com/taiki-e/auto_enums/pull/123))

- Forbid custom `Unpin`/`Drop` impl if trait has `Pin<&mut self>` receiver. See [#135](https://github.com/taiki-e/auto_enums/pull/135) for more.

- Update `derive_utils` to 0.12. ([#135](https://github.com/taiki-e/auto_enums/pull/135))

## [0.7.12] - 2021-01-05

- Exclude unneeded files from crates.io.

## [0.7.11] - 2020-12-23

- [Add support for `tokio1::{AsyncRead, AsyncWrite, AsyncSeek, AsyncBufRead}`.](https://github.com/taiki-e/auto_enums/pull/122)

## [0.7.10] - 2020-11-15

- Documentation improvements.

## [0.7.9] - 2020-11-06

- Update `derive_utils` to 0.11.

## [0.7.8] - 2020-10-16

- [Add support for `tokio03::{AsyncRead, AsyncWrite, AsyncSeek, AsyncBufRead}`.](https://github.com/taiki-e/auto_enums/pull/114)

## [0.7.7] - 2020-09-21

- [Fix panic when multiple errors occur.](https://github.com/taiki-e/auto_enums/pull/111)

- Diagnostic improvements.

## [0.7.6] - 2020-09-18

- [`type_analysis` feature can now support impl trait in bindings.](https://github.com/taiki-e/auto_enums/pull/108)

  ```rust
  #[auto_enum]
  fn foo(x: i32) {
    // Unlike `feature(impl_trait_in_bindings)`, this works on stable compilers.
    #[auto_enum]
    let iter: impl Iterator<Item = i32> = match x {
        0 => Some(0).into_iter(),
        _ => 0..x,
    };
  }
  ```

## [0.7.5] - 2020-06-03

- Updated `derive_utils` to 0.10.

## [0.7.4] - 2020-05-07

- [Fixed an issue that `#[auto_enum]` on non-statement expression does not work without unstable features.](https://github.com/taiki-e/auto_enums/pull/97)

## [0.7.3] - 2020-04-19

- [Added support for `tokio02::{AsyncRead, AsyncWrite, AsyncSeek, AsyncBufRead}`.](https://github.com/taiki-e/auto_enums/pull/92)

- [Added support for `tokio01::{AsyncRead, AsyncWrite}`.](https://github.com/taiki-e/auto_enums/pull/92)

- [Added `futures03` feature. This is an alias of `futures` feature.](https://github.com/taiki-e/auto_enums/pull/92)

## [0.7.2] - 2020-04-13

- [Fix unused braces warnings.](https://github.com/taiki-e/auto_enums/pull/88)

- Update to support latest generator.

## [0.7.1] - 2019-11-16

- Updated to support `futures` 0.3.0. (futures feature is no longer unstable)

## [0.7.0] - 2019-10-20

- [Support `#[nested]` for nested if expressions.](https://github.com/taiki-e/auto_enums/pull/67)

- Fixed bugs of `"type_analysis"` feature.

- [Removed unstable `"exact_size_is_empty"`, `"read_initializer"`, and `"try_trait"` crate features.](https://github.com/taiki-e/auto_enums/pull/69)

## [0.6.4] - 2019-09-28

- Updated to support `futures-preview` 0.3.0-alpha.19.

## [0.6.3] - 2019-09-20

- [Removed usage of mutable global state from `#[auto_enum]` for forward compatibility.](https://github.com/taiki-e/auto_enums/pull/60) See also [rust-lang/rust#63831](https://github.com/rust-lang/rust/pull/63831).

## [0.6.2] - 2019-09-08

- Fixed links to generated code.

## [0.6.1] - 2019-09-08

- Documentation improvements.

## [0.6.0] - 2019-09-07

- [Added `"unstable"` crate feature to separate unstable features from stable features.](https://github.com/taiki-e/auto_enums/pull/56) When using features that depend on unstable APIs, the `"unstable"` feature must be explicitly enabled.

- Improved compile time.

- Renamed `#[rec]` to `#[nested]`.

- [Removed `marker(name)` option in favor of `marker = name`.](https://github.com/taiki-e/auto_enums/pull/55)

- [Removed `never` option in argument position in favor of `#[enum_derive]` attribute.](https://github.com/taiki-e/auto_enums/pull/48)

- [Removed `"proc_macro"` crate feature.](https://github.com/taiki-e/auto_enums/pull/54)

- Added `"ops"` crate feature, and made `[std|core]::ops`'s `Deref`, `DerefMut`, `Index`, `IndexMut`, and `RangeBounds` traits optional.

- Added `"convert"` crate feature, and made `[std|core]::convert`'s `AsRef` and `AsMut` traits optional.

- Added `"generator_trait"` crate feature, and made `[std|core]::ops::Generator` traits optional. *(nightly-only)*

- Added `"fn_traits"` crate feature, and made `Fn`, `FnMut`, and `FnOnce` traits optional. *(nightly-only)*

- Added `"trusted_len"` crate feature, and made `[std|core]::iter::TrustedLen` traits optional. *(nightly-only)*

- Diagnostic improvements.

(There are no changes since the 0.6.0-alpha.3 release.)

## [0.6.0-alpha.3] - 2019-09-06

- [Added `"unstable"` crate feature to separate unstable features from stable features.](https://github.com/taiki-e/auto_enums/pull/56) When using features that depend on unstable APIs, the `"unstable"` feature must be explicitly enabled.

## [0.6.0-alpha.2] - 2019-08-30

- [Removed `marker(name)` option in favor of `marker = name`.](https://github.com/taiki-e/auto_enums/pull/55)

- [Removed `"proc_macro"` crate feature.](https://github.com/taiki-e/auto_enums/pull/54)

## [0.6.0-alpha.1] - 2019-08-24

- Renamed `#[rec]` to `#[nested]`.

- [Removed `never` option in argument position in favor of `#[enum_derive]` attribute.](https://github.com/taiki-e/auto_enums/pull/48)

- Improved compile time.

- Added `"ops"` crate feature, and made `[std|core]::ops`'s `Deref`, `DerefMut`, `Index`, `IndexMut`, and `RangeBounds` traits optional.

- Added `"convert"` crate feature, and made `[std|core]::convert`'s `AsRef` and `AsMut` traits optional.

- Added `"generator_trait"` crate feature, and made `[std|core]::ops::Generator` traits optional. *(nightly-only)*

- Added `"fn_traits"` crate feature, and made `Fn`, `FnMut`, and `FnOnce` traits optional. *(nightly-only)*

- Added `"trusted_len"` crate feature, and made `[std|core]::iter::TrustedLen` traits optional. *(nightly-only)*

- Diagnostic improvements.

## [0.5.10] - 2019-08-15

- Updated `proc-macro2`, `syn`, and `quote` to 1.0.

- Updated `derive_utils` to 0.9. This improves the error message.

- Added some generated code examples.

## [0.5.9] - 2019-07-07

- Updated to support `futures-preview` 0.3.0-alpha.17.

- Added some generated code examples.

## [0.5.8] - 2019-05-22

- Added support for `futures::io::{AsyncSeek, AsyncBufRead}`.

## [0.5.7] - 2019-05-12

- Updated to new nightly. `iovec` stabilized. `#[enum_derive]` automatically detects the rustc version and supports `Read::read_vectored` and `Write::write_vectored` as the part of `Read` and `Write`.

- Supported for latest `futures` 0.3.0-alpha.16.

## [0.5.6] - 2019-04-16

- Updated to new nightly. `futures_api` stabilized.

## [0.5.5] - 2019-03-29

- Fixed trait support in `"type_analysis"` feature.

## [0.5.4] - 2019-03-14

- Fixed the problem that `"failed to resolve: use of undeclared type or module"` (E0433) error is shown when one or more compilation errors occur when multiple `#[auto_enum]` attributes are used.

- Improved the error message of `#[enum_derive]` attribute.

- Updated minimum `derive_utils` version to 0.7.0. This improves the error message.

## [0.5.3] - 2019-03-13

- Greatly improved the error message of `#[auto_enum]` attribute.

## [0.5.2] - 2019-03-10

- Added some generated code examples.

- Added `"iovec"` crate feature. This supports the unstable `iovec` feature ([rust-lang/rust#58452](https://github.com/rust-lang/rust/issues/58452)).

- Updated minimum `syn` version to 0.15.29. This fixes some warnings.

## [0.5.1] - 2019-03-03

- Fixed examples and some sentence in README.md.

## [0.5.0] - 2019-03-03

- Transition to Rust 2018. With this change, the minimum required version will go up to Rust 1.31.

- Reduced the feature of `"std"` crate feature. The current `"std"` crate feature only determines whether to enable `std` library's traits (e.g., `std::io::Read`) support. `"std"` crate feature is enabled by default, but you can reduce compile time by disabling this feature.

- Fixed problem where "macro attributes must be placed before `#[derive]`" error occurred when `#[enum_derive]` attribute was used with other attributes.

- No longer need `#[macro_use] extern crate auto_enums;`. You can use `#[auto_enum]` attribute by `use auto_enums::auto_enum;`.

- Removed `"unstable"` crate feature.

## [0.4.1] - 2019-02-21

- Updated to new nightly.

- Added some generated code examples.

- Updated minimum `derive_utils` version to 0.6.3.

- Updated minimum `syn` version to 0.15.22.

- Updated minimum `smallvec` version to 0.6.9.

## [0.4.0] - 2019-01-30

- Added support for `?` operator in functions and closures.

- Added support for `[core|std]::ops::Generator`.

## [0.3.8] - 2019-01-26

- Updated minimum `derive_utils` version to 0.6.1.

- Updated minimum `smallvec` version to 0.6.8.

## [0.3.7] - 2019-01-26

- Fixed bug of closure support.

## [0.3.6] - 2019-01-19

- Parentheses and type ascription can now be searched recursively.

## [0.3.5] - 2019-01-09

- Improved performance of `#[auto_enum]` attribute.

- Updated minimum `derive_utils` version to 0.6.0.

## [0.3.4] - 2019-01-06

- Added support for `futures::{AsyncRead, AsyncWrite}`.

## [0.3.3] - 2019-01-04

- Updated minimum `derive_utils` version to 0.5.4.

## [0.3.2] - 2018-12-27

- Improved error messages.

- Updated minimum `derive_utils` version to 0.5.3.

## [0.3.1] - 2018-12-26

- Updated minimum `derive_utils` version to 0.5.1. This includes support to stable Pin API.

## [0.3.0] - 2018-12-24

- Added support for `break` in loop. This includes support for nested loops and labeled `break`.

- Changed `#[enum_derive(Error)]` implementation. [The code generated by the new implementation](https://github.com/taiki-e/auto_enums/tree/v0.3.0/docs/supported_traits/std/error.md).

- Removed `"error_cause"` crate feature.

- Updated minimum `derive_utils` version to 0.5.0.

## [0.2.1] - 2018-12-22

- Updated minimum `derive_utils` version to 0.4.0.

## [0.2.0] - 2018-12-20

- Added support for `return` in function and closure.

- Added `"fmt"` crate feature, and made `[std|core]::fmt`'s traits other than `Debug`, `Display` and `Write` optional.

## [0.1.3] - 2018-12-15

- Changed `#[enum_derive(Error)]` implementation. In Rust 1.33, `Error::cause` is deprecated. In the new implementation, `Error::cause` is optional for Rust 1.33 and later. In versions less than 1.33, `Error::cause` is always implemented.

## [0.1.2] - 2018-12-15

- Moved features of derive/utils to [derive_utils](https://github.com/taiki-e/derive_utils).

- Aligned version number of `auto_enumerate` and `auto_enums`.

## 0.1.1 - 2018-12-13

- Renamed from `auto_enumerate` to `auto_enums`.

## 0.1.0 - 2018-12-09

Initial release

[Unreleased]: https://github.com/taiki-e/auto_enums/compare/v0.8.5...HEAD
[0.8.5]: https://github.com/taiki-e/auto_enums/compare/v0.8.4...v0.8.5
[0.8.4]: https://github.com/taiki-e/auto_enums/compare/v0.8.3...v0.8.4
[0.8.3]: https://github.com/taiki-e/auto_enums/compare/v0.8.2...v0.8.3
[0.8.2]: https://github.com/taiki-e/auto_enums/compare/v0.8.1...v0.8.2
[0.8.1]: https://github.com/taiki-e/auto_enums/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/taiki-e/auto_enums/compare/v0.7.12...v0.8.0
[0.7.12]: https://github.com/taiki-e/auto_enums/compare/v0.7.11...v0.7.12
[0.7.11]: https://github.com/taiki-e/auto_enums/compare/v0.7.10...v0.7.11
[0.7.10]: https://github.com/taiki-e/auto_enums/compare/v0.7.9...v0.7.10
[0.7.9]: https://github.com/taiki-e/auto_enums/compare/v0.7.8...v0.7.9
[0.7.8]: https://github.com/taiki-e/auto_enums/compare/v0.7.7...v0.7.8
[0.7.7]: https://github.com/taiki-e/auto_enums/compare/v0.7.6...v0.7.7
[0.7.6]: https://github.com/taiki-e/auto_enums/compare/v0.7.5...v0.7.6
[0.7.5]: https://github.com/taiki-e/auto_enums/compare/v0.7.4...v0.7.5
[0.7.4]: https://github.com/taiki-e/auto_enums/compare/v0.7.3...v0.7.4
[0.7.3]: https://github.com/taiki-e/auto_enums/compare/v0.7.2...v0.7.3
[0.7.2]: https://github.com/taiki-e/auto_enums/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/taiki-e/auto_enums/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/taiki-e/auto_enums/compare/v0.6.4...v0.7.0
[0.6.4]: https://github.com/taiki-e/auto_enums/compare/v0.6.3...v0.6.4
[0.6.3]: https://github.com/taiki-e/auto_enums/compare/v0.6.2...v0.6.3
[0.6.2]: https://github.com/taiki-e/auto_enums/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/taiki-e/auto_enums/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/taiki-e/auto_enums/compare/v0.6.0-alpha.3...v0.6.0
[0.6.0-alpha.3]: https://github.com/taiki-e/auto_enums/compare/v0.6.0-alpha.2...v0.6.0-alpha.3
[0.6.0-alpha.2]: https://github.com/taiki-e/auto_enums/compare/v0.6.0-alpha.1...v0.6.0-alpha.2
[0.6.0-alpha.1]: https://github.com/taiki-e/auto_enums/compare/v0.5.10...v0.6.0-alpha.1
[0.5.10]: https://github.com/taiki-e/auto_enums/compare/v0.5.9...v0.5.10
[0.5.9]: https://github.com/taiki-e/auto_enums/compare/v0.5.8...v0.5.9
[0.5.8]: https://github.com/taiki-e/auto_enums/compare/v0.5.7...v0.5.8
[0.5.7]: https://github.com/taiki-e/auto_enums/compare/v0.5.6...v0.5.7
[0.5.6]: https://github.com/taiki-e/auto_enums/compare/v0.5.5...v0.5.6
[0.5.5]: https://github.com/taiki-e/auto_enums/compare/v0.5.4...v0.5.5
[0.5.4]: https://github.com/taiki-e/auto_enums/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/taiki-e/auto_enums/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/taiki-e/auto_enums/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/taiki-e/auto_enums/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/taiki-e/auto_enums/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/taiki-e/auto_enums/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/taiki-e/auto_enums/compare/v0.3.8...v0.4.0
[0.3.8]: https://github.com/taiki-e/auto_enums/compare/v0.3.7...v0.3.8
[0.3.7]: https://github.com/taiki-e/auto_enums/compare/v0.3.6...v0.3.7
[0.3.6]: https://github.com/taiki-e/auto_enums/compare/v0.3.5...v0.3.6
[0.3.5]: https://github.com/taiki-e/auto_enums/compare/v0.3.4...v0.3.5
[0.3.4]: https://github.com/taiki-e/auto_enums/compare/v0.3.3...v0.3.4
[0.3.3]: https://github.com/taiki-e/auto_enums/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/taiki-e/auto_enums/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/taiki-e/auto_enums/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/taiki-e/auto_enums/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/taiki-e/auto_enums/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/taiki-e/auto_enums/compare/v0.1.3...v0.2.0
[0.1.3]: https://github.com/taiki-e/auto_enums/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/taiki-e/auto_enums/releases/tag/v0.1.2
