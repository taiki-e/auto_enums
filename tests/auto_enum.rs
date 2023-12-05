// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg_attr(
    feature = "fn_traits",
    feature(proc_macro_hygiene, stmt_expr_attributes, type_ascription)
)]
#![cfg_attr(feature = "coroutine_trait", feature(coroutine_trait))]
#![cfg_attr(feature = "fn_traits", feature(fn_traits, unboxed_closures))]
#![cfg_attr(feature = "trusted_len", feature(trusted_len))]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(
    dead_code,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::items_after_statements,
    clippy::let_and_return,
    clippy::needless_return,
    clippy::never_loop,
    clippy::no_effect_underscore_binding,
    clippy::undocumented_unsafe_blocks,
    clippy::unnecessary_wraps
)]

use core::iter;

use auto_enums::auto_enum;

#[test]
fn stable() {
    const ANS: &[i32] = &[28, 3];

    #[auto_enum(Iterator)]
    fn match_(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..8,
            n if n > 3 => 2..=10,
            _ => (0..2).map(|x| x + 1),
        }
    }
    for (i, x) in ANS.iter().enumerate() {
        assert_eq!(match_(i).sum::<i32>(), *x);
    }

    // block + unsafe block + parentheses
    #[rustfmt::skip]
    #[allow(unused_parens, unused_braces, unused_unsafe)]
    #[auto_enum(Iterator)]
    fn block(x: usize) -> impl Iterator<Item = i32> {
        {{({ unsafe {{({ unsafe { unsafe {{
            match x {
                0 => 1..8,
                n if n > 3 => 2..=10,
                _ => (0..2).map(|x| x + 1),
            }
        }}}})}}})}}
    }
    for (i, x) in ANS.iter().enumerate() {
        assert_eq!(block(i).sum::<i32>(), *x);
    }

    #[auto_enum(Iterator)]
    fn if_(x: usize) -> impl Iterator<Item = i32> {
        if x == 0 {
            1..8
        } else if x > 3 {
            2..=10
        } else {
            (0..2).map(|x| x + 1)
        }
    }
    for (i, x) in ANS.iter().enumerate() {
        assert_eq!(if_(i).sum::<i32>(), *x);
    }

    #[auto_enum(Iterator)]
    fn method_call(x: usize) -> impl Iterator<Item = i32> {
        if x == 0 {
            1..8
        } else if x > 3 {
            2..=10
        } else {
            (0..2).map(|x| x + 1)
        }
        .map(|x| x + 1)
        .map(|x| x - 1)
    }
    for (i, x) in ANS.iter().enumerate() {
        assert_eq!(method_call(i).sum::<i32>(), *x);
    }

    #[auto_enum(Iterator)]
    fn no_return(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..8,
            3 => panic!(),
            _ => (0..2).map(|x| x + 1),
        }
    }
    for (i, x) in ANS.iter().enumerate() {
        assert_eq!(no_return(i).sum::<i32>(), *x);
    }

    #[auto_enum(Iterator)]
    fn no_return2(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..8,
            3 => match x {
                0 => panic!(),
                1..=3 => panic!(),
                _ => unreachable!(),
            },
            _ => (0..2).map(|x| x + 1),
        }
    }
    for (i, x) in ANS.iter().enumerate() {
        assert_eq!(no_return2(i).sum::<i32>(), *x);
    }

    #[auto_enum(Iterator)]
    fn no_return3(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..8,
            3 => match x {
                0 => panic!(),
                1..=3 => (1..4).map(|x| x + 1),
                _ => unreachable!(),
            },
            _ => (0..2).map(|x| x + 1),
        }
    }
    for (i, x) in ANS.iter().enumerate() {
        assert_eq!(no_return3(i).sum::<i32>(), *x);
    }
    #[auto_enum(Iterator)]
    fn no_return4(i: usize) -> impl Iterator<Item = i32> {
        if i == 0 {
            1..8
        } else if i > 6 {
            panic!();
        } else if i > 5 {
            std::panic!("5");
        } else if i > 4 {
            core::panic!("4");
        } else if i > 3 {
            unreachable!("3");
        } else if i > 2 {
            std::unreachable!("2");
        } else if i > 1 {
            core::unreachable!();
        } else {
            (0..2).map(|x| x + 1)
        }
    }
    for (i, x) in ANS.iter().enumerate() {
        assert_eq!(no_return4(i).sum::<i32>(), *x);
    }

    #[auto_enum(fmt::Debug)]
    fn no_return5(x: usize) -> impl core::fmt::Debug {
        match x {
            0 => 1..8,
            3 => {}
            _ => 0..=2,
        }
    }

    #[auto_enum(Iterator)]
    fn return1(x: usize) -> impl Iterator<Item = i32> {
        if x > 10 {
            return (0..x as _).map(|x| x - 1);
        }
        if x == 0 {
            1..8
        } else if x > 3 {
            2..=10
        } else {
            (0..2).map(|x| x + 1)
        }
    }
    for (i, x) in ANS.iter().enumerate() {
        assert_eq!(return1(i).sum::<i32>(), *x);
    }

    #[auto_enum(Iterator)]
    fn break_in_loop(mut x: i32) -> impl Iterator<Item = i32> {
        loop {
            if x < 0 {
                break x..0;
            } else if x % 5 == 0 {
                break 0..=x;
            }
            x -= 1;
        }
    }
    assert_eq!(break_in_loop(14).sum::<i32>(), 55);
    assert_eq!(break_in_loop(-5).sum::<i32>(), -15);

    #[auto_enum(Iterator)]
    fn break2(mut x: i32) -> impl Iterator<Item = i32> {
        'a: loop {
            if x < 0 {
                break 'a x..0;
            } else if x % 5 == 0 {
                break 0..=x;
            }
            x -= 1;
        }
    }
    assert_eq!(break2(14).sum::<i32>(), 55);
    assert_eq!(break2(-5).sum::<i32>(), -15);

    #[auto_enum(Iterator)]
    fn break3(mut x: i32) -> impl Iterator<Item = i32> {
        'a: loop {
            if x < 0 {
                loop {
                    break 'a x..0;
                }
            } else if x % 5 == 0 {
                return loop {
                    break 0..=x;
                };
            }
            x -= 1;
        }
    }
    assert_eq!(break3(14).sum::<i32>(), 55);
    assert_eq!(break3(-5).sum::<i32>(), -15);

    #[auto_enum(Iterator)]
    fn return_in_loop(mut x: i32) -> impl Iterator<Item = i32> {
        loop {
            if x < 0 {
                return x..0;
            } else if x % 5 == 0 {
                return 0..=x;
            }
            x -= 1;
        }
    }
    assert_eq!(return_in_loop(14).sum::<i32>(), 55);
    assert_eq!(return_in_loop(-5).sum::<i32>(), -15);

    #[auto_enum(Iterator)]
    fn return2(x: i32, y: i32) -> impl Iterator<Item = i32> {
        #[auto_enum(Iterator)]
        let iter = match x {
            0 => 2..8,
            _ if y < 0 => return y..=0,
            _ => 2..=10,
        };

        match y {
            0 => iter.flat_map(|x| 0..x),
            _ => iter.map(|x| x + 1),
        }
    }
    assert_eq!(return2(10, 10).sum::<i32>(), 63);

    #[auto_enum]
    fn return3(x: i32) -> Option<impl Iterator<Item = i32>> {
        if x < 0 {
            return None;
        }

        #[auto_enum(Iterator)]
        let iter = match x {
            0 => 2..8,
            1 => None?,
            2 => std::option::Option::None?,
            3 => core::option::Option::None?,
            4 => None::<_>?,
            _ => 2..=10,
        };

        Some(iter)
    }
    assert_eq!(return3(10).unwrap().sum::<i32>(), 54);

    #[auto_enum]
    fn return4(x: i32) -> Result<impl Iterator<Item = i32>, ()> {
        if x < 0 {
            return Err(());
        }

        #[auto_enum(Iterator)]
        let iter = match x {
            0 => 2..8,
            1 => Err(())?,
            2 => std::result::Result::Err(())?,
            3 => core::result::Result::Err(())?,
            4 => Err::<_, ()>(())?,
            _ => 2..=10,
        };

        Ok(iter)
    }
    assert_eq!(return4(10).unwrap().sum::<i32>(), 54);

    #[auto_enum(Debug, Display)]
    fn try_operator1(x: i32) -> Result<impl Iterator<Item = i32>, impl core::fmt::Debug> {
        if x < 0 {
            Err(1_i32)?;
        }

        let iter = match x {
            0 => Err(())?,
            _ => 2..=10,
        };

        Ok(iter)
    }
    assert_eq!(try_operator1(10).unwrap().sum::<i32>(), 54);

    #[auto_enum(Debug)]
    fn try_operator2(
        x: i32,
    ) -> std::result::Result<impl Iterator<Item = i32>, impl core::fmt::Debug> {
        if x < 0 {
            Err(1_i32)?;
        }

        match x {
            0 => Err(())?,
            _ => Ok(2..=10),
        }
    }
    assert_eq!(try_operator2(10).unwrap().sum::<i32>(), 54);

    #[auto_enum(Debug)]
    fn try_operator3(
        x: i32,
    ) -> core::result::Result<impl Iterator<Item = i32>, impl core::fmt::Debug> {
        if x < 0 {
            Err(1_i32)?;
        }

        match x {
            0 => Err(())?,
            _ => Ok(2..=10),
        }
    }
    assert_eq!(try_operator3(10).unwrap().sum::<i32>(), 54);

    #[auto_enum]
    fn closure() {
        #[auto_enum(Iterator)]
        let f = |x| {
            if x > 10 {
                return (0..x as _).map(|x| x - 1);
            }
            if x == 0 {
                1..8
            } else if x > 3 {
                2..=10
            } else {
                (0..2).map(|x| x + 1)
            }
        };

        for (i, x) in ANS.iter().enumerate() {
            assert_eq!(f(i).sum::<i32>(), *x);
        }

        let f = {
            #[auto_enum(Iterator)]
            |x| {
                if x > 10 {
                    return (0..x as _).map(|x| x - 1);
                }
                if x == 0 {
                    1..8
                } else if x > 3 {
                    2..=10
                } else {
                    (0..2).map(|x| {
                        return x + 1;
                    })
                }
            }
        };

        for (i, x) in ANS.iter().enumerate() {
            assert_eq!(f(i).sum::<i32>(), *x);
        }
    }
    closure();

    #[cfg(feature = "transpose_methods")]
    #[auto_enum(Transpose, Iterator, Clone)]
    fn transpose(x: isize) -> Option<impl Iterator<Item = i32> + Clone> {
        match x {
            0 => Some(1..8),
            _ if x < 0 => return None,
            _ => Some(0..=10),
        }
        .transpose()
        .map(|i| i.map(|x| x + 1).map(|x| x - 1))
    }
    #[cfg(feature = "transpose_methods")]
    assert_eq!(transpose(0).unwrap().sum::<i32>(), 28);
}

