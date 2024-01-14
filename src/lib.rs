// SPDX-License-Identifier: Apache-2.0 OR MIT

/*!
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

# `#[auto_enum]`

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

<details>
<summary>Code like this will be generated:</summary>

```rust
fn foo(x: i32) -> impl Iterator<Item = i32> {
    enum __Enum1<__T1, __T2> {
        __T1(__T1),
        __T2(__T2),
    }

    impl<__T1, __T2> ::core::iter::Iterator for __Enum1<__T1, __T2>
    where
        __T1: ::core::iter::Iterator,
        __T2: ::core::iter::Iterator<Item = <__T1 as ::core::iter::Iterator>::Item>,
    {
        type Item = <__T1 as ::core::iter::Iterator>::Item;
        #[inline]
        fn next(&mut self) -> ::core::option::Option<Self::Item> {
            match self {
                __Enum1::__T1(x) => x.next(),
                __Enum1::__T2(x) => x.next(),
            }
        }
        #[inline]
        fn size_hint(&self) -> (usize, ::core::option::Option<usize>) {
            match self {
                __Enum1::__T1(x) => x.size_hint(),
                __Enum1::__T2(x) => x.size_hint(),
            }
        }
    }

    match x {
        0 => __Enum1::__T1(1..10),
        _ => __Enum1::__T2(vec![5, 10].into_iter()),
    }
}
```

</details>
<br>

## Nested arms/branches

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

`#[nested]` can be used basically in the same place as `#[auto_enum]`,
except that `#[nested]` cannot be used in functions.

## Recursion

If an error due to recursion occurs, you need to box branches where recursion occurs.

```rust
use auto_enums::auto_enum;

struct Type {
    child: Vec<Type>,
}

impl Type {
    #[auto_enum(Iterator)]
    fn method(&self) -> impl Iterator<Item = ()> + '_ {
        if self.child.is_empty() {
            Some(()).into_iter()
        } else {
            // Boxing is only needed on branches where recursion occurs.
            Box::new(self.child.iter().flat_map(|c| c.method())) as Box<dyn Iterator<Item = _>>
        }
    }
}
```

## Positions where `#[auto_enum]` can be used.

`#[auto_enum]` can be used in the following three places. However, since
[stmt_expr_attributes] and [proc_macro_hygiene] are not stabilized, you need
to use empty `#[auto_enum]` for functions except nightly.

[stmt_expr_attributes]: https://github.com/rust-lang/rust/issues/15701
[proc_macro_hygiene]: https://github.com/rust-lang/rust/issues/54727

- functions

  ```rust
  use auto_enums::auto_enum;

  #[auto_enum(Iterator)]
  fn func(x: i32) -> impl Iterator<Item=i32> {
      if x == 0 {
          Some(0).into_iter()
      } else {
          0..x
      }
  }
  ```

- expressions

  ```rust
  use auto_enums::auto_enum;

  #[auto_enum] // Nightly does not need an empty attribute to the function.
  fn expr(x: i32) -> impl Iterator<Item=i32> {
      #[auto_enum(Iterator)]
      match x {
          0 => Some(0).into_iter(),
          _ => 0..x,
      }
  }
  ```

- let binding

  ```rust
  use auto_enums::auto_enum;

  #[auto_enum] // Nightly does not need an empty attribute to the function.
  fn let_binding(x: i32) -> impl Iterator<Item=i32> {
      #[auto_enum(Iterator)]
      let iter = match x {
          0 => Some(0).into_iter(),
          _ => 0..x,
      };
      iter
  }
  ```

## Supported syntax

- `if` and `match`

  Wrap each branch with a variant.

  ```rust
  use auto_enums::auto_enum;

  // if
  #[auto_enum(Iterator)]
  fn expr_if(x: i32) -> impl Iterator<Item=i32> {
      if x == 0 {
          Some(0).into_iter()
      } else {
          0..x
      }
  }

  // match
  #[auto_enum] // Nightly does not need an empty attribute to the function.
  fn expr_match(x: i32) -> impl Iterator<Item=i32> {
      #[auto_enum(Iterator)]
      let iter = match x {
          0 => Some(0).into_iter(),
          _ => 0..x,
      };
      iter
  }
  ```

- `loop`

  Wrap each `break` with a variant. Nested loops and labeled `break` are
  also supported.

  ```rust
  use auto_enums::auto_enum;

  #[auto_enum(Iterator)]
  fn expr_loop(mut x: i32) -> impl Iterator<Item = i32> {
      loop {
          if x < 0 {
              break x..0;
          } else if x % 5 == 0 {
              break 0..=x;
          }
          x -= 1;
      }
  }
  ```

- `return` (in functions)

  `#[auto_enum]` can parse the `return` in the scope.

  This analysis is valid only when the return type is `impl Trait`.

  ```rust
  use auto_enums::auto_enum;

  // return (in functions)
  #[auto_enum(Iterator)]
  fn func(x: i32) -> impl Iterator<Item=i32> {
      if x == 0 {
          return Some(0).into_iter();
      }

      if x > 0 {
          0..x
      } else {
          x..=0
      }
  }
  ```

- `return` (in closures)

  `#[auto_enum]` can parse the `return` in the scope.

  This analysis is valid only when the following two conditions are satisfied.

    - `#[auto_enum]` must be used directly for that closure (or the let binding of the closure).
    - `?` operator not used in the scope.

  ```rust
  use auto_enums::auto_enum;

  // return (in closures)
  #[auto_enum] // Nightly does not need an empty attribute to the function.
  fn closure() -> impl Iterator<Item=i32> {
      #[auto_enum(Iterator)]
      let f = |x| {
          if x == 0 {
              return Some(0).into_iter();
          }

          if x > 0 {
              0..x
          } else {
              x..=0
          }
      };
      f(1)
  }
  ```

- `?` operator (in functions)

  `#[auto_enum]` can parse the `?` operator in the scope.

  This analysis is valid only when the return type is `Result<T, impl Trait>`.

  ```rust
  use auto_enums::auto_enum;
  use std::fmt::{Debug, Display};

  // `?` operator (in functions)
  #[auto_enum(Debug, Display)]
  fn func(x: i32) -> Result<i32, impl Debug + Display> {
      if x == 0 {
          Err("`x` is zero")?;
      }

      // The last branch of the function is not parsed.
      if x < 0 {
          Err(x)?
      } else {
          Ok(x + 1)
      }
  }
  ```

  `?` operator is expanded as follows:

  ```rust
  # enum Enum<A> { Variant(A) }
  # fn dox<T, E>(expr: Result<T, E>) -> Result<T, Enum<E>> {
  # Ok(
  match expr {
      Ok(val) => val,
      Err(err) => return Err(Enum::Variant(err)),
  }
  # )
  # }
  ```

- `?` operator (in closures)

  `#[auto_enum]` can parse the `?` operator in the scope.

  However, `#[auto_enum]` must be used directly for that closure
  (or the let binding of the closure).

  ```rust
  use auto_enums::auto_enum;
  use std::fmt::{Debug, Display};

  // `?` operator (in closures)
  #[auto_enum] // Nightly does not need an empty attribute to the function.
  fn closure() -> Result<i32, impl Debug + Display> {
      #[auto_enum(Debug, Display)]
      let f = |x| {
          if x == 0 {
              Err("`x` is zero")?
          }

          // The last branch of the function is not interpreted as a branch.
          if x < 0 {
              Err(x)?
          } else {
              Ok(x + 1)
          }
      };
      f(1)
  }
  ```

- Block, unsafe block, method call, parentheses, and type ascription

  The following expressions are recursively searched until an `if`, `match`,
  `loop` or unsupported expression is found.

  - blocks
  - unsafe blocks
  - method calls
  - parentheses
  - type ascriptions

  ```rust
  use auto_enums::auto_enum;

  // block
  #[auto_enum] // Nightly does not need an empty attribute to the function.
  fn expr_block(x: i32) -> impl Iterator<Item=i32> {
      #[auto_enum(Iterator)]
      {
          if x == 0 {
              Some(0).into_iter()
          } else {
              0..x
          }
      }
  }

  // method call
  #[auto_enum] // Nightly does not need an empty attribute to the function.
  fn expr_method(x: i32) -> impl Iterator<Item=i32> {
     #[auto_enum(Iterator)]
      match x {
          0 => Some(0).into_iter(),
          _ => 0..x,
      }.map(|y| y + 1)
  }

  // parentheses
  # #[allow(unused_parens)]
  #[auto_enum(Iterator)]
  fn expr_parentheses(x: i32) -> impl Iterator<Item=i32> {
      (if x == 0 { Some(0).into_iter() } else { 0..x })
  }
  ```

## Expression that no value will be returned

If the last expression of a branch is one of the following, it is
interpreted that no value will be returned (variant assignment is skipped).

- `panic!(..)`
- `unreachable!(..)`
- `return`
- `break`
- `continue`
- `None?`
- `Err(..)?`
- Expression level marker (`marker!` macro).
- An item definition.

Also, if the branch contains `#[nested]`, it is interpreted as returning
an anonymous enum generated by `#[auto_enum]`, not a value.

```rust
use auto_enums::auto_enum;

#[auto_enum(Iterator)]
fn foo(x: i32) -> impl Iterator<Item = i32> {
    match x {
        0 => 1..10,
        1 => panic!(), // variant assignment is skipped
        _ => vec![5, 10].into_iter(),
    }
}
```

You can also skip that branch explicitly by `#[never]` attribute.

```rust
use auto_enums::auto_enum;

#[auto_enum(Iterator)]
fn foo(x: i32) -> impl Iterator<Item = i32> {
    match x {
        0 => 1..10,
        #[never]
        1 => loop {
            panic!()
        },
        _ => vec![5, 10].into_iter(),
    }
}
```

## Expression level marker (`marker!` macro)

`#[auto_enum]` replaces `marker!` macros with variants.
If values of two or more are specified by `marker!` macros, `#[auto_enum]`
can be used for unsupported expressions and statements.

```rust
use auto_enums::auto_enum;

#[auto_enum(Iterator)]
fn foo(x: i32) -> impl Iterator<Item = i32> {
    if x < 0 {
        return x..=0;
    }
    marker!(1..10)
}
```

The default name of the macro is `"marker"`, but you can change it by
`marker` option.

```rust
use auto_enums::auto_enum;

#[auto_enum(marker = bar, Iterator)]
fn foo(x: i32) -> impl Iterator<Item = i32> {
    if x < 0 {
        return x..=0;
    }
    bar!(1..10)
}
```

## Rust Nightly

When using `#[auto_enum]` for expressions and statements, `#[auto_enum]` for
function is unnecessary.

```rust
// Add this to your crate root:
#![feature(proc_macro_hygiene, stmt_expr_attributes)]
```

```rust
# #![feature(proc_macro_hygiene, stmt_expr_attributes)]
use auto_enums::auto_enum;

fn foo(x: i32) -> i32 {
    #[auto_enum(Iterator)]
    let iter = match x {
        0 => 1..10,
        _ => vec![5, 10].into_iter(),
    };

    iter.fold(0, |sum, x| sum + x)
}
```

You can also return closures.

```rust
// Add this to your crate root:
#![feature(fn_traits, unboxed_closures)]
```

```rust
# #![feature(fn_traits, unboxed_closures)]
use auto_enums::auto_enum;

#[auto_enum(Fn)]
fn foo(x: bool) -> impl Fn(i32) -> i32 {
    if x {
        |y| y + 1
    } else {
        |z| z - 1
    }
}
```

# `#[enum_derive]`

`#[enum_derive]` implements the supported traits and passes unsupported
traits to `#[derive]`.

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

`#[enum_derive]` adds the dependency of the specified trait if it is not
specified.

```rust
use auto_enums::enum_derive;

// `#[enum_derive]` implements `Iterator` and `ExactSizeIterator`.
#[enum_derive(ExactSizeIterator)]
enum Foo<A, B> {
    A(A),
    B(B),
}
```

[derive_utils]: https://github.com/taiki-e/derive_utils

# Supported traits

Some traits support is disabled by default.
Note that some traits have aliases.

*When using features that depend on unstable APIs, the `unstable` feature must be explicitly enabled*

## The standard library (`std`, `core`)

### `[std|core]::iter`

- [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/iter/iterator.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/iter/iterator.expanded.rs)
- [`DoubleEndedIterator`](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/iter/double_ended_iterator.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/iter/double_ended_iterator.expanded.rs)
- [`ExactSizeIterator`](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/iter/exact_size_iterator.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/iter/exact_size_iterator.expanded.rs)
- [`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/iter/fused_iterator.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/iter/fused_iterator.expanded.rs)
- [`Extend`](https://doc.rust-lang.org/std/iter/trait.Extend.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/iter/extend.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/iter/extend.expanded.rs)
- [`TrustedLen`](https://doc.rust-lang.org/std/iter/trait.TrustedLen.html) *(requires `"trusted_len"` and `"unstable"` crate features)* - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/iter/trusted_len.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/iter/trusted_len.expanded.rs)

*See also [iter-enum] crate.*

### `[std|core]::future`

- [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/future.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/future.expanded.rs)

### `std::io` *(requires `"std"` crate feature)*

- [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) (alias: `io::Read`) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/io/read.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/io/read.expanded.rs)
- [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html) (alias: `io::BufRead`) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/io/buf_read.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/io/buf_read.expanded.rs)
- [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) (alias: `io::Write`) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/io/write.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/io/write.expanded.rs)
- [`Seek`](https://doc.rust-lang.org/std/io/trait.Seek.html) (alias: `io::Seek`) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/io/seek.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/io/seek.expanded.rs)

*See also [io-enum] crate.*

### `[std|core]::ops`

- [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html) *(requires `"ops"` crate feature)*
- [`DerefMut`](https://doc.rust-lang.org/std/ops/trait.DerefMut.html) *(requires `"ops"` crate feature)*
- [`Index`](https://doc.rust-lang.org/std/ops/trait.Index.html) *(requires `"ops"` crate feature)*
- [`IndexMut`](https://doc.rust-lang.org/std/ops/trait.IndexMut.html) *(requires `"ops"` crate feature)*
- [`RangeBounds`](https://doc.rust-lang.org/std/ops/trait.RangeBounds.html) *(requires `"ops"` crate feature)*
- [`Fn`](https://doc.rust-lang.org/std/ops/trait.Fn.html) *(requires `"fn_traits"` and `"unstable"` crate features)*
- [`FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html) *(requires `"fn_traits"` and `"unstable"` crate features)*
- [`FnOnce`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html) *(requires `"fn_traits"` and `"unstable"` crate features)*
- [`Coroutine`](https://doc.rust-lang.org/nightly/std/ops/trait.Coroutine.html) *(requires `"coroutine_trait"` and `"unstable"` crate features)*

### `[std|core]::convert`

- [`AsRef`](https://doc.rust-lang.org/std/convert/trait.AsRef.html) *(requires `"convert"` crate feature)*
- [`AsMut`](https://doc.rust-lang.org/std/convert/trait.AsMut.html) *(requires `"convert"` crate feature)*

### `[std|core]::fmt`

- [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) (alias: `fmt::Debug`) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/debug.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/debug.expanded.rs)
- [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) (alias: `fmt::Display`)
- [`fmt::Binary`](https://doc.rust-lang.org/std/fmt/trait.Binary.html) *(requires `"fmt"` crate feature)*
- [`fmt::LowerExp`](https://doc.rust-lang.org/std/fmt/trait.LowerExp.html) *(requires `"fmt"` crate feature)*
- [`fmt::LowerHex`](https://doc.rust-lang.org/std/fmt/trait.LowerHex.html) *(requires `"fmt"` crate feature)*
- [`fmt::Octal`](https://doc.rust-lang.org/std/fmt/trait.Octal.html) *(requires `"fmt"` crate feature)*
- [`fmt::Pointer`](https://doc.rust-lang.org/std/fmt/trait.Pointer.html) *(requires `"fmt"` crate feature)*
- [`fmt::UpperExp`](https://doc.rust-lang.org/std/fmt/trait.UpperExp.html) *(requires `"fmt"` crate feature)*
- [`fmt::UpperHex`](https://doc.rust-lang.org/std/fmt/trait.UpperHex.html) *(requires `"fmt"` crate feature)*
- [`fmt::Write`](https://doc.rust-lang.org/std/fmt/trait.Write.html)

### `std::error` *(requires `"std"` crate feature)*

- [`Error`](https://doc.rust-lang.org/std/error/trait.Error.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/error.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/std/error.expanded.rs)

## External libraries

You can use support for external library traits by activating each crate feature.

To use support for external library traits, you need to use the path starting with the feature name. For example:

```rust
# extern crate rayon_crate as rayon;
use auto_enums::auto_enum;
use rayon::prelude::*;

#[auto_enum(rayon::ParallelIterator)] // Note that this is not `#[auto_enum(ParallelIterator)]`
fn func(x: i32) -> impl ParallelIterator {
    match x {
        0 => (1..10).into_par_iter(),
        _ => vec![5, 10].into_par_iter(),
    }
}
```

### [futures v0.3][futures03] *(requires `"futures03"` crate feature)*

- [`futures03::Stream`](https://docs.rs/futures/0.3/futures/stream/trait.Stream.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/futures/stream.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/futures/stream.expanded.rs)
- [`futures03::Sink`](https://docs.rs/futures/0.3/futures/sink/trait.Sink.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/futures/sink.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/futures/sink.expanded.rs)
- [`futures03::AsyncRead`](https://docs.rs/futures/0.3/futures/io/trait.AsyncRead.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/futures/async_read.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/futures/async_read.expanded.rs)
- [`futures03::AsyncWrite`](https://docs.rs/futures/0.3/futures/io/trait.AsyncWrite.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/futures/async_write.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/futures/async_write.expanded.rs)
- [`futures03::AsyncSeek`](https://docs.rs/futures/0.3/futures/io/trait.AsyncSeek.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/futures/async_seek.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/futures/async_seek.expanded.rs)
- [`futures03::AsyncBufRead`](https://docs.rs/futures/0.3/futures/io/trait.AsyncBufRead.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/futures/async_buf_read.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/futures/async_buf_read.expanded.rs)

### [futures v0.1][futures01] *(requires `"futures01"` crate feature)*

- [`futures01::Future`](https://docs.rs/futures/0.1/futures/future/trait.Future.html)
- [`futures01::Stream`](https://docs.rs/futures/0.1/futures/stream/trait.Stream.html)
- [`futures01::Sink`](https://docs.rs/futures/0.1/futures/sink/trait.Sink.html)

### [rayon] *(requires `"rayon"` crate feature)*

- [`rayon::ParallelIterator`](https://docs.rs/rayon/1/rayon/iter/trait.ParallelIterator.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/rayon/parallel_iterator.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/rayon/parallel_iterator.expanded.rs)
- [`rayon::IndexedParallelIterator`](https://docs.rs/rayon/1/rayon/iter/trait.IndexedParallelIterator.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/rayon/indexed_parallel_iterator.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/rayon/indexed_parallel_iterator.expanded.rs)
- [`rayon::ParallelExtend`](https://docs.rs/rayon/1/rayon/iter/trait.ParallelExtend.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/rayon/parallel_extend.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/rayon/parallel_extend.expanded.rs)

### [serde] *(requires `"serde"` crate feature)*

- [`serde::Serialize`](https://docs.rs/serde/1/serde/trait.Serialize.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/serde/serialize.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/serde/serialize.expanded.rs)

### [tokio v1][tokio1] *(requires `"tokio1"` crate feature)*

- [`tokio1::AsyncRead`](https://docs.rs/tokio/1/tokio/io/trait.AsyncRead.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/tokio/async_read.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/tokio/async_read.expanded.rs)
- [`tokio1::AsyncWrite`](https://docs.rs/tokio/1/tokio/io/trait.AsyncWrite.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/tokio/async_write.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/tokio/async_write.expanded.rs)
- [`tokio1::AsyncSeek`](https://docs.rs/tokio/1/tokio/io/trait.AsyncSeek.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/tokio/async_seek.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/tokio/async_seek.expanded.rs)
- [`tokio1::AsyncBufRead`](https://docs.rs/tokio/1/tokio/io/trait.AsyncBufRead.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/tokio/async_buf_read.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/tokio/async_buf_read.expanded.rs)

### [tokio v0.3][tokio03] *(requires `"tokio03"` crate feature)*

- [`tokio03::AsyncRead`](https://docs.rs/tokio/0.3/tokio/io/trait.AsyncRead.html)
- [`tokio03::AsyncWrite`](https://docs.rs/tokio/0.3/tokio/io/trait.AsyncWrite.html)
- [`tokio03::AsyncSeek`](https://docs.rs/tokio/0.3/tokio/io/trait.AsyncSeek.html)
- [`tokio03::AsyncBufRead`](https://docs.rs/tokio/0.3/tokio/io/trait.AsyncBufRead.html)

### [tokio v0.2][tokio02] *(requires `"tokio02"` crate feature)*

- [`tokio02::AsyncRead`](https://docs.rs/tokio/0.2/tokio/io/trait.AsyncRead.html)
- [`tokio02::AsyncWrite`](https://docs.rs/tokio/0.2/tokio/io/trait.AsyncWrite.html)
- [`tokio02::AsyncSeek`](https://docs.rs/tokio/0.2/tokio/io/trait.AsyncSeek.html)
- [`tokio02::AsyncBufRead`](https://docs.rs/tokio/0.2/tokio/io/trait.AsyncBufRead.html)

### [tokio v0.1][tokio01] *(requires `"tokio01"` crate feature)*

- [`tokio01::AsyncRead`](https://docs.rs/tokio/0.1/tokio/io/trait.AsyncRead.html)
- [`tokio01::AsyncWrite`](https://docs.rs/tokio/0.1/tokio/io/trait.AsyncWrite.html)

### [http_body v1][http_body1] *(requires `"http_body1"` crate feature)*

- [`http_body1::Body`](https://docs.rs/http-body/1/http_body/trait.Body.html) - [example](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/http_body/body.rs) | [generated code](https://github.com/taiki-e/auto_enums/blob/HEAD/tests/expand/external/http_body/body.expanded.rs)

## Inherent methods

These don't derive traits, but derive inherent methods instead.

- `Transpose` *(requires `"transpose_methods"` crate feature)* - this derives the following conversion methods.

  - `transpose` — convert from `enum<Option<T1>,..>` to `Option<enum<T1,..>>`

  - `transpose` — convert from `enum<Result<T1, E1>,..>` to `Result<enum<T1,..>, enum<E1,..>>`

  - `transpose_ok` — convert from `enum<Result<T1, E>,..>` to `Option<enum<T1,..>, E>`

    Examples:

    ```rust
    use auto_enums::auto_enum;
    use std::{fs::File, io, path::Path};

    #[auto_enum(Transpose, Write)]
    fn output_stream(file: Option<&Path>) -> io::Result<impl io::Write> {
        match file {
            Some(f) => File::create(f),
            None => Ok(io::stdout()),
        }.transpose_ok()
    }
    ```

  - `transpose_err` — convert from `enum<Result<T, E1>,..>` to `Result<T, enum<E1,..>>`

# Optional features

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

## `type_analysis` feature

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

# Known limitations

- There needs to explicitly specify the trait to be implemented (`type_analysis` crate feature reduces this limitation).
- There needs to be marker macros for unsupported expressions.

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
[http_body1]: https://docs.rs/http-body/1
*/

