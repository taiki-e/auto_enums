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

fn main() {}