#[test]
fn marker() {
    #[auto_enum(Iterator)]
    fn marker3(x: i32, y: i32) -> impl Iterator<Item = i32> {
        let iter;
        // if #[auto_enum] is used on Stmt::Semi, #[auto_enum] does not visit last expr
        #[auto_enum(Iterator)]
        match x {
            0 => iter = marker!(2..8),
            _ => iter = marker!(2..=10),
        };

        if y < 0 {
            return y..=0;
        }
        match y {
            0 => iter.flat_map(|x| 0..x),
            _ => iter.map(|x| {
                if x < 0 {
                    return x - 1;
                }
                x + 1
            }),
        }
    }
    assert_eq!(marker3(10, 10).sum::<i32>(), 63);

    #[auto_enum(marker = marker_a, Iterator)]
    fn marker4(x: i32, y: i32) -> impl Iterator<Item = i32> {
        let iter;
        #[auto_enum(Iterator)]
        match x {
            0 => iter = marker!(2..8),
            _ if y < 0 => return y..=0,
            _ => iter = marker!(2..=10),
        };

        match y {
            0 => iter.flat_map(|x| 0..x),
            _ => iter.map(|x| x + 1),
        }
    }
    assert_eq!(marker4(10, 10).sum::<i32>(), 63);

    #[auto_enum(Iterator)]
    fn marker5(x: i32, y: i32) -> impl Iterator<Item = i32> {
        let iter;
        #[auto_enum(marker = marker_a, Iterator)]
        match x {
            0 => iter = marker_a!(2..8),
            _ if y < 0 => return y..=0,
            _ => iter = marker_a!(2..=10),
        };

        match y {
            0 => iter.flat_map(|x| 0..x),
            _ => iter.map(|x| x + 1),
        }
    }
    assert_eq!(marker5(10, 10).sum::<i32>(), 63);

    #[auto_enum(Iterator, marker = foo)]
    fn marker6(x: usize) -> impl Iterator<Item = i32> {
        #[auto_enum(Iterator)]
        let _iter = match x {
            0 => 1..8,
            _ => (0..2).map(|x| x + 1),
        };

        #[auto_enum(Iterator, marker = bar)]
        let _iter = match x {
            0 => 1..8,
            1 => return foo!(1..9),
            n if n > 3 =>
            {
                #[auto_enum(Iterator, marker = baz)]
                match x {
                    0 => 1..8,
                    1 => return foo!(1..9),
                    2 => baz!(1..9),
                    n if n > 3 => 2..=10,
                    _ => (0..2).map(|x| x + 1),
                }
            }
            _ => (0..2).map(|x| x + 1),
        };

        match x {
            0 => 1..8,
            _ => (0..2).map(|x| x + 1),
        }
    }
    assert_eq!(marker6(10).sum::<i32>(), 3);
}