#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms, single_use_lifetimes),
        allow(dead_code, unused_variables)
    )
))]
#![forbid(unsafe_code)]
#![allow(clippy::doc_link_with_quotes)]

#[cfg(all(feature = "coroutine_trait", not(feature = "unstable")))]
compile_error!(
    "The `coroutine_trait` feature requires the `unstable` feature as an explicit opt-in to unstable features"
);

#[cfg(all(feature = "fn_traits", not(feature = "unstable")))]
compile_error!(
    "The `fn_traits` feature requires the `unstable` feature as an explicit opt-in to unstable features"
);

#[cfg(all(feature = "trusted_len", not(feature = "unstable")))]
compile_error!(
    "The `trusted_len` feature requires the `unstable` feature as an explicit opt-in to unstable features"
);

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
const _README: () = ();

#[macro_use]
mod error;

mod auto_enum;
mod derive;
mod enum_derive;
mod utils;

use proc_macro::TokenStream;

/// An attribute macro like a wrapper of `#[derive]`, implementing
/// the supported traits and passing unsupported traits to `#[derive]`.
///
/// See crate level documentation for details.
#[proc_macro_attribute]
pub fn enum_derive(args: TokenStream, input: TokenStream) -> TokenStream {
    crate::enum_derive::attribute(args.into(), input.into()).into()
}

/// An attribute macro for to allow multiple return types by automatically generated enum.
///
/// See crate level documentation for details.
#[proc_macro_attribute]
pub fn auto_enum(args: TokenStream, input: TokenStream) -> TokenStream {
    crate::auto_enum::attribute(args.into(), input.into()).into()
}
