// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(feature = "type_analysis")]
#![allow(dead_code)]

use std::fmt;

use auto_enums::auto_enum;

#[test]
fn func() {
    #[auto_enum] // there is no need to specify std library's traits
    fn test1(x: i32) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..10,
            _ => vec![5, 10].into_iter(),
        }
    }

    #[auto_enum] // this is handled as a dummy attribute.
    fn test2(x: i32) -> impl Iterator<Item = i32> {
        #[auto_enum(Iterator)]
        match x {
            0 => 1..10,
            _ => vec![5, 10].into_iter(),
        }
    }

    #[auto_enum] // there is no need to specify std library's traits
    fn test3(x: i32) -> impl Iterator<Item = i32> {
        match x {
            0 => return 1..10,
            _ => vec![5, 10].into_iter(),
        }
    }

    #[auto_enum] // this is handled as a dummy attribute.
    fn test4(x: i32) -> impl Iterator<Item = i32> {
        #[auto_enum(Iterator)]
        let iter = match x {
            0 => 1..10,
            1 => 11..=20,
            _ => return vec![5, 10].into_iter(),
        };
        iter.collect::<Vec<_>>().into_iter()
    }

    #[auto_enum]
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

    #[auto_enum]
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
}

#[test]
fn local() {
    #[auto_enum]
    fn test1(x: i32) {
        #[auto_enum]
        let _y: impl Iterator<Item = i32> = match x {
            0 => 1..10,
            _ => vec![5, 10].into_iter(),
        };
    }

    #[auto_enum]
    fn test2(x: i32) -> impl Iterator<Item = i32> + fmt::Debug {
        #[auto_enum(fmt::Debug)]
        let y: impl Iterator<Item = i32> = match x {
            0 => 1..10,
            _ => vec![5, 10].into_iter(),
        };
        y
    }
}