#[cfg(feature = "transpose_methods")]
#[cfg(feature = "std")]
#[test]
fn stable_std() {
    use std::{error::Error, fs::File, io, path::Path};

    use auto_enums::enum_derive;

    #[auto_enum(Transpose, Write)]
    fn transpose_ok(file: Option<&Path>) -> io::Result<impl io::Write> {
        if let Some(file) = file { File::create(file) } else { Ok(io::stdout()) }.transpose_ok()
    }
    assert!(transpose_ok(None).is_ok());

    #[auto_enum(Transpose, Write)]
    fn transpose_option(file: Option<&Path>) -> Option<impl io::Write> {
        if let Some(file) = file { File::create(file).ok() } else { Some(io::stdout()) }.transpose()
    }
    assert!(transpose_option(None).is_some());

    #[enum_derive(Debug, Display, Error)]
    enum IoError {
        Io(io::Error),
        Io2(io::Error),
    }

    #[auto_enum(Transpose, Write, Debug, Display, Error)]
    fn transpose_result(file: Option<&Path>) -> Result<impl io::Write, impl Error> {
        if let Some(file) = file {
            File::create(file).map_err(IoError::Io)
        } else {
            let out: Result<io::Stdout, io::Error> = Ok(io::stdout());
            out
        }
        .transpose()
    }
    assert!(transpose_result(None).is_ok());

    #[auto_enum(Transpose, Debug, Display, Error)]
    fn transpose_err(file: Option<&Path>) -> Result<(), impl Error> {
        if let Some(_f) = file {
            Err(IoError::Io2(io::Error::from(io::ErrorKind::NotFound)))
        } else {
            Err(io::Error::from(io::ErrorKind::NotFound))
        }
        .transpose_err()
    }
    assert!(transpose_err(None).unwrap_err().source().is_some());

    #[auto_enum(Debug, Display, Error)]
    fn try_operator(file: Option<&Path>) -> Result<(), impl Error> {
        if let Some(_f) = file {
            Err(IoError::Io2(io::Error::from(io::ErrorKind::NotFound)))?;
        } else {
            Err(io::Error::from(io::ErrorKind::NotFound))?;
        }
        Ok(())
    }
    assert!(try_operator(None).unwrap_err().source().is_some());
}

