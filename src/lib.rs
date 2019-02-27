//! A library for to allow multiple return types by automatically generated enum.
//!
//! This library provides the following attribute macros:
//!
//! * `#[auto_enum]`
//!
//!   Parses syntax, creates the enum, inserts variants, and passes specified traits to `#[enum_derive]`.
//!
//! * `#[enum_derive]`
//!
//!   Implements specified traits to the enum.
//!
//! ## `#[auto_enum]`
//!
//! `#[auto_enum]`'s most basic feature is to wrap the value returned by the last if or match expression by an enum that implemented the specified traits.
//!
//! ```rust
//! # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//! # #[macro_use]
//! # extern crate auto_enums;
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
//! # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//! # #[macro_use]
//! # extern crate auto_enums;
//! fn foo(x: i32) -> impl Iterator<Item = i32> {
//!     #[enum_derive(Iterator)]
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
//! # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//! # extern crate auto_enums;
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
//!
//! ### Positions where `#[auto_enum]` can be used.
//!
//! `#[auto_enum]` can be used in the following three places. However, since [stmt_expr_attributes](https://github.com/rust-lang/rust/issues/15701) and [proc_macro_hygiene](https://github.com/rust-lang/rust/issues/54727) are not stabilized, you need to use empty `#[auto_enum]` for functions except nightly.
//!
//! * functions
//!
//!   ```rust
//!   # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//!   # #[macro_use]
//!   # extern crate auto_enums;
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
//!   # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//!   # #[macro_use]
//!   # extern crate auto_enums;
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
//!   # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//!   # #[macro_use]
//!   # extern crate auto_enums;
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
//!   # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//!   # #[macro_use]
//!   # extern crate auto_enums;
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
//!   Wrap each `break` with a variant. Nested loops and labeled `break` are also supported.
//!
//!   ```rust
//!   # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//!   # #[macro_use]
//!   # extern crate auto_enums;
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
//!   # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//!   # #[macro_use]
//!   # extern crate auto_enums;
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
//!   # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//!   # #[macro_use]
//!   # extern crate auto_enums;
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
//!   # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//!   # #[macro_use]
//!   # extern crate auto_enums;
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
//!   When "try_trait" crate feature is enabled, `?` operator is expanded as follows (note that this uses [an unstable feature](https://github.com/rust-lang/rust/issues/42327)):
//!
//!   ```rust
//!   # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//!   # #[cfg(feature = "try_trait")]
//!   # use std::ops::Try;
//!   # #[cfg(feature = "try_trait")]
//!   # pub enum Enum<A> { Veriant(A) }
//!   # #[cfg(feature = "try_trait")]
//!   # pub fn a<T, E>(expr: Result<T, E>) -> Result<T, Enum<E>> {
//!   # Ok(
//!   match Try::into_result(expr) {
//!       Ok(val) => val,
//!       Err(err) => return Try::from_error(Enum::Veriant(err)),
//!   }
//!   # )
//!   # }
//!   # fn main() {}
//!   ```
//!
//! * `?` operator (in closures)
//!
//!   `#[auto_enum]` can parse the `?` operator in the scope.
//!
//!   However, `#[auto_enum]` must be used directly for that closure (or the let binding of the closure).
//!
//!   ```rust
//!   # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//!   # #[macro_use]
//!   # extern crate auto_enums;
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
//!           // The last branch of the function is not parsed.
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
//!   The following expressions are recursively searched until an `if`, `match`, `loop` or unsupported expression is found.
//!
//!   * blocks
//!   * unsafe blocks
//!   * method calls
//!   * parentheses
//!   * type ascriptions
//!
//!   ```rust
//!   # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//!   # #[macro_use]
//!   # extern crate auto_enums;
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
//! ### Parse nested branches
//!
//! You can parse nested branches by `#[rec]` attribute.
//!
//! ```rust
//! # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//! # #[macro_use]
//! # extern crate auto_enums;
//! #[auto_enum(Iterator)]
//! fn foo(x: i32) -> impl Iterator<Item = i32> {
//!     match x {
//!         0 => 1..10,
//!         #[rec]
//!         _ => match x {
//!             1 => vec![5, 10].into_iter(),
//!             _ => 0..=x,
//!         },
//!     }
//! }
//! # fn main() { let _ = foo(0); }
//! ```
//!
//! ### Expression that no value will be returned
//!
//! If the last expression of a branch is one of the following, it is interpreted that no value will be returned (variant assignment is skipped).
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
//! ```rust
//! # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//! # #[macro_use]
//! # extern crate auto_enums;
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
//! # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//! # #[macro_use]
//! # extern crate auto_enums;
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
//! You can also skip all branches by `never` option.
//!
//! ```rust
//! # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//! # #[macro_use]
//! # extern crate auto_enums;
//! #[auto_enum(never, Iterator)]
//! fn foo(x: i32) -> impl Iterator<Item = i32> {
//!     match x {
//!         0 => loop {
//!             return marker!(1..10);
//!         },
//!         1 => loop {
//!             panic!()
//!         },
//!         _ => loop {
//!             return marker!(vec![5, 10].into_iter());
//!         },
//!     }
//! }
//! # fn main() { let _ = foo(0); }
//! ```
//!
//! ### Expression level marker (`marker!` macro)
//!
//! `#[auto_enum]` replaces `marker!` macros with variants.
//! If values of two or more are specified by `marker!` macros, `#[auto_enum]` can be used for unsupported expressions and statements.
//!
//! ```rust
//! # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//! # #[macro_use]
//! # extern crate auto_enums;
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
//! The default name of the macro is `"marker"`, but you can change it by `marker` option.
//!
//! ```rust
//! # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//! # #[macro_use]
//! # extern crate auto_enums;
//! #[auto_enum(marker(bar), Iterator)]
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
//! When using `#[auto_enum]` for expressions and statements, `#[auto_enum]` for function is unnecessary.
//!
//! ```rust,ignore
//! # #![cfg(feature = "unstable")]
//! // Add this to your crate root:
//! #![feature(proc_macro_hygiene, stmt_expr_attributes)]
//! # fn main() {}
//! ```
//!
//! ```rust
//! # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//! # #![cfg_attr(feature = "unstable", feature(proc_macro_hygiene, stmt_expr_attributes))]
//! # #[cfg(feature = "unstable")]
//! # #[macro_use]
//! # extern crate auto_enums;
//! # #[cfg(feature = "unstable")]
//! fn foo(x: i32) -> i32 {
//!     #[auto_enum(Iterator)]
//!     let iter = match x {
//!         0 => 1..10,
//!         _ => vec![5, 10].into_iter(),
//!     };
//!
//!     iter.fold(0, |sum, x| sum + x)
//! }
//! # #[cfg(feature = "unstable")]
//! # fn main() { let _ = foo(0); }
//! # #[cfg(not(feature = "unstable"))]
//! # fn main() {}
//! ```
//!
//! You can also return closures.
//!
//! ```rust,ignore
//! # #![cfg(feature = "unstable")]
//! // Add this to your crate root:
//! #![feature(fn_traits, unboxed_closures)]
//! # fn main() {}
//! ```
//!
//! ```rust
//! # #![cfg_attr(feature = "unstable", feature(fn_traits, unboxed_closures))]
//! # #[cfg(feature = "unstable")]
//! # #[macro_use]
//! # extern crate auto_enums;
//! # #[cfg(feature = "unstable")]
//! #[auto_enum(Fn)]
//! fn foo(x: bool) -> impl Fn(i32) -> i32 {
//!     if x {
//!         |y| y + 1
//!     } else {
//!         |z| z - 1
//!     }
//! }
//! # #[cfg(feature = "unstable")]
//! # fn main() { let _ = foo(false); }
//! # #[cfg(not(feature = "unstable"))]
//! # fn main() {}
//! ```
//!
//! ## `#[enum_derive]`
//!
//! `#[enum_derive]` implements the supported traits and passes unsupported traits to `#[derive]`.
//!
//! If you want to use traits that are not supported by `#[enum_derive]`, you can use another crate that provides `proc_macro_derive`, or you can define `proc_macro_derive` yourself([derive_utils] probably can help it).
//!
//! Basic usage of `#[enum_derive]`
//!
//! ```rust
//! # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//! # #[macro_use]
//! # extern crate auto_enums;
//! // `#[enum_derive]` implements `Iterator`, and `#[derive]` implements `Clone`.
//! #[enum_derive(Iterator, Clone)]
//! enum Foo<A, B> {
//!     A(A),
//!     B(B),
//! }
//! # fn main() { let _: Foo<i32, i32> = Foo::A(0); }
//! ```
//!
//! `#[enum_derive]` adds the dependency of the specified trait if it is not specified.
//!
//! ```rust
//! # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//! # #![cfg_attr(feature = "exact_size_is_empty", feature(exact_size_is_empty))]
//! # #[macro_use]
//! # extern crate auto_enums;
//! // `#[enum_derive]` implements `Iterator` and `ExactSizeIterator`.
//! #[enum_derive(ExactSizeIterator)]
//! enum Foo<A, B> {
//!     A(A),
//!     B(B),
//! }
//! # fn main() { let _: Foo<i32, i32> = Foo::A(0); }
//! ```
//!
//! [derive_utils]: https://crates.io/crates/derive_utils
//!
//! ## Supported traits
//!
//! ### [std|core] libraries
//!
//! Note that some traits have aliases.
//!
//! `[std|core]::ops`
//!
//! * [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html)
//! * [`DerefMut`](https://doc.rust-lang.org/std/ops/trait.DerefMut.html)
//! * [`Index`](https://doc.rust-lang.org/std/ops/trait.Index.html)
//! * [`IndexMut`](https://doc.rust-lang.org/std/ops/trait.IndexMut.html)
//! * [`RangeBounds`](https://doc.rust-lang.org/std/ops/trait.RangeBounds.html)
//! * [`Fn`](https://doc.rust-lang.org/std/ops/trait.Fn.html) (*nightly-only*)
//! * [`FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html) (*nightly-only*)
//! * [`FnOnce`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html) (*nightly-only*)
//! * [`Generator`](https://doc.rust-lang.org/nightly/std/ops/trait.Generator.html) (*nightly-only*)
//!
//! `[std|core]::convert`
//!
//! * [`AsRef`](https://doc.rust-lang.org/std/convert/trait.AsRef.html)
//! * [`AsMut`](https://doc.rust-lang.org/std/convert/trait.AsMut.html)
//!
//! `[std|core]::iter`
//!
//! * [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
//! * [`DoubleEndedIterator`](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html)
//! * [`ExactSizeIterator`](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html)
//! * [`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html)
//! * [`TrustedLen`](https://doc.rust-lang.org/std/iter/trait.TrustedLen.html) (*nightly-only*)
//! * [`Extend`](https://doc.rust-lang.org/std/iter/trait.Extend.html)
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
//! `[std|core]::future`
//!
//! * [`Future`](https://doc.rust-lang.org/nightly/std/future/trait.Future.html) - *nightly-only* - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/std/future.md)
//!
//! `std::io` *(requires `"std"` crate feature)*
//!
//! * [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) (alias: `io::Read`)
//! * [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html) (alias: `io::BufRead`)
//! * [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) (alias: `io::Write`)
//! * [`Seek`](https://doc.rust-lang.org/std/io/trait.Seek.html) (alias: `io::Seek`)
//!
//! `std::error` *(requires `"std"` crate feature)*
//!
//! * [`Error`](https://doc.rust-lang.org/std/error/trait.Error.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/std/error.md)
//!
//! ### External libraries
//!
//! You can add support for external library by activating the each crate feature.
//!
//! [`futures(v0.3)`](https://github.com/rust-lang-nursery/futures-rs) *(requires `"futures"` crate feature)*
//!
//! * [`futures::Stream`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/stream/trait.Stream.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/external/futures/stream.md)
//! * [`futures::Sink`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/sink/trait.Sink.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/external/futures/sink.md)
//! * [`futures::AsyncRead`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/io/trait.AsyncRead.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/external/futures/async_read.md)
//! * [`futures::AsyncWrite`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/io/trait.AsyncWrite.html) - [generated code](https://github.com/taiki-e/auto_enums/blob/master/docs/supported_traits/external/futures/async_write.md)
//!
//! [`futures(v0.1)`](https://github.com/rust-lang-nursery/futures-rs) *(requires `"futures01"` crate feature)*
//!
//! * [`futures01::Future`](https://docs.rs/futures/0.1/futures/future/trait.Future.html)
//! * [`futures01::Stream`](https://docs.rs/futures/0.1/futures/stream/trait.Stream.html)
//! * [`futures01::Sink`](https://docs.rs/futures/0.1/futures/sink/trait.Sink.html)
//!
//! [`quote`](https://github.com/dtolnay/quote) *(requires `"proc_macro"` crate feature)*
//!
//! * [`quote::ToTokens`](https://docs.rs/quote/0.6/quote/trait.ToTokens.html)
//!
//! [`rayon`](https://github.com/rayon-rs/rayon) *(requires `"rayon"` crate feature)*
//!
//! * [`rayon::ParallelIterator`](https://docs.rs/rayon/1.0/rayon/iter/trait.ParallelIterator.html)
//! * [`rayon::IndexedParallelIterator`](https://docs.rs/rayon/1.0/rayon/iter/trait.IndexedParallelIterator.html)
//! * [`rayon::ParallelExtend`](https://docs.rs/rayon/1.0/rayon/iter/trait.ParallelExtend.html)
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
//!     # #[macro_use]
//!     # extern crate auto_enums;
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
//! * `fmt`
//!   * Disabled by default.
//!   * Enable to use `[std|core]::fmt`'s traits other than `Debug`, `Display` and `Write`.
//!
//! * `type_analysis`
//!   * Disabled by default.
//!   * Analyze return type of function and `let` binding.
//!
//!     Examples:
//!
//!     ```rust
//!     # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//!     # #[cfg(feature = "type_analysis")]
//!     # #[macro_use]
//!     # extern crate auto_enums;
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
//! * `try_trait`
//!   * Disabled by default.
//!   * Make `?` operator support more flexible, and to make iterator implementation more effective.
//!   * This requires Rust Nightly and you need to enable the unstable [`try_trait`](https://github.com/rust-lang/rust/issues/42327) feature gate.
//!
//! ### Using external libraries (disabled by default)
//!
//! * `futures` - [futures(v0.3)](https://github.com/rust-lang-nursery/futures-rs)
//!
//! * `futures01` - [futures(v0.1)](https://github.com/rust-lang-nursery/futures-rs)
//!
//! * `proc_macro` - [quote](https://github.com/dtolnay/quote)
//!
//! * `rayon` - [rayon](https://github.com/rayon-rs/rayon)
//!
//! * `serde` - [serde](https://github.com/serde-rs/serde)
//!
//! ### Enable unstable features of [std|core] libraries (disabled by default, nightly-only)
//!
//! For these features, you need to enable the unstable feature gate of the same name.
//!
//! * [`exact_size_is_empty`](https://github.com/rust-lang/rust/issues/35428) - Implements `ExactSizeIterator::is_empty`.
//!
//! * [`read_initializer`](https://github.com/rust-lang/rust/issues/42788) - Implements `io::Read::read_initializer`.
//!
//! * [`try_trait`](https://github.com/rust-lang/rust/issues/42327) - Make iterator implementation more effective.
//!
//! * [`unsized_locals`](https://github.com/rust-lang/rust/issues/48055) - Allow `Index<Idx: ?Sized>` and `IndexMut<Idx: ?Sized>`.
//!
//! ## Known limitations
//!
//! * There needs to explicitly specify the trait to be implemented (`type_analysis` crate feature reduces this limitation).
//!
//! * There needs to be marker macros for unsupported expressions.
//!

#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/auto_enums/0.4.1")]
#![no_std]

#[doc(hidden)]
pub use auto_enums_core::auto_enum;
#[doc(hidden)]
pub use auto_enums_derive::enum_derive;
