#![cfg_attr(
    feature = "unstable",
    feature(
        proc_macro_hygiene,
        stmt_expr_attributes,
        fn_traits,
        unboxed_closures,
        exact_size_is_empty,
        generators,
        generator_trait,
        read_initializer,
        trusted_len,
        try_trait,
        type_ascription,
    )
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(not(feature = "std"), feature = "unstable"), feature(alloc))]
#![warn(rust_2018_idioms)]
#![allow(ellipsis_inclusive_range_patterns)] // syn generates both as `...`.
#![allow(unused_imports)]

#[cfg(all(not(feature = "std"), feature = "unstable"))]
#[macro_use]
extern crate alloc;

use auto_enums::auto_enum;

#[test]
fn stable_1_30() {
    const ANS: &[i32] = &[28, 3];

    #[auto_enum(Iterator)]
    fn match_(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..8,
            n if n > 3 => 2..=10,
            _ => (0..2).map(|x| x + 1),
        }
    }
    for i in 0..2 {
        assert_eq!(match_(i).fold(0, |sum, x| sum + x), ANS[i]);
    }

    // block + unsafe block + parentheses
    #[cfg_attr(feature = "rustfmt", rustfmt_skip)]
    #[allow(unused_unsafe)]
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
    for i in 0..2 {
        assert_eq!(block(i).fold(0, |sum, x| sum + x), ANS[i]);
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
    for i in 0..2 {
        assert_eq!(if_(i).fold(0, |sum, x| sum + x), ANS[i]);
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
    for i in 0..2 {
        assert_eq!(method_call(i).fold(0, |sum, x| sum + x), ANS[i]);
    }

    #[auto_enum(Iterator)]
    fn no_return(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..8,
            3 => panic!(),
            _ => (0..2).map(|x| x + 1),
        }
    }
    for i in 0..2 {
        assert_eq!(no_return(i).fold(0, |sum, x| sum + x), ANS[i]);
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
    for i in 0..2 {
        assert_eq!(no_return2(i).fold(0, |sum, x| sum + x), ANS[i]);
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
    for i in 0..2 {
        assert_eq!(no_return3(i).fold(0, |sum, x| sum + x), ANS[i]);
    }
    #[auto_enum(Iterator)]
    fn no_return4(x: usize) -> impl Iterator<Item = i32> {
        if x == 0 {
            1..8
        } else if x > 3 {
            panic!();
        } else {
            (0..2).map(|x| x + 1)
        }
    }
    for i in 0..2 {
        assert_eq!(no_return4(i).fold(0, |sum, x| sum + x), ANS[i]);
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
    for i in 0..2 {
        assert_eq!(return1(i).fold(0, |sum, x| sum + x), ANS[i]);
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
    assert_eq!(break_in_loop(14).fold(0, |sum, x| sum + x), 55);
    assert_eq!(break_in_loop(-5).fold(0, |sum, x| sum + x), -15);

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
    assert_eq!(break2(14).fold(0, |sum, x| sum + x), 55);
    assert_eq!(break2(-5).fold(0, |sum, x| sum + x), -15);

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
    assert_eq!(break3(14).fold(0, |sum, x| sum + x), 55);
    assert_eq!(break3(-5).fold(0, |sum, x| sum + x), -15);

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
    assert_eq!(return_in_loop(14).fold(0, |sum, x| sum + x), 55);
    assert_eq!(return_in_loop(-5).fold(0, |sum, x| sum + x), -15);

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
    assert_eq!(return2(10, 10).fold(0, |sum, x| sum + x), 63);

    #[auto_enum]
    fn return3(x: i32) -> Option<impl Iterator<Item = i32>> {
        if x < 0 {
            return None;
        }

        #[auto_enum(Iterator)]
        let iter = match x {
            0 => 2..8,
            _ => 2..=10,
        };

        Some(iter)
    }
    assert_eq!(return3(10).unwrap().fold(0, |sum, x| sum + x), 54);

    #[auto_enum(Debug, Display)]
    fn try_operator1(x: i32) -> Result<impl Iterator<Item = i32>, impl core::fmt::Debug> {
        if x < 0 {
            Err(1i32)?;
        }

        let iter = match x {
            0 => Err(())?,
            _ => 2..=10,
        };

        Ok(iter)
    }
    assert_eq!(try_operator1(10).unwrap().fold(0, |sum, x| sum + x), 54);

    #[auto_enum(Debug)]
    fn try_operator2(x: i32) -> Result<impl Iterator<Item = i32>, impl core::fmt::Debug> {
        if x < 0 {
            Err(1i32)?;
        }

        match x {
            0 => Err(())?,
            _ => Ok(2..=10),
        }
    }
    assert_eq!(try_operator2(10).unwrap().fold(0, |sum, x| sum + x), 54);

    #[auto_enum(Iterator)]
    fn marker3(x: i32, y: i32) -> impl Iterator<Item = i32> {
        let iter;
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
    assert_eq!(marker3(10, 10).fold(0, |sum, x| sum + x), 63);

    #[auto_enum(marker(marker_a), Iterator)]
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
    assert_eq!(marker4(10, 10).fold(0, |sum, x| sum + x), 63);

    #[auto_enum(Iterator)]
    fn marker5(x: i32, y: i32) -> impl Iterator<Item = i32> {
        let iter;
        #[auto_enum(marker(marker_a), Iterator)]
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
    assert_eq!(marker5(10, 10).fold(0, |sum, x| sum + x), 63);

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

        for i in 0..2 {
            assert_eq!(f(i).fold(0, |sum, x| sum + x), ANS[i]);
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

        for i in 0..2 {
            assert_eq!(f(i).fold(0, |sum, x| sum + x), ANS[i]);
        }
    }
    closure();

    #[auto_enum(Iterator)]
    fn rec_match_in_match(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..8,
            #[rec]
            n if n > 3 => match x {
                2..=10 => (1..x as _).map(|x| x - 1),
                _ => 2..=10,
            },
            _ => (0..2).map(|x| x + 1),
        }
    }
    for i in 0..2 {
        assert_eq!(rec_match_in_match(i).fold(0, |sum, x| sum + x), ANS[i]);
    }

    #[cfg_attr(feature = "rustfmt", rustfmt_skip)]
    #[allow(unused_unsafe)]
    #[auto_enum(Iterator)]
    fn rec_in_block(x: usize) -> impl Iterator<Item = i32> {
        {{{ unsafe {{{ unsafe { unsafe {{
            match x {
                0 => 1..8,
                #[rec]
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
    for i in 0..2 {
        assert_eq!(rec_in_block(i).fold(0, |sum, x| sum + x), ANS[i]);
    }

    #[auto_enum(Iterator)]
    fn rec_match_in_if(x: usize) -> impl Iterator<Item = i32> {
        if x == 0 {
            1..8
        } else if x > 3 {
            #[rec]
            match x {
                1..=4 => 2..=10,
                _ => (11..20).map(|x| x - 1),
            }
        } else {
            (0..2).map(|x| x + 1)
        }
    }
    for i in 0..2 {
        assert_eq!(rec_match_in_if(i).fold(0, |sum, x| sum + x), ANS[i]);
    }

    #[auto_enum(Iterator)]
    fn rec_if_in_if(x: usize) -> impl Iterator<Item = i32> {
        if x == 0 {
            1..8
        } else if x > 3 {
            #[rec]
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
    for i in 0..2 {
        assert_eq!(rec_if_in_if(i).fold(0, |sum, x| sum + x), ANS[i]);
    }

    #[auto_enum(Iterator)]
    fn rec_nop(x: usize) -> impl Iterator<Item = i32> {
        if x == 0 {
            1..8
        } else if x > 3 {
            #[rec]
            2..=10
        } else {
            (0..2).map(|x| x + 1)
        }
    }
    for i in 0..2 {
        assert_eq!(rec_nop(i).fold(0, |sum, x| sum + x), ANS[i]);
    }

    #[auto_enum(Iterator)]
    fn rec_no_return(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..8,
            #[rec]
            3 => panic!(),
            _ => (0..2).map(|x| x + 1),
        }
    }
    for i in 0..2 {
        assert_eq!(rec_no_return(i).fold(0, |sum, x| sum + x), ANS[i]);
    }

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
    assert_eq!(transpose(0).unwrap().fold(0, |sum, x| sum + x), 28);
}

#[cfg(feature = "std")]
#[test]
fn stable_1_30_std() {
    use auto_enums::enum_derive;
    use std::{error::Error, fs, io, path::Path};

    #[auto_enum(Transpose, Write)]
    fn transpose_ok(file: Option<&Path>) -> io::Result<impl io::Write> {
        if let Some(file) = file { fs::File::create(file) } else { Ok(io::stdout()) }.transpose_ok()
    }
    assert!(transpose_ok(None).is_ok());

    #[auto_enum(Transpose, Write)]
    fn transpose_option(file: Option<&Path>) -> Option<impl io::Write> {
        if let Some(file) = file { fs::File::create(file).ok() } else { Some(io::stdout()) }
            .transpose()
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
            fs::File::create(file).map_err(IoError::Io)
        } else {
            let out: Result<io::Stdout, io::Error> = Ok(io::stdout());
            out
        }
        .transpose()
    }
    assert!(transpose_result(None).is_ok());

    #[auto_enum(Transpose, Debug, Display, Error)]
    fn transpose_err(file: Option<&Path>) -> Result<(), impl Error> {
        if let Some(_) = file {
            Err(io::Error::from(io::ErrorKind::NotFound)).map_err(IoError::Io2)
        } else {
            Err(io::Error::from(io::ErrorKind::NotFound))
        }
        .transpose_err()
    }
    assert!(transpose_err(None).unwrap_err().source().is_some());

    #[auto_enum(Debug, Display, Error)]
    fn try_operator(file: Option<&Path>) -> Result<(), impl Error> {
        if let Some(_) = file {
            Err(io::Error::from(io::ErrorKind::NotFound)).map_err(IoError::Io2)?
        } else {
            Err(io::Error::from(io::ErrorKind::NotFound))?
        }

        Ok(())
    }
    assert!(try_operator(None).unwrap_err().source().is_some());
}

#[cfg(feature = "unstable")]
#[test]
fn nightly() {
    const ANS: &[i32] = &[28, 3];

    // let match
    for i in 0..2 {
        #[auto_enum(Iterator)]
        let iter = match i {
            0 => 1..8,
            _ => vec![1, 2, 0].into_iter(),
        };
        assert_eq!(iter.fold(0, |sum, x| sum + x), ANS[i]);
    }

    // let if
    for i in 0..2 {
        #[auto_enum(Iterator)]
        let iter = if i == 0 {
            1..8
        } else if i > 3 {
            1..=10
        } else {
            vec![1, 2, 0].into_iter()
        };
        assert_eq!(iter.fold(0, |sum, x| sum + x), ANS[i]);
    }

    // no return
    for i in 0..2 {
        #[auto_enum(Iterator)]
        let iter = match i {
            0 => 1..8,
            #[never]
            5..=10 => loop {
                panic!()
            },
            _ => vec![1, 2, 0].into_iter(),
        };
        assert_eq!(iter.fold(0, |sum, x| sum + x), ANS[i]);
    }
    for i in 0..2 {
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
        assert_eq!(iter.fold(0, |sum, x| sum + x), ANS[i]);
    }
    for i in 0..2 {
        #[auto_enum(Iterator)]
        let iter = if i > 3 {
            #[never]
            loop {
                panic!()
            }
        } else if i == 0 {
            1..8
        } else {
            vec![1, 2, 0].into_iter()
        };
        assert_eq!(iter.fold(0, |sum, x| sum + x), ANS[i]);
    }

    // rec
    for i in 0..2 {
        #[auto_enum(Iterator)]
        let iter = if i > 3 {
            #[rec]
            match i {
                1..=10 => (1..3).map(|x| x + 1),
                11..=20 => 4..=5,
                _ => (5..10).map(|x| x - 1),
            }
        } else if i == 0 {
            1..8
        } else {
            vec![1, 2, 0].into_iter()
        };
        assert_eq!(iter.fold(0, |sum, x| sum + x), ANS[i]);
    }

    // never opt
    for i in 0..2 {
        #[auto_enum(never, Iterator)]
        let iter = match i {
            0 => marker!(1..8),
            5..=10 => loop {
                panic!()
            },
            _ => marker!(vec![1, 2, 0].into_iter()),
        };
        assert_eq!(iter.fold(0, |sum, x| sum + x), ANS[i]);
    }

    // never attr
    for i in 0..2 {
        #[cfg_attr(feature = "rustfmt", rustfmt_skip)]
        #[auto_enum(Iterator)]
        let iter = match i {
            0 => 1..8,
            #[never]
            5..=10 => loop {
                panic!()
            },
            _ => {
                match i {
                    #[never]
                    5..=10 => loop {
                        panic!()
                    },
                    #[never]
                    11..=20 => loop {
                        panic!()
                    },
                    _ => vec![1, 2, 0].into_iter(),
                }
            }
        };
        assert_eq!(iter.fold(0, |sum, x| sum + x), ANS[i]);
    }
    for i in 0..2 {
        #[cfg_attr(feature = "rustfmt", rustfmt_skip)]
        #[auto_enum(Iterator)]
        let iter = match i {
            0 => 1..8,
            #[never]
            5..=10 => loop {
                panic!()
            },
            1..=4 => vec![1, 2, 0].into_iter(),
            _ => {
                match i {
                    #[never]
                    5..=10 => loop {
                        panic!()
                    },
                    #[never]
                    11..=20 => loop {
                        panic!()
                    },
                    _ => panic!(),
                }
            }
        };
        assert_eq!(iter.fold(0, |sum, x| sum + x), ANS[i]);
    }

    #[auto_enum(Debug)]
    fn try_operator(x: i32) -> Result<impl Iterator<Item = i32>, impl core::fmt::Debug> {
        if x < 0 {
            Err(1i32)?;
        }

        let iter = match x {
            0 => Err(())?,
            1 => None?,
            _ => 2..=10,
        };

        Ok(iter)
    }
    assert_eq!(try_operator(10).unwrap().fold(0, |sum, x| sum + x), 54);

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
    for i in 0..2 {
        let _ = marker1(i).clone().fold(0, |sum, x| sum + x);
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
    for i in 0..2 {
        assert_eq!(marker2(i).clone().fold(0, |sum, x| sum + x), ANS[i] - 1);
    }

    /*
    This can not be supported. It is parsed as follows.
        expected: ExprAssign { left: ExprPath, right: ExprMatch, .. }
           found: ExprPath
    #[auto_enum(Iterator, Clone)]
    a = match x {
        0 => 2..8,
        _ if x < 2 => vec![2, 0].into_iter(),
        _ => 2..=10,
    };
    */
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
    for i in 0..2 {
        assert_eq!(assign(i).fold(0, |sum, x| sum + x), ANS[i] - 1);
    }

    #[auto_enum(Fn)]
    fn fn_traits1(option: bool) -> impl Fn(i32) -> i32 {
        if option {
            |x| x + 1
        } else {
            |y| y - 1
        }
    }
    assert_eq!(fn_traits1(true)(1), 2);

    // parentheses and type ascription
    #[auto_enum(Fn)]
    fn fn_traits2(option: bool) -> impl Fn(i32) -> i32 {
        (if option { |x| x + 1 } else { |y| y - 1 }): _
    }
    assert_eq!(fn_traits2(true)(1), 2);

    use core::{
        ops::{Generator, GeneratorState},
        pin::Pin,
    };

    #[auto_enum(Generator)]
    fn generator_trait(x: i32) -> impl Generator<Yield = i32, Return = &'static str> {
        match x {
            0 => || {
                yield 1;
                return "foo";
            },
            _ => || {
                yield 2;
                return "bar";
            },
        }
    }

    let mut generator = generator_trait(0);
    match Pin::new(&mut generator).resume() {
        GeneratorState::Yielded(1) => {}
        _ => panic!("unexpected return from resume"),
    }
    match Pin::new(&mut generator).resume() {
        GeneratorState::Complete("foo") => {}
        _ => panic!("unexpected return from resume"),
    }

    let mut generator = generator_trait(1);
    match Pin::new(&mut generator).resume() {
        GeneratorState::Yielded(2) => {}
        _ => panic!("unexpected return from resume"),
    }
    match Pin::new(&mut generator).resume() {
        GeneratorState::Complete("bar") => {}
        _ => panic!("unexpected return from resume"),
    }
}