#[auto_enum]
fn if_attr(x: bool) -> impl Iterator<Item = u8> {
    let res = {
        #[auto_enum(Iterator)]
        if x {
            iter::once(0)
        } else {
            iter::repeat(1)
        }
    };
    res
}

#[auto_enum(Iterator)]
fn if_attr_in_if(x: usize) -> impl Iterator<Item = i32> {
    if x == 0 {
        1..8
    } else if x > 3 {
        #[nested]
        if x > 4 {
            2..=10
        } else {
            (11..20).map(|x| x - 1)
        }
    } else {
        (0..2).map(|x| x + 1)
    }
}

#[auto_enum]
fn non_stmt_expr_match1(x: bool) -> Option<impl Iterator<Item = u8>> {
    Some(
        #[auto_enum(Iterator)]
        match x {
            true => iter::once(0),
            _ => iter::repeat(1),
        },
    )
}

#[auto_enum]
fn non_stmt_expr_match2(x: bool) -> Option<impl Iterator<Item = u8>> {
    Some({
        #[auto_enum(Iterator)]
        match x {
            true => iter::once(0),
            _ => iter::repeat(1),
        }
    })
}

#[auto_enum]
fn non_stmt_expr_match3(x: bool) {
    loop {
        let _ = {
            #[auto_enum(Iterator)]
            match x {
                true => iter::once(0),
                _ => iter::repeat(1),
            }
        };
        break;
    }
}

