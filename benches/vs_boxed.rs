#![feature(test)]
#![warn(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes)]

extern crate test;

use auto_enums::auto_enum;
use rand::Rng;
use test::Bencher;

fn iter_no_branch(_x: u32) -> impl Iterator<Item = i64> {
    (0..).map(|x| x + 2 - 1)
}

fn iter_boxed2(x: u32) -> Box<dyn Iterator<Item = i64>> {
    match x {
        0 => Box::new((0..).map(|x| (x * 2) - 1)),
        _ => Box::new((0..).map(|x| (x + 1) / 2)),
    }
}

fn iter_boxed16(x: u32) -> Box<dyn Iterator<Item = i64>> {
    match x {
        0 => Box::new((0..).map(|x| (x * 2) * 2)),
        1 => Box::new((0..).map(|x| (x - 1) / 2)),
        2 => Box::new((0..).map(|x| (x * 2) - 1)),
        3 => Box::new((0..).map(|x| (x + 1) + 2)),
        4 => Box::new((0..).map(|x| (x / 2) - 1)),
        5 => Box::new((0..).map(|x| (x * 2) - 1)),
        6 => Box::new((0..).map(|x| (x / 2) + 1)),
        7 => Box::new((0..).map(|x| (x * 2) + 1)),
        8 => Box::new((0..).map(|x| (x / 2) / 2)),
        9 => Box::new((0..).map(|x| (x / 2) - 2)),
        10 => Box::new((0..).map(|x| (x + 2) * 2)),
        11 => Box::new((0..).map(|x| (x + 2) / 2)),
        12 => Box::new((0..).map(|x| (x - 1) - 2)),
        13 => Box::new((0..).map(|x| (x - 2) * 2)),
        14 => Box::new((0..).map(|x| (x - 2) / 2)),
        _ => Box::new((0..).map(|x| (x + 1) / 2)),
    }
}

#[auto_enum(Iterator)]
fn iter_enum2(x: u32) -> impl Iterator<Item = i64> {
    match x {
        0 => (0..).map(|x| (x * 2) - 1),
        _ => (0..).map(|x| (x + 1) / 2),
    }
}

#[auto_enum(Iterator)]
fn iter_enum4(x: u32) -> impl Iterator<Item = i64> {
    match x {
        0 => (0..).map(|x| (x * 2) * 2),
        1 => (0..).map(|x| (x - 1) / 2),
        2 => (0..).map(|x| (x * 2) - 1),
        _ => (0..).map(|x| (x + 1) / 2),
    }
}

#[auto_enum(Iterator)]
fn iter_enum8(x: u32) -> impl Iterator<Item = i64> {
    match x {
        0 => (0..).map(|x| (x * 2) * 2),
        1 => (0..).map(|x| (x - 1) / 2),
        2 => (0..).map(|x| (x * 2) - 1),
        3 => (0..).map(|x| (x + 1) + 2),
        4 => (0..).map(|x| (x / 2) - 1),
        5 => (0..).map(|x| (x * 2) - 1),
        6 => (0..).map(|x| (x / 2) + 1),
        _ => (0..).map(|x| (x + 1) / 2),
    }
}

#[auto_enum(Iterator)]
fn iter_enum16(x: u32) -> impl Iterator<Item = i64> {
    match x {
        0 => (0..).map(|x| (x * 2) * 2),
        1 => (0..).map(|x| (x - 1) / 2),
        2 => (0..).map(|x| (x * 2) - 1),
        3 => (0..).map(|x| (x + 1) + 2),
        4 => (0..).map(|x| (x / 2) - 1),
        5 => (0..).map(|x| (x * 2) - 1),
        6 => (0..).map(|x| (x / 2) + 1),
        7 => (0..).map(|x| (x * 2) + 1),
        8 => (0..).map(|x| (x / 2) / 2),
        9 => (0..).map(|x| (x / 2) - 2),
        10 => (0..).map(|x| (x + 2) * 2),
        11 => (0..).map(|x| (x + 2) / 2),
        12 => (0..).map(|x| (x - 1) - 2),
        13 => (0..).map(|x| (x - 2) * 2),
        14 => (0..).map(|x| (x - 2) / 2),
        _ => (0..).map(|x| (x + 1) / 2),
    }
}

macro_rules! bench_next {
    ($($fn:ident, $iter:ident, $max:expr, $num:expr,)*) => {$(
        #[bench]
        fn $fn(b: &mut Bencher) {
            let mut rng = rand::thread_rng();
            b.iter(|| {
                let mut iter = $iter(rng.gen_range(0, $max));
                (0..$num).for_each(|_| assert!(iter.next().is_some()))
            })
        }
    )*};
}

macro_rules! bench_fold {
    ($($fn:ident, $iter:ident, $max:expr, $num:expr,)*) => {$(
        #[bench]
        fn $fn(b: &mut Bencher) {
            let mut rng = rand::thread_rng();
            b.iter(|| {
                let iter = $iter(rng.gen_range(0, $max));
                iter.take($num).fold(0, |sum, x| sum + x)
            })
        }
    )*};
}

macro_rules! bench_fold_semi {
    ($($fn:ident, $iter:ident, $max:expr, $num:expr,)*) => {$(
        #[bench]
        fn $fn(b: &mut Bencher) {
            let mut rng = rand::thread_rng();
            b.iter(|| {
                let iter = $iter(rng.gen_range(0, $max));
                iter.take($num).fold(0, |sum, x| sum + x);
            })
        }
    )*};
}

