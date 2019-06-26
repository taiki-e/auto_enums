// compile-fail

#![deny(warnings)]
#![feature(try_trait)]

use auto_enums::auto_enum;

#[auto_enum(Iterator)]
fn match_(x: usize) -> impl Iterator<Item = i32> {
    match x {
        0 => marker!(1..8..),
        _ => (0..2).map(|x| x + 1),
    }
}

#[auto_enum(Iterator)]
fn if_(x: usize) -> impl Iterator<Item = i32> {
    if x == 0 {
        1..8
    } else if x > 3 {
        2..=10
    }
}

#[auto_enum(Iterator)]
fn return2(x: i32, y: i32) -> impl Iterator<Item = i32> {
    #[auto_enum(Iterator)]
    let iter = match x {
        _ if y < 0 => return y..=0,
        _ => 2..=10,
    };

    match y {
        0 => iter.flat_map(|x| 0..x),
        _ => iter.map(|x| x + 1),
    }
}

fn main() {}
