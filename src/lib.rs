//! A library for to allow multiple return types by automatically generated enum.
//!
//! This library provides the following attribute macros:
//!
//! * `#[auto_enum]`
//!
//!   Parses syntax, creates the enum, inserts variants, and passes specified
//!   traits to `#[enum_derive]`.
//!
//! * `#[enum_derive]`
//!
//!   Implements specified traits to the enum.
//!
//! ## `#[auto_enum]`
//!
//! `#[auto_enum]`'s basic feature is to wrap the value returned by the obvious
//! branches (`match`, `if`, `return`, etc..) by an enum that implemented the
//! specified traits.
//!
//! ```rust
//! use auto_enums::auto_enum;
//!
//! #[auto_enum(Iterator)]
//! fn foo(x: i32) -> impl Iterator<Item = i32> {
//!     match x {
//!         0 => 1..10,
//!         _ => vec![5, 10].into_iter(),
//!     }
//! }
//! # fn main() { let _ = foo(0); }
//! ```
//!
//! `#[auto_enum]` generates code in two stages.
//!
//! First, `#[auto_enum]` will do the following.
//!
//! * parses syntax
//! * creates the enum
//! * inserts variants
//!
//! Code like this will be generated:
//!
//! ```rust
//! fn foo(x: i32) -> impl Iterator<Item = i32> {
//!     #[::auto_enums::enum_derive(Iterator)]
//!     enum __Enum1<__T1, __T2> {
//!         __T1(__T1),
//!         __T2(__T2),
//!     }
//!
//!     match x {
//!         0 => __Enum1::__T1(1..10),
//!         _ => __Enum1::__T2(vec![5, 10].into_iter()),
//!     }
//! }
//! # fn main() { let _ = foo(0); }
//! ```
//!
//! Next, `#[enum_derive]` implements the specified traits.
//!
//! <details>
//! <summary>Code like this will be generated:</summary>
//!
//! ```rust
//! fn foo(x: i32) -> impl Iterator<Item = i32> {
//!     enum __Enum1<__T1, __T2> {
//!         __T1(__T1),
//!         __T2(__T2),
//!     }
//!
//!     impl<__T1, __T2> ::core::iter::Iterator for __Enum1<__T1, __T2>
//!     where
//!         __T1: ::core::iter::Iterator,
//!         __T2: ::core::iter::Iterator<Item = <__T1 as ::core::iter::Iterator>::Item>,
//!     {
//!         type Item = <__T1 as ::core::iter::Iterator>::Item;
//!         #[inline]
//!         fn next(&mut self) -> ::core::option::Option<Self::Item> {
//!             match self {
//!                 __Enum1::__T1(x) => x.next(),
//!                 __Enum1::__T2(x) => x.next(),
//!             }
//!         }
//!         #[inline]
//!         fn size_hint(&self) -> (usize, ::core::option::Option<usize>) {
//!             match self {
//!                 __Enum1::__T1(x) => x.size_hint(),
//!                 __Enum1::__T2(x) => x.size_hint(),
//!             }
//!         }
//!     }
//!
//!     match x {
//!         0 => __Enum1::__T1(1..10),
//!         _ => __Enum1::__T2(vec![5, 10].into_iter()),
//!     }
//! }
//! # fn main() { let _ = foo(0); }
//! ```
//!
//! </details>
//! <br>
//!
//! `#[auto_enum]` can also parse nested arms/branches by using the `#[nested]`
//! attribute.
//!
//! ```rust
//! # use auto_enums::auto_enum;
//! #[auto_enum(Iterator)]
//! fn foo(x: i32) -> impl Iterator<Item = i32> {
//!     match x {
//!         0 => 1..10,
//!         #[nested]
//!         _ => match x {
//!             1 => vec![5, 10].into_iter(),
//!             _ => 0..=x,
//!         },
//!     }
//! }
//! # fn main() { let _ = foo(0); }
//! ```
//!
//! `#[nested]` can be used basically in the same place as `#[auto_enum]`,
//! except that `#[nested]` cannot be used in functions.
//!
//! ### Positions where `#[auto_enum]` can be used.
//!
//! `#[auto_enum]` can be used in the following three places. However, since
//! [stmt_expr_attributes] and [proc_macro_hygiene] are not stabilized, you need
//! to use empty `#[auto_enum]` for functions except nightly.
//!
//! [stmt_expr_attributes]: https://github.com/rust-lang/rust/issues/15701
//! [proc_macro_hygiene]: https://github.com/rust-lang/rust/issues/54727
//!
//! * functions
//!
//!   ```rust
//!   # use auto_enums::auto_enum;
//!   #[auto_enum(Iterator)]
//!   fn func(x: i32) -> impl Iterator<Item=i32> {
//!       if x == 0 {
//!           Some(0).into_iter()
//!       } else {
//!           0..x
//!       }
//!   }
//!   # fn main() { let _ = func(0); }
//!   ```
//!
//! * expressions
//!
//!   ```rust
//!   # use auto_enums::auto_enum;
//!   #[auto_enum] // Nightly does not need an empty attribute to the function.
//!   fn expr(x: i32) -> impl Iterator<Item=i32> {
//!       #[auto_enum(Iterator)]
//!       match x {
//!           0 => Some(0).into_iter(),
//!           _ => 0..x,
//!       }
//!   }
//!   # fn main() { let _ = expr(0); }
//!   ```
//!
//! * let binding
//!
//!   ```rust
//!   # use auto_enums::auto_enum;
//!   #[auto_enum] // Nightly does not need an empty attribute to the function.
//!   fn let_binding(x: i32) -> impl Iterator<Item=i32> {
//!       #[auto_enum(Iterator)]
//!       let iter = match x {
//!           0 => Some(0).into_iter(),
//!           _ => 0..x,
//!       };
//!       iter
//!   }
//!   # fn main() { let _ = let_binding(0); }
//!   ```
//!
//! ### Supported syntax
//!
//! * `if` and `match`
//!
//!   Wrap each branch with a variant.
//!
//!   ```rust
//!   # use auto_enums::auto_enum;
//!   // if
//!   #[auto_enum(Iterator)]
//!   fn expr_if(x: i32) -> impl Iterator<Item=i32> {
//!       if x == 0 {
//!           Some(0).into_iter()
//!       } else {
//!           0..x
//!       }
//!   }
//!
//!   // match
//!   #[auto_enum] // Nightly does not need an empty attribute to the function.
//!   fn expr_match(x: i32) -> impl Iterator<Item=i32> {
//!       #[auto_enum(Iterator)]
//!       let iter = match x {
//!           0 => Some(0).into_iter(),
//!           _ => 0..x,
//!       };
//!       iter
//!   }
//!   # fn main() { let _ = expr_if(0); let _ = expr_match(0); }
//!   ```
//!
//! * `loop`
//!
//!   Wrap each `break` with a variant. Nested loops and labeled `break` are
//!   also supported.
//!
//!   ```rust
//!   # use auto_enums::auto_enum;
//!   #[auto_enum(Iterator)]
//!   fn expr_loop(mut x: i32) -> impl Iterator<Item = i32> {
//!       loop {
//!           if x < 0 {
//!               break x..0;
//!           } else if x % 5 == 0 {
//!               break 0..=x;
//!           }
//!           x -= 1;
//!       }
//!   }
//!   # fn main() { let _ = expr_loop(0); }
//!   ```
//!
//! * `return` (in functions)
//!
//!   `#[auto_enum]` can parse the `return` in the scope.
//!
//!   This analysis is valid only when the return type is `impl Trait`.
//!
//!   ```rust
//!   # use auto_enums::auto_enum;
//!   // return (in functions)
//!   #[auto_enum(Iterator)]
//!   fn func(x: i32) -> impl Iterator<Item=i32> {
//!       if x == 0 {
//!           return Some(0).into_iter();
//!       }
//!
//!       if x > 0 {
//!           0..x
//!       } else {
//!           x..=0
//!       }
//!   }
//!   # fn main() { let _ = func(1); }
//!   ```
//!
//! * `return` (in closures)
//!
//!   `#[auto_enum]` can parse the `return` in the scope.
//!
//!   This analysis is valid only when the following two conditions are satisfied.
//!
//!     * `#[auto_enum]` must be used directly for that closure (or the let binding of the closure).
//!     * `?` operator not used in the scope.
//!
//!   ```rust
//!   # use auto_enums::auto_enum;
//!   // return (in closures)
//!   #[auto_enum] // Nightly does not need an empty attribute to the function.
//!   fn closure() -> impl Iterator<Item=i32> {
//!       #[auto_enum(Iterator)]
//!       let f = |x| {
//!           if x == 0 {
//!               return Some(0).into_iter();
//!           }
//!
//!           if x > 0 {
//!               0..x
//!           } else {
//!               x..=0
//!           }
//!       };
//!       f(1)
//!   }
//!   # fn main() { let _ = closure(); }
//!   ```
//!
//! * `?` operator (in functions)
//!
//!   `#[auto_enum]` can parse the `?` operator in the scope.
//!
//!   This analysis is valid only when the return type is `Result<T, impl Trait>`.
//!
//!   ```rust
//!   # use auto_enums::auto_enum;
//!   use std::fmt::{Debug, Display};
//!
//!   // `?` operator (in functions)
//!   #[auto_enum(Debug, Display)]
//!   fn func(x: i32) -> Result<i32, impl Debug + Display> {
//!       if x == 0 {
//!           Err("`x` is zero")?;
//!       }
//!
//!       // The last branch of the function is not parsed.
//!       if x < 0 {
//!           Err(x)?
//!       } else {
//!           Ok(x + 1)
//!       }
//!   }
//!   # fn main() { let _ = func(1); }
//!   ```
//!
//!   By default, `?` operator is expanded as follows:
//!
//!   ```rust
//!   # pub enum Enum<A> { Veriant(A) }
//!   # pub fn a<T, E>(expr: Result<T, E>) -> Result<T, Enum<E>> {
//!   # Ok(
//!   match expr {
//!       Ok(val) => val,
//!       Err(err) => return Err(Enum::Veriant(err)),
//!   }
//!   # )
//!   # }
//!   ```
//!
//! * `?` operator (in closures)
//!
//!   `#[auto_enum]` can parse the `?` operator in the scope.
//!
//!   However, `#[auto_enum]` must be used directly for that closure
//!   (or the let binding of the closure).
//!
//!   ```rust
//!   # use auto_enums::auto_enum;
//!   use std::fmt::{Debug, Display};
//!
//!   // `?` operator (in closures)
//!   #[auto_enum] // Nightly does not need an empty attribute to the function.
//!   fn closure() -> Result<i32, impl Debug + Display> {
//!       #[auto_enum(Debug, Display)]
//!       let f = |x| {
//!           if x == 0 {
//!               Err("`x` is zero")?
//!           }
//!
//!           // The last branch of the function is not interpreted as a branch.
//!           if x < 0 {
//!               Err(x)?
//!           } else {
//!               Ok(x + 1)
//!           }
//!       };
//!       f(1)
//!   }
//!   # fn main() { let _ = closure(); }
//!   ```
//!
//! * Block, unsafe block, method call, parentheses, and type ascription
//!
//!   The following expressions are recursively searched until an `if`, `match`,
//!   `loop` or unsupported expression is found.
//!
//!   * blocks
//!   * unsafe blocks
//!   * method calls
//!   * parentheses
//!   * type ascriptions
//!
//!   ```rust
//!   # use auto_enums::auto_enum;
//!   // block
//!   #[auto_enum] // Nightly does not need an empty attribute to the function.
//!   fn expr_block(x: i32) -> impl Iterator<Item=i32> {
//!       #[auto_enum(Iterator)]
//!       {
//!           if x == 0 {
//!               Some(0).into_iter()
//!           } else {
//!               0..x
//!           }
//!       }
//!   }
//!
//!   // method call
//!   #[auto_enum] // Nightly does not need an empty attribute to the function.
//!   fn expr_method(x: i32) -> impl Iterator<Item=i32> {
//!      #[auto_enum(Iterator)]
//!       match x {
//!           0 => Some(0).into_iter(),
//!           _ => 0..x,
//!       }.map(|y| y + 1)
//!   }
//!
//!   // parentheses
//!   #[auto_enum(Iterator)]
//!   fn expr_parentheses(x: i32) -> impl Iterator<Item=i32> {
//!       (if x == 0 { Some(0).into_iter() } else { 0..x })
//!   }
//!   # fn main() { let _ = expr_block(0); let _ = expr_method(0); let _ = expr_parentheses(0); }
//!   ```
//!
//! ### Expression that no value will be returned
//!
//! If the last expression of a branch is one of the following, it is
//! interpreted that no value will be returned (variant assignment is skipped).
//!
//! * `panic!(..)`
//! * `unreachable!(..)`
//! * `return`
//! * `break`
//! * `continue`
//! * `None?`
//! * `Err(..)?`
//! * Expression level marker (`marker!` macro).
//! * An item definition.
//!
//! Also, if the branch contains `#[nested]`, it is interpreted as returning
//! an anonymous enum generated by `#[auto_enum]`, not a value.
//!
//! ```rust
//! # use auto_enums::auto_enum;
//! #[auto_enum(Iterator)]
//! fn foo(x: i32) -> impl Iterator<Item = i32> {
//!     match x {
//!         0 => 1..10,
//!         1 => panic!(), // variant assignment is skipped
//!         _ => vec![5, 10].into_iter(),
//!     }
//! }
//! # fn main() { let _ = foo(0); }
//! ```
//!
//! You can also skip that branch explicitly by `#[never]` attribute.
//!
//! ```rust
//! # use auto_enums::auto_enum;
//! #[auto_enum(Iterator)]
//! fn foo(x: i32) -> impl Iterator<Item = i32> {
//!     match x {
//!         0 => 1..10,
//!         #[never]
//!         1 => loop {
//!             panic!()
//!         },
//!         _ => vec![5, 10].into_iter(),
//!     }
//! }
//! # fn main() { let _ = foo(0); }
//! ```
//!
//! ### Expression level marker (`marker!` macro)
//!
//! `#[auto_enum]` replaces `marker!` macros with variants.
//! If values of two or more are specified by `marker!` macros, `#[auto_enum]`
//! can be used for unsupported expressions and statements.
//!
//! ```rust
//! # use auto_enums::auto_enum;
//! #[auto_enum(Iterator)]
//! fn foo(x: i32) -> impl Iterator<Item = i32> {
//!     if x < 0 {
//!         return x..=0;
//!     }
//!     marker!(1..10)
//! }
//! # fn main() { let _ = foo(0); }
//! ```
//!
//! The default name of the macro is `"marker"`, but you can change it by
//! `marker` option.
//!
//! ```rust
//! # use auto_enums::auto_enum;
//! #[auto_enum(marker = bar, Iterator)]
//! fn foo(x: i32) -> impl Iterator<Item = i32> {
//!     if x < 0 {
//!         return x..=0;
//!     }
//!     bar!(1..10)
//! }
//! # fn main() { let _ = foo(0); }
//! ```
//!
//! ## Rust Nightly
//!
//! When using `#[auto_enum]` for expressions and statements, `#[auto_enum]` for
//! function is unnecessary.
//!
//! ```rust
//! // Add this to your crate root:
//! #![feature(proc_macro_hygiene, stmt_expr_attributes)]
//! # fn main() {}
//! ```
//!
//! ```rust
//! # #![feature(proc_macro_hygiene, stmt_expr_attributes)]
//! # use auto_enums::auto_enum;
//! fn foo(x: i32) -> i32 {
//!     #[auto_enum(Iterator)]
//!     let iter = match x {
//!         0 => 1..10,
//!         _ => vec![5, 10].into_iter(),
//!     };
//!
//!     iter.fold(0, |sum, x| sum + x)
//! }
//! # fn main() { let _ = foo(0); }
//! ```
//!
//! You can also return closures.
//!
//! ```rust
//! // Add this to your crate root:
//! #![feature(fn_traits, unboxed_closures)]
//! # fn main() {}
//! ```
//!
//! ```rust
//! # #![feature(fn_traits, unboxed_closures)]
//! # use auto_enums::auto_enum;
//! #[auto_enum(Fn)]
//! fn foo(x: bool) -> impl Fn(i32) -> i32 {
//!     if x { |y| y + 1 } else { |z| z - 1 }
//! }
//! # fn main() { let _ = foo(false); }
//! ```
//!
//! ## `#[enum_derive]`
//!
//! `#[enum_derive]` implements the supported traits and passes unsupported
//! traits to `#[derive]`.
//!
//! If you want to use traits that are not supported by `#[enum_derive]`, you
//! can use another crate that provides `proc_macro_derive`, or you can define
//! `proc_macro_derive` yourself([derive_utils] probably can help it).
//!
//! Basic usage of `#[enum_derive]`
//!
//! ```rust
//! use auto_enums::enum_derive;
//!
//! // `#[enum_derive]` implements `Iterator`, and `#[derive]` implements `Clone`.
//! #[enum_derive(Iterator, Clone)]
//! enum Foo<A, B> {
//!     A(A),
//!     B(B),
//! }
//! # fn main() { let _: Foo<i32, i32> = Foo::A(0); }
//! ```
//!
//! `#[enum_derive]` adds the dependency of the specified trait if it is not
//! specified.
//!
//! ```rust
//! use auto_enums::enum_derive;
//!
//! // `#[enum_derive]` implements `Iterator` and `ExactSizeIterator`.
//! #[enum_derive(ExactSizeIterator)]
//! enum Foo<A, B> {
//!     A(A),
//!     B(B),
//! }
//! # fn main() { let _: Foo<i32, i32> = Foo::A(0); }
//! ```
//!
//! [derive_utils]: https://github.com/taiki-e/derive_utils
//!
//! ## Supported traits
//!
//! Some traits support is disabled by default.
//! Note that some traits have aliases.
//!
//! *When using features that depend on unstable APIs, the `unstable` feature must be explicitly enabled*
//!
//! ### [std|core] libraries
//!
//! `[std|core]::iter`
//!
//! * [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/std/iter/iterator.md)
//! * [`DoubleEndedIterator`](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/std/iter/DoubleEndedIterator.md)
//! * [`ExactSizeIterator`](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/std/iter/ExactSizeIterator.md)
//! * [`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/std/iter/FusedIterator.md)
//! * [`Extend`](https://doc.rust-lang.org/std/iter/trait.Extend.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/std/iter/extend.md)
//! * [`TrustedLen`](https://doc.rust-lang.org/std/iter/trait.TrustedLen.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/std/iter/TrustedLen.md) *(requires `"trusted_len"` and `"unstable"` crate features)*
//!
//! `[std|core]::future`
//!
//! * [`Future`](https://doc.rust-lang.org/nightly/std/future/trait.Future.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/std/future.md)
//!
//! `std::io` *(requires `"std"` crate feature)*
//!
//! * [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) (alias: `io::Read`) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/std/io/read.md)
//! * [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html) (alias: `io::BufRead`) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/std/io/BufRead.md)
//! * [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) (alias: `io::Write`) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/std/io/write.md)
//! * [`Seek`](https://doc.rust-lang.org/std/io/trait.Seek.html) (alias: `io::Seek`) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/std/io/seek.md)
//!
//! `[std|core]::ops`
//!
//! * [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html) *(requires `"ops"` crate feature)*
//! * [`DerefMut`](https://doc.rust-lang.org/std/ops/trait.DerefMut.html) *(requires `"ops"` crate feature)*
//! * [`Index`](https://doc.rust-lang.org/std/ops/trait.Index.html) *(requires `"ops"` crate feature)*
//! * [`IndexMut`](https://doc.rust-lang.org/std/ops/trait.IndexMut.html) *(requires `"ops"` crate feature)*
//! * [`RangeBounds`](https://doc.rust-lang.org/std/ops/trait.RangeBounds.html) *(requires `"ops"` crate feature)*
//! * [`Fn`](https://doc.rust-lang.org/std/ops/trait.Fn.html) *(requires `"fn_traits"` and `"unstable"` crate features)*
//! * [`FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html) *(requires `"fn_traits"` and `"unstable"` crate features)*
//! * [`FnOnce`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html) *(requires `"fn_traits"` and `"unstable"` crate features)*
//! * [`Generator`](https://doc.rust-lang.org/nightly/std/ops/trait.Generator.html) *(requires `"generator_trait"` and `"unstable"` crate features)*
//!
//! `[std|core]::convert`
//!
//! * [`AsRef`](https://doc.rust-lang.org/std/convert/trait.AsRef.html) *(requires `"convert"` crate feature)*
//! * [`AsMut`](https://doc.rust-lang.org/std/convert/trait.AsMut.html) *(requires `"convert"` crate feature)*
//!
//! `[std|core]::fmt`
//!
//! * [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) (alias: `fmt::Debug`) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/std/debug.md)
//! * [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) (alias: `fmt::Display`)
//! * [`fmt::Binary`](https://doc.rust-lang.org/std/fmt/trait.Binary.html) *(requires `"fmt"` crate feature)*
//! * [`fmt::LowerExp`](https://doc.rust-lang.org/std/fmt/trait.LowerExp.html) *(requires `"fmt"` crate feature)*
//! * [`fmt::LowerHex`](https://doc.rust-lang.org/std/fmt/trait.LowerHex.html) *(requires `"fmt"` crate feature)*
//! * [`fmt::Octal`](https://doc.rust-lang.org/std/fmt/trait.Octal.html) *(requires `"fmt"` crate feature)*
//! * [`fmt::Pointer`](https://doc.rust-lang.org/std/fmt/trait.Pointer.html) *(requires `"fmt"` crate feature)*
//! * [`fmt::UpperExp`](https://doc.rust-lang.org/std/fmt/trait.UpperExp.html) *(requires `"fmt"` crate feature)*
//! * [`fmt::UpperHex`](https://doc.rust-lang.org/std/fmt/trait.UpperHex.html) *(requires `"fmt"` crate feature)*
//! * [`fmt::Write`](https://doc.rust-lang.org/std/fmt/trait.Write.html)
//!
//! `std::error` *(requires `"std"` crate feature)*
//!
//! * [`Error`](https://doc.rust-lang.org/std/error/trait.Error.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/std/error.md)
//!
//! ### External libraries
//!
//! You can add support for external library by activating the each crate feature.
//!
//! [`futures(v0.3)`](https://github.com/rust-lang-nursery/futures-rs) *(requires `"futures"` and `"unstable"` crate feature)*
//!
//! * [`futures::Stream`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/stream/trait.Stream.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/external/futures/stream.md)
//! * [`futures::Sink`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/sink/trait.Sink.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/external/futures/sink.md)
//! * [`futures::AsyncRead`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/io/trait.AsyncRead.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/external/futures/AsyncRead.md)
//! * [`futures::AsyncWrite`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/io/trait.AsyncWrite.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/external/futures/AsyncWrite.md)
//! * [`futures::AsyncSeek`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/io/trait.AsyncSeek.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/external/futures/AsyncSeek.md)
//! * [`futures::AsyncBufRead`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/io/trait.AsyncBufRead.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/external/futures/AsyncBufRead.md)
//!
//! [`futures(v0.1)`](https://github.com/rust-lang-nursery/futures-rs) *(requires `"futures01"` crate feature)*
//!
//! * [`futures01::Future`](https://docs.rs/futures/0.1/futures/future/trait.Future.html)
//! * [`futures01::Stream`](https://docs.rs/futures/0.1/futures/stream/trait.Stream.html)
//! * [`futures01::Sink`](https://docs.rs/futures/0.1/futures/sink/trait.Sink.html)
//!
//! [`rayon`](https://github.com/rayon-rs/rayon) *(requires `"rayon"` crate feature)*
//!
//! * [`rayon::ParallelIterator`](https://docs.rs/rayon/1.0/rayon/iter/trait.ParallelIterator.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/external/rayon/ParallelIterator.md)
//! * [`rayon::IndexedParallelIterator`](https://docs.rs/rayon/1.0/rayon/iter/trait.IndexedParallelIterator.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/external/rayon/IndexedParallelIterator.md)
//! * [`rayon::ParallelExtend`](https://docs.rs/rayon/1.0/rayon/iter/trait.ParallelExtend.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/external/rayon/ParallelExtend.md)
//!
//! [`serde`](https://github.com/serde-rs/serde) *(requires `"serde"` crate feature)*
//!
//! * [`serde::Serialize`](https://docs.serde.rs/serde/trait.Serialize.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/external/serde/serialize.md)
//!
//! ### Static methods
//!
//! These don't derive traits, but derive static methods instead.
//!
//! * `Transpose` *(requires `"transpose_methods"` crate feature)* - this derives the following conversion methods.
//!
//!   * `transpose` - convert from `enum<Option<T1>,..>` to `Option<enum<T1,..>>`
//!
//!   * `transpose` - convert from `enum<Result<T1, E1>,..>` to `Result<enum<T1,..>, enum<E1,..>>`
//!
//!   * `transpose_ok` - convert from `enum<Result<T1, E>,..>` to `Option<enum<T1,..>, E>`
//!
//!     Examples:
//!
//!     ```rust
//!     # #[cfg(feature = "transpose_methods")]
//!     # use auto_enums::auto_enum;
//!     # #[cfg(feature = "transpose_methods")]
//!     use std::{fs, io, path::Path};
//!
//!     # #[cfg(feature = "transpose_methods")]
//!     #[auto_enum(Transpose, Write)]
//!     fn output_stream(file: Option<&Path>) -> io::Result<impl io::Write> {
//!         match file {
//!             Some(f) => fs::File::create(f),
//!             None => Ok(io::stdout()),
//!         }.transpose_ok()
//!     }
//!     # #[cfg(feature = "transpose_methods")]
//!     # fn main() { let _ = output_stream(None); }
//!     # #[cfg(not(feature = "transpose_methods"))]
//!     # fn main() {}
//!     ```
//!
//!   * `transpose_err` - convert from `enum<Result<T, E1>,..>` to `Result<T, enum<E1,..>>`
//!
//! ## Crate Features
//!
//! * `std`
//!   * Enabled by default.
//!   * Enable to use `std` library's traits.
//!
//! * `ops`
//!   * Disabled by default.
//!   * Enable to use `[std|core]::ops`'s `Deref`, `DerefMut`, `Index`, `IndexMut`, and `RangeBounds` traits.
//!
//! * `convert`
//!   * Disabled by default.
//!   * Enable to use `[std|core]::convert`'s `AsRef` and `AsMut` traits.
//!
//! * `fmt`
//!   * Disabled by default.
//!   * Enable to use `[std|core]::fmt`'s traits other than `Debug`, `Display` and `Write`.
//!
//! * `type_analysis`
//!   * Disabled by default.
//!   * Analyze return type of function and `let` binding.
//!
//!     **Note that this feature is still experimental.**
//!
//!     Examples:
//!
//!     ```rust
//!     # #[cfg(feature = "type_analysis")]
//!     # use auto_enums::auto_enum;
//!     # #[cfg(feature = "type_analysis")]
//!     #[auto_enum] // there is no need to specify std library's traits
//!     fn foo(x: i32) -> impl Iterator<Item = i32> {
//!         match x {
//!             0 => 1..10,
//!             _ => vec![5, 10].into_iter(),
//!         }
//!     }
//!     # #[cfg(feature = "type_analysis")]
//!     # fn main() { let _ = foo(0); }
//!     # #[cfg(not(feature = "type_analysis"))]
//!     # fn main() {}
//!     ```
//!
//!     Please be careful if you return another traits with the same name.
//!
//! * `transpose_methods`
//!   * Disabled by default.
//!   * Enable to use `transpose*` methods.
//!
//! ### Using external libraries (disabled by default)
//!
//! * `futures` - [futures(v0.3)](https://github.com/rust-lang-nursery/futures-rs) *(requires `"unstable"` crate feature)*
//!
//! * `futures01` - [futures(v0.1)](https://github.com/rust-lang-nursery/futures-rs)
//!
//! * `rayon` - [rayon](https://github.com/rayon-rs/rayon)
//!
//! * `serde` - [serde](https://github.com/serde-rs/serde)
//!
//! ### Enable unstable features of [std|core] libraries (disabled by default, requires `"unstable"` crate feature)
//!
//! For these features, you need to enable the unstable feature gate of the same name.
//!
//! Note that support for these features are unstable and may cause incompatible changes between patch versions.
//!
//! * [`generator_trait`](https://github.com/rust-lang/rust/issues/43122) - Enable to use `[std|core]::ops::Generator` trait.
//!
//! * [`fn_traits`](https://github.com/rust-lang/rust/issues/29625) - Enable to use `[std|core]::ops`'s `Fn`, `FnMut`, and `FnOnce` traits.
//!
//! * [`trusted_len`](https://github.com/rust-lang/rust/issues/37572) - Enable to use `[std|core]::iter::TrustedLen` trait.
//!
//! ## Known limitations
//!
//! * There needs to explicitly specify the trait to be implemented (`type_analysis` crate feature reduces this limitation).
//!
//! * There needs to be marker macros for unsupported expressions.

#![no_std]
#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/auto_enums/0.6.4")]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms, single_use_lifetimes), allow(dead_code))
))]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes, unreachable_pub)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::use_self)]

#[cfg(all(feature = "futures", not(feature = "unstable")))]
compile_error!(
    "The `futures` feature requires the `unstable` feature as an explicit opt-in to unstable features"
);

#[cfg(all(feature = "generator_trait", not(feature = "unstable")))]
compile_error!(
    "The `generator_trait` feature requires the `unstable` feature as an explicit opt-in to unstable features"
);

#[cfg(all(feature = "fn_traits", not(feature = "unstable")))]
compile_error!(
    "The `fn_traits` feature requires the `unstable` feature as an explicit opt-in to unstable features"
);

#[cfg(all(feature = "trusted_len", not(feature = "unstable")))]
compile_error!(
    "The `trusted_len` feature requires the `unstable` feature as an explicit opt-in to unstable features"
);

#[doc(hidden)]
pub use auto_enums_core::auto_enum;
#[doc(hidden)]
pub use auto_enums_derive::enum_derive;