#[auto_enum]
fn non_stmt_expr_if(x: bool) -> Option<impl Iterator<Item = u8>> {
    Some(
        #[auto_enum(Iterator)]
        if x { iter::once(0) } else { iter::repeat(1) },
    )
}

#[test]
fn nested() {
    #[auto_enum(Iterator)]
    fn match_in_match(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..8,
            #[nested]
            n if n > 3 => match x {
                2..=10 => (1..x as _).map(|x| x - 1),
                _ => 2..=10,
            },
            _ => (0..2).map(|x| x + 1),
        }
    }
    for (i, x) in [28, 3].iter().enumerate() {
        assert_eq!(match_in_match(i).sum::<i32>(), *x);
    }

    #[auto_enum(Iterator)]
    fn match_in_match_nested(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..8,
            #[nested]
            n if n > 3 => match x {
                2..=10 =>
                {
                    #[nested]
                    match n {
                        4 => (1..x as _).map(|x| x - 1),
                        _ => (1..x as _).map(|x| x + 1),
                    }
                }
                _ => 2..=10,
            },
            _ => (0..2).map(|x| x + 1),
        }
    }

    #[rustfmt::skip]
    #[allow(unused_braces, unused_unsafe)]
    #[auto_enum(Iterator)]
    fn in_block(x: usize) -> impl Iterator<Item = i32> {
        {{{ unsafe {{{ unsafe { unsafe {{
            match x {
                0 => 1..8,
                #[nested]
                n if n > 3 => {{{ unsafe {{
                    if x > 10 {
                        (-10..=x as _).map(|x| x - 4)
                    } else {
                        (1..=4).map(|x| x - 4)
                    }
                }}}}}
                _ => (0..2).map(|x| x + 1),
            }
        }}}}}}}}}
    }
    for (i, x) in [28, 3].iter().enumerate() {
        assert_eq!(in_block(i).sum::<i32>(), *x);
    }

    #[auto_enum(Iterator)]
    fn match_in_if(x: usize) -> impl Iterator<Item = i32> {
        if x == 0 {
            1..8
        } else if x > 3 {
            #[nested]
            match x {
                1..=4 => 2..=10,
                _ => (11..20).map(|x| x - 1),
            }
        } else {
            (0..2).map(|x| x + 1)
        }
    }
    for (i, x) in [28, 3].iter().enumerate() {
        assert_eq!(match_in_if(i).sum::<i32>(), *x);
    }

    #[auto_enum(Iterator)]
    fn if_in_block_if(x: usize) -> impl Iterator<Item = i32> {
        if x == 0 {
            1..8
        } else if x > 3 {
            #[nested]
            {
                if x > 4 {
                    2..=10
                } else {
                    (11..20).map(|x| x - 1)
                }
            }
        } else {
            (0..2).map(|x| x + 1)
        }
    }
    for (i, x) in [28, 3].iter().enumerate() {
        assert_eq!(if_in_block_if(i).sum::<i32>(), *x);
    }

    #[auto_enum(Iterator)]
    fn match_in_let_match(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..8,
            3 => {
                #[nested]
                let x = match x {
                    4..=10 => 2..=10,
                    _ => (11..20).map(|x| x - 1),
                };
                x
            }
            _ => (0..2).map(|x| x + 1),
        }
    }
    for (i, x) in [28, 3].iter().enumerate() {
        assert_eq!(match_in_let_match(i).sum::<i32>(), *x);
    }

    #[auto_enum(Iterator)]
    fn match_in_let_if(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..8,
            3 => {
                #[nested]
                let x = if x > 4 { 2..=10 } else { (11..20).map(|x| x - 1) };
                x
            }
            _ => (0..2).map(|x| x + 1),
        }
    }
    for (i, x) in [28, 3].iter().enumerate() {
        assert_eq!(match_in_let_if(i).sum::<i32>(), *x);
    }

    #[auto_enum(Iterator)]
    fn match_in_let_if_2(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..8,
            3 => {
                #[nested]
                let x = if x > 4 {
                    2..=10
                } else {
                    #[nested]
                    let x = match x {
                        4..=10 => 2..10,
                        _ => (11..20).map(|x| x - 1),
                    };
                    x
                };
                x
            }
            _ => (0..2).map(|x| x + 1),
        }
    }

    #[auto_enum(Iterator)]
    fn if_in_let_if(x: usize) -> impl Iterator<Item = i32> {
        if x == 0 {
            1..8
        } else if x > 3 {
            #[nested]
            let x = if x > 4 { 2..=10 } else { (11..20).map(|x| x - 1) };
            x
        } else {
            (0..2).map(|x| x + 1)
        }
    }
    for (i, x) in [28, 3].iter().enumerate() {
        assert_eq!(if_in_let_if(i).sum::<i32>(), *x);
    }

    #[auto_enum(Iterator)]
    fn if_in_let_if_nested(x: usize) -> impl Iterator<Item = i32> {
        if x == 0 {
            1..8
        } else if x > 3 {
            #[nested]
            let x = if x > 4 {
                #[nested]
                let x = if x > 4 { (2..=10).flat_map(|x| 1..x) } else { core::iter::empty() };
                x
            } else {
                (11..20).map(|x| x - 1)
            };
            x
        } else {
            (0..2).map(|x| x + 1)
        }
    }

    #[auto_enum(Iterator)]
    fn if_in_let_if_nested_nested(x: usize) -> impl Iterator<Item = i32> {
        // 1 enum with 9 variants is generated.
        if x == 0 {
            1..8
        } else if x != 1 {
            #[nested]
            let x = if x > 5 {
                #[nested]
                let x = if x > 10 {
                    #[nested]
                    let x =
                        if x > 11 { (2..=10).flat_map(|x| 1..x) } else { (11..20).map(|x| x - 1) };
                    x
                } else {
                    #[nested]
                    let x = if x > 9 {
                        (2..=10).flat_map(|x| 1..x)
                    } else {
                        #[nested]
                        let x = if x > 6 {
                            #[nested]
                            let x = if x > 7 {
                                (2..=10).flat_map(|x| 1..x)
                            } else {
                                (11..20).map(|x| x - 1)
                            };
                            x
                        } else {
                            (11..20).map(|x| x - 1)
                        };
                        x
                    };
                    x
                };
                x
            } else {
                (11..20).map(|x| x - 1)
            };
            x
        } else {
            (0..2).map(|x| x + 1)
        }
    }

    #[auto_enum(Iterator)]
    fn if_in_let_if_2(x: usize) -> impl Iterator<Item = i32> {
        if x == 0 {
            1..8
        } else if x > 3 {
            #[nested]
            let y = if x > 4 { 2..=10 } else { (11..20).map(|x| x - 1) };

            #[nested]
            let z = if x < 4 { 2..10 } else { (11..20).map(|x| x - 1) };

            if x > 5 {
                y
            } else {
                z
            }
        } else {
            (0..2).map(|x| x + 1)
        }
    }

    #[auto_enum(Iterator)]
    fn match_in_let_if_nop(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..8,
            3 => {
                #[nested]
                let x = if x > 4 { 2..=10 } else { (11..20).map(|x| x - 1) };
                #[nested] // no-op
                let _y = 111..120;
                x
            }
            _ => (0..2).map(|x| x + 1),
        }
    }

    #[auto_enum(Iterator)]
    fn if_in_let_if_nop(x: usize) -> impl Iterator<Item = i32> {
        if x == 0 {
            1..8
        } else if x > 3 {
            #[nested]
            let x = if x > 4 { 2..=10 } else { (11..20).map(|x| x - 1) };
            #[nested] // no-op
            x
        } else {
            (0..2).map(|x| x + 1)
        }
    }

    #[auto_enum(Iterator)]
    fn match_nop(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..8,
            3 => 2..=10,
            _ => (0..2).map(|x| x + 1),
        }
    }

    #[auto_enum(Iterator)]
    fn if_nop(x: usize) -> impl Iterator<Item = i32> {
        if x == 0 {
            1..8
        } else if x > 3 {
            2..=10
        } else {
            (0..2).map(|x| x + 1)
        }
    }

    #[auto_enum(Iterator)]
    fn no_return(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..8,
            #[nested]
            3 => panic!(),
            _ => (0..2).map(|x| x + 1),
        }
    }
}

