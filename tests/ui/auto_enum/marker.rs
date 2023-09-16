// SPDX-License-Identifier: Apache-2.0 OR MIT

use auto_enums::auto_enum;

#[auto_enum(Iterator, marker = foo)]
fn a(x: usize) -> impl Iterator<Item = i32> {
    #[auto_enum(Iterator, marker = foo)]
    //~ ERROR a custom marker name is specified that duplicated the name already used in the parent scope
    let _iter = match x {
        0 => 1..8,
        1 => return foo!(1..9),
        _ => (0..2).map(|x| x + 1),
    };

    match x {
        0 => 1..8,
        _ => (0..2).map(|x| x + 1),
    }
}

#[auto_enum(Iterator, marker = foo)]
fn b(x: usize) -> impl Iterator<Item = i32> {
    #[auto_enum(Iterator, marker = bar)]
    let _iter = match x {
        0 => 1..8,
        1 => return foo!(1..9),    // OK
        2 => return marker!(1..9), //~ ERROR cannot find macro `marker!` in this scope
        _ => (0..2).map(|x| x + 1),
    };

    match x {
        0 => 1..8,
        _ => (0..2).map(|x| x + 1),
    }
}

fn main() {}
