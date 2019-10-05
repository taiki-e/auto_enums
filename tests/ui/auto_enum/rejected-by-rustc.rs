// compile-fail

#![feature(try_trait)]

use auto_enums::auto_enum;

#[auto_enum(Iterator)]
fn wrong_if(x: usize) -> impl Iterator<Item = i32> {
    if x == 0 {
        1..8
    } else return {
        //~^ ERROR expected `{`, found keyword `return`
        2..=10
    }
}

#[auto_enum(Iterator)]
fn if_in_if(x: usize) -> impl Iterator<Item = i32> {
    if x == 0 {
        1..8
    } else if x > 3 {
        #[nested]
        if x > 4 { 2..=10 } else { (11..20).map(|x| x - 1) }
    } else {
        (0..2).map(|x| x + 1)
    }
}

fn main() {}
