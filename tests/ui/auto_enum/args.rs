// SPDX-License-Identifier: Apache-2.0 OR MIT

use auto_enums::auto_enum;

#[auto_enum(Iterator;)] //~ ERROR expected `,`
fn unexpected_token_1(x: usize) -> impl Iterator<Item = i32> {
    match x {
        0 => 1..=8,
        _ => 0..2,
    }
}

#[auto_enum(Iterator,;)] //~ ERROR expected identifier
fn unexpected_token_2(x: usize) -> impl Iterator<Item = i32> {
    match x {
        0 => 1..=8,
        _ => 0..2,
    }
}

mod marker {
    use auto_enums::auto_enum;

    #[auto_enum(marker{f}, Iterator)] //~ ERROR expected `,`
    fn marker_invalid_delimiter_1(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..=8,
            _ => 0..2,
        }
    }

    #[auto_enum(marker[f], Iterator)] //~ ERROR expected `,`
    fn marker_invalid_delimiter_2(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..=8,
            _ => 0..2,
        }
    }

    #[auto_enum(marker(f), Iterator)] //~ ERROR expected `,`
    fn marker_removed_delimiter(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..=8,
            _ => 0..2,
        }
    }

    #[auto_enum(marker = f, marker = g, Iterator)] //~ ERROR duplicate `marker` argument
    fn multiple_marker(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..=8,
            _ => 0..2,
        }
    }

    #[auto_enum(marker =, Iterator)] //~ ERROR expected identifier
    fn empty_marker(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..=8,
            _ => 0..2,
        }
    }

    #[auto_enum(marker = f t, Iterator)] //~ ERROR expected `,`
    fn marker_multiple_ident(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..=8,
            _ => 0..2,
        }
    }
}

fn main() {}
