// compile-fail

#![deny(warnings)]
#![feature(try_trait)]

use auto_enums::auto_enum;

#[auto_enum(Iterator;)] //~ ERROR expected one of `,`, or `::`, found `;`
fn unexpected_token_1(x: usize) -> impl Iterator<Item = i32> {
    match x {
        0 => 1..=8,
        _ => 0..2,
    }
}

#[auto_enum(Iterator,;)] //~ ERROR expected one of `,`, `::`, or identifier, found `;`
fn unexpected_token_2(x: usize) -> impl Iterator<Item = i32> {
    match x {
        0 => 1..=8,
        _ => 0..2,
    }
}

mod marker {
    use auto_enums::auto_enum;

    #[auto_enum(never, never, Iterator)] //~ ERROR multiple `never` option
    fn multiple_never(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..=8,
            _ => 0..2,
        }
    }

    #[auto_enum(marker{f}, Iterator)] //~ ERROR invalid delimiter
    fn marker_invalid_delimiter_1(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..=8,
            _ => 0..2,
        }
    }

    #[auto_enum(marker[f], Iterator)] //~ ERROR invalid delimiter
    fn marker_invalid_delimiter_2(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..=8,
            _ => 0..2,
        }
    }

    #[auto_enum(marker(f), marker(g), Iterator)] //~ ERROR multiple `marker` option
    fn multiple_marker(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..=8,
            _ => 0..2,
        }
    }

    #[auto_enum(marker(), Iterator)] //~ ERROR empty `marker` option
    fn empty_marker(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..=8,
            _ => 0..2,
        }
    }

    #[auto_enum(marker(f, g), Iterator)] //~ ERROR multiple identifier in `marker` option
    fn marker_multiple_ident_1(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..=8,
            _ => 0..2,
        }
    }

    #[auto_enum(marker(f t), Iterator)] //~ ERROR multiple identifier in `marker` option
    fn marker_multiple_ident_2(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..=8,
            _ => 0..2,
        }
    }

    #[auto_enum(marker=f, marker=g, Iterator)] //~ ERROR multiple `marker` option
    fn multiple_marker_eq(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..=8,
            _ => 0..2,
        }
    }

    #[auto_enum(marker=, Iterator)] //~ ERROR empty `marker` option
    fn empty_marker_eq(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..=8,
            _ => 0..2,
        }
    }

    #[auto_enum(marker=f t, Iterator)] //~ ERROR expected `,`, found `t`
    fn marker_eq_multiple_ident(x: usize) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..=8,
            _ => 0..2,
        }
    }
}

fn main() {}
