#![warn(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes)]
#![allow(dead_code)]

use auto_enums::auto_enum;

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
    #[allow(unknown_lints)]
    #[allow(unsafe_code)]
    #[allow(unused_unsafe)]
    #[allow(unused_braces)]
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
                if x > 4 { 2..=10 } else { (11..20).map(|x| x - 1) }
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

            if x > 5 { y } else { z }
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