// nightly
#[cfg(feature = "fn_traits")]
mod nightly {
    use auto_enums::auto_enum;

    const ANS: &[i32] = &[28, 3];

    #[test]
    fn nightly() {
        // let match
        for (i, x) in ANS.iter().enumerate() {
            #[auto_enum(Iterator)]
            let iter = match i {
                0 => 1..8,
                _ => vec![1, 2, 0].into_iter(),
            };
            assert_eq!(iter.sum::<i32>(), *x);
        }

        // let if
        for (i, x) in ANS.iter().enumerate() {
            #[auto_enum(Iterator)]
            let iter = if i == 0 {
                1..8
            } else if i > 3 {
                1..=10
            } else {
                vec![1, 2, 0].into_iter()
            };
            assert_eq!(iter.sum::<i32>(), *x);
        }

        // no return
        for (i, x) in ANS.iter().enumerate() {
            #[auto_enum(Iterator)]
            let iter = match i {
                0 => 1..8,
                #[never]
                5..=10 => loop {
                    panic!();
                },
                _ => vec![1, 2, 0].into_iter(),
            };
            assert_eq!(iter.sum::<i32>(), *x);
        }
        for (i, x) in ANS.iter().enumerate() {
            #[auto_enum(Iterator)]
            let iter = match i {
                0 => 1..8,
                5..=10 => panic!(),
                11..=20 => unreachable!(),
                21..=30 => break,
                31..=40 => continue,
                41..=50 => return,
                _ => vec![1, 2, 0].into_iter(),
            };
            assert_eq!(iter.sum::<i32>(), *x);
        }
        for (i, x) in ANS.iter().enumerate() {
            #[auto_enum(Iterator)]
            let iter = if i > 3 {
                #[never]
                loop {
                    panic!();
                }
            } else if i == 0 {
                1..8
            } else {
                vec![1, 2, 0].into_iter()
            };
            assert_eq!(iter.sum::<i32>(), *x);
        }

        #[allow(clippy::needless_late_init)]
        fn assign(x: usize) -> impl Iterator<Item = i32> + Clone {
            let a;
            a = #[auto_enum(Iterator, Clone)]
            match x {
                0 => 2..8,
                _ if x < 2 => vec![2, 0].into_iter(),
                _ => 2..=10,
            };
            a
        }
        for (i, x) in ANS.iter().enumerate() {
            assert_eq!(assign(i).sum::<i32>(), *x - 1);
        }

        /*
        This can not be supported. It is parsed as follows.
            expected: ExprAssign { left: ExprPath, right: ExprMatch, .. }
               found: ExprPath
        fn assign2(x: usize) -> impl Iterator<Item = i32> + Clone {
            let a;
            #[auto_enum(Iterator, Clone)]
            a = match x {
                0 => 2..8,
                _ if x < 2 => vec![2, 0].into_iter(),
                _ => 2..=10,
            };
            a
        }
        */

        #[auto_enum(Fn)]
        fn fn_traits1(option: bool) -> impl Fn(i32) -> i32 {
            if option {
                |x| x + 1
            } else {
                |y| y - 1
            }
        }
        assert_eq!(fn_traits1(true)(1), 2);

        #[auto_enum(Iterator, Clone)]
        let _y = match 0 {
            0 => 2..8,
            _ => 2..=10,
        };

        #[auto_enum(Iterator, Clone)]
        let _x = match 0 {
            0 => 2..8,
            _ => 2..=10,
        };
    }