bench_next! {
    bench_next100_boxed02, iter_boxed2, 2, 100,
    bench_next100_boxed16, iter_boxed16, 16, 100,
    bench_next1000_boxed02, iter_boxed2, 2, 1000,
    bench_next1000_boxed16, iter_boxed16, 16, 1000,
    bench_next100_enum02, iter_enum2, 2, 100,
    bench_next100_enum16, iter_enum16, 16, 100,
    bench_next1000_enum02, iter_enum2, 2, 1000,
    bench_next1000_enum16, iter_enum16, 16, 1000,
    bench_next100_no_branch, iter_no_branch, 10, 100,
    bench_next1000_no_branch, iter_no_branch, 10, 1000,
}

bench_fold! {
    bench_fold100_boxed02, iter_boxed2, 2, 100,
    bench_fold100_boxed16, iter_boxed16, 16, 100,
    bench_fold1000_boxed02, iter_boxed2, 2, 1000,
    bench_fold1000_boxed16, iter_boxed16, 16, 1000,
    bench_fold100_enum02, iter_enum2, 2, 100,
    bench_fold100_enum04, iter_enum4, 4, 100,
    bench_fold100_enum08, iter_enum8, 8, 100,
    bench_fold100_enum16, iter_enum16, 16, 100,
    bench_fold1000_enum02, iter_enum2, 2, 1000,
    bench_fold1000_enum04, iter_enum4, 4, 1000,
    bench_fold1000_enum08, iter_enum8, 8, 1000,
    bench_fold1000_enum16, iter_enum16, 16, 1000,
    bench_fold100_no_branch, iter_no_branch, 10, 100,
    bench_fold1000_no_branch, iter_no_branch, 10, 1000,
}

bench_fold_semi! {
    bench_fold100_boxed_semi02, iter_boxed2, 2, 100,
    bench_fold100_boxed_semi16, iter_boxed16, 16, 100,
    bench_fold1000_boxed_semi02, iter_boxed2, 2, 1000,
    bench_fold1000_boxed_semi16, iter_boxed16, 16, 1000,
    bench_fold100_enum_semi02, iter_enum2, 2, 100,
    bench_fold100_enum_semi16, iter_enum16, 16, 100,
    bench_fold1000_enum_semi02, iter_enum2, 2, 1000,
    bench_fold1000_enum_semi16, iter_enum16, 16, 1000,
    bench_fold100_no_branch_semi, iter_no_branch, 10, 100,
    bench_fold1000_no_branch_semi, iter_no_branch, 10, 1000,
}

/*
result:

running 34 tests
test bench_fold1000_boxed02        ... bench:       1,413 ns/iter (+/- 98)
test bench_fold1000_boxed16        ... bench:       1,608 ns/iter (+/- 158)
test bench_fold1000_boxed_semi02   ... bench:       1,661 ns/iter (+/- 110)
test bench_fold1000_boxed_semi16   ... bench:       1,388 ns/iter (+/- 70)
test bench_fold1000_enum02         ... bench:          97 ns/iter (+/- 6)
test bench_fold1000_enum04         ... bench:         173 ns/iter (+/- 7)
test bench_fold1000_enum08         ... bench:         677 ns/iter (+/- 25)
test bench_fold1000_enum16         ... bench:         829 ns/iter (+/- 27)
test bench_fold1000_enum_semi02    ... bench:          13 ns/iter (+/- 1)
test bench_fold1000_enum_semi16    ... bench:          13 ns/iter (+/- 1)
test bench_fold1000_no_branch      ... bench:          10 ns/iter (+/- 1)
test bench_fold1000_no_branch_semi ... bench:          10 ns/iter (+/- 1)
test bench_fold100_boxed02         ... bench:         204 ns/iter (+/- 15)
test bench_fold100_boxed16         ... bench:         212 ns/iter (+/- 15)
test bench_fold100_boxed_semi02    ... bench:         208 ns/iter (+/- 14)
test bench_fold100_boxed_semi16    ... bench:         212 ns/iter (+/- 25)
test bench_fold100_enum02          ... bench:          14 ns/iter (+/- 1)
test bench_fold100_enum04          ... bench:          16 ns/iter (+/- 1)
test bench_fold100_enum08          ... bench:          86 ns/iter (+/- 9)
test bench_fold100_enum16          ... bench:         120 ns/iter (+/- 8)
test bench_fold100_enum_semi02     ... bench:          14 ns/iter (+/- 3)
test bench_fold100_enum_semi16     ... bench:          13 ns/iter (+/- 1)
test bench_fold100_no_branch       ... bench:          10 ns/iter (+/- 1)
test bench_fold100_no_branch_semi  ... bench:          10 ns/iter (+/- 0)
test bench_next1000_boxed02        ... bench:       1,361 ns/iter (+/- 86)
test bench_next1000_boxed16        ... bench:       1,379 ns/iter (+/- 120)
test bench_next1000_enum02         ... bench:          14 ns/iter (+/- 1)
test bench_next1000_enum16         ... bench:          14 ns/iter (+/- 1)
test bench_next1000_no_branch      ... bench:          10 ns/iter (+/- 3)
test bench_next100_boxed02         ... bench:         210 ns/iter (+/- 13)
test bench_next100_boxed16         ... bench:         219 ns/iter (+/- 93)
test bench_next100_enum02          ... bench:          14 ns/iter (+/- 0)
test bench_next100_enum16          ... bench:          14 ns/iter (+/- 1)
test bench_next100_no_branch       ... bench:           9 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 0 ignored; 34 measured; 0 filtered out

*/
