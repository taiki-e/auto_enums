// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(proc_macro_hygiene, stmt_expr_attributes, type_ascription)]
#![feature(coroutine_trait)]
#![feature(fn_traits, unboxed_closures)]
#![feature(trusted_len)]

use auto_enums::auto_enum;

const ANS: &[i32] = &[28, 3];

fn main() {
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

    // marker
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

    // TODO: workaround rustc bug in the same way as https://github.com/taiki-e/futures-async-stream/pull/94.
    // use std::iter;
    //
    // fn match_(x: bool) -> Option<impl Iterator<Item = u8>> {
    //     Some(
    //         #[auto_enum(Iterator)]
    //         match x {
    //             true => iter::once(0),
    //             _ => iter::repeat(1),
    //         },
    //     )
    // }
    //
    // fn if_(x: bool) -> Option<impl Iterator<Item = u8>> {
    //     Some(
    //         #[auto_enum(Iterator)]
    //         if x { iter::once(0) } else { iter::repeat(1) },
    //     )
    // }
}