    #[test]
    fn never() {
        // never attr
        for (i, x) in ANS.iter().enumerate() {
            #[auto_enum(Iterator)]
            let iter = match i {
                0 => 1..8,
                #[never]
                5..=10 => loop {
                    panic!();
                },
                _ => match i {
                    #[never]
                    5..=10 => loop {
                        panic!();
                    },
                    #[never]
                    11..=20 => loop {
                        panic!();
                    },
                    _ => vec![1, 2, 0].into_iter(),
                },
            };
            assert_eq!(iter.sum::<i32>(), *x);
        }
        for (i, x) in ANS.iter().enumerate() {
            #[rustfmt::skip]
            #[auto_enum(Iterator)]
            let iter = match i {
                0 => 1..8,
                #[never]
                5..=10 => loop {
                    panic!();
                },
                1..=4 => vec![1, 2, 0].into_iter(),
                _ => {
                    match i {
                        #[never]
                        5..=10 => loop {
                            panic!();
                        },
                        #[never]
                        11..=20 => loop {
                            panic!();
                        },
                        _ => panic!(),
                    }
                }
            };
            assert_eq!(iter.sum::<i32>(), *x);
        }
    }

    #[test]
    fn marker() {
        fn marker1(x: usize) -> impl Iterator<Item = i32> + Clone {
            #[auto_enum(Iterator, Clone)]
            (0..x as i32).map(|x| x + 1).flat_map(|x| {
                if x > 10 {
                    marker!(0..x)
                } else {
                    marker!(-100..=0)
                }
            })
        }
        for (i, _x) in ANS.iter().enumerate() {
            let _ = marker1(i).clone().sum::<i32>();
        }

        fn marker2(x: usize) -> impl Iterator<Item = i32> + Clone {
            let a;

            #[auto_enum(Iterator, Clone)]
            match x {
                0 => a = marker!(2..8),
                _ if x < 2 => a = marker!(vec![2, 0].into_iter()),
                _ => a = marker!(2..=10),
            };
            a
        }
        for (i, x) in ANS.iter().enumerate() {
            assert_eq!(marker2(i).clone().sum::<i32>(), *x - 1);
        }
    }

    #[test]
    fn non_stmt_expr() {
        // use std::iter;

        // TODO: uncomment when rustc bug fixed.
        // fn match_(x: bool) -> Option<impl Iterator<Item = u8>> {
        //     Some(
        //         #[auto_enum(Iterator)]
        //         match x {
        //             true => iter::once(0),
        //             _ => iter::repeat(1),
        //         },
        //     )
        // }

        // TODO: uncomment when rustc bug fixed.
        // fn if_(x: bool) -> Option<impl Iterator<Item = u8>> {
        //     Some(
        //         #[auto_enum(Iterator)]
        //         if x { iter::once(0) } else { iter::repeat(1) },
        //     )
        // }
    }
}
