// compile-fail

#![feature(try_trait)]

use auto_enums::auto_enum;

#[auto_enum(Iterator)]
fn unexpected_token_in_marker(x: usize) -> impl Iterator<Item = i32> {
    match x {
        0 => marker!(1..8..), //~ ERROR expected one of `)`, `,`, `.`, `?`, or an operator, found `..`
        _ => (0..2).map(|x| x + 1),
    }
}

#[auto_enum(Iterator)]
fn if_missing_else(x: usize) -> impl Iterator<Item = i32> {
    if x == 0 {
        1..8
    } else if x > 3 {
        //~^ ERROR `if` expression missing an else clause
        2..=10
    }
}

#[auto_enum(Iterator)]
fn return1(x: i32, y: i32) -> impl Iterator<Item = i32> {
    #[auto_enum(Iterator)]
    let iter = match x {
        //~^ ERROR `#[auto_enum]` is required two or more branches or marker macros in total, there is only one branch or marker macro in this statement
        _ if y < 0 => return y..=0,
        _ => 2..=10,
    };

    match y {
        0 => iter.flat_map(|x| 0..x),
        _ => iter.map(|x| x + 1),
    }
}

#[auto_enum(Iterator)]
fn return0(x: i32, y: i32) -> impl Iterator<Item = i32> {
    #[auto_enum(Iterator)]
    let iter = match x {
        //~^ ERROR `#[auto_enum]` is required two or more branches or marker macros in total, there is no branch or marker macro in this statement
        _ if y < 0 => return y..=0,
        _ => return 2..=10,
    };

    match y {
        0 => iter.flat_map(|x| 0..x),
        _ => iter.map(|x| x + 1),
    }
}

fn main() {}
