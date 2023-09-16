// SPDX-License-Identifier: Apache-2.0 OR MIT

use auto_enums::auto_enum;

#[auto_enum(Iterator)]
fn unexpected_token_in_never(x: usize) -> impl Iterator<Item = i32> {
    match x {
        0 => 1..8,
        #[never(foo)] //~ ERROR unexpected token
        1 => panic!(),
        _ => (0..2).map(|x| x + 1),
    }
}

#[auto_enum(Iterator)]
fn unexpected_token_in_nested(x: usize) -> impl Iterator<Item = i32> {
    match x {
        0 => 1..8,
        #[nested(foo)] //~ ERROR unexpected token
        1 => panic!(),
        _ => (0..2).map(|x| x + 1),
    }
}

#[auto_enum(Iterator)]
fn removed_rec(x: usize) -> impl Iterator<Item = i32> {
    match x {
        0 => 1..8,
        #[rec] //~ ERROR #[rec] has been removed and replaced with #[nested]
        1 => panic!(),
        _ => (0..2).map(|x| x + 1),
    }
}

fn main() {}
