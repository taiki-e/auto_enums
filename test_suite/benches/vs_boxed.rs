#![feature(test)]
#![feature(box_syntax)]
#![cfg_attr(feature = "unstable", feature(try_trait))]

extern crate test;

use auto_enums::auto_enum;
use rand::Rng;
use test::Bencher;

/*
This code will fail to compile(E0308).
fn iter(x: u32) -> impl Iterator<Item = i64> {
    match x {
        0 => (0..).map(|x| x + 1),
        _ => (0..).map(|x| x + 1),
    }
}
*/

fn iter_no_branch(_x: u32) -> impl Iterator<Item = i64> {
    (0..).map(|x| x + 2 - 1)
}

fn iter_boxed2(x: u32) -> Box<dyn Iterator<Item = i64>> {
    match x {
        0 => box (0..).map(|x| (x * 2) - 1),
        _ => box (0..).map(|x| (x + 1) / 2),
    }
}

fn iter_boxed16(x: u32) -> Box<dyn Iterator<Item = i64>> {
    match x {
        0 => box (0..).map(|x| (x * 2) * 2),
        1 => box (0..).map(|x| (x - 1) / 2),
        2 => box (0..).map(|x| (x * 2) - 1),
        3 => box (0..).map(|x| (x + 1) + 2),
        4 => box (0..).map(|x| (x / 2) - 1),
        5 => box (0..).map(|x| (x * 2) - 1),
        6 => box (0..).map(|x| (x / 2) + 1),
        7 => box (0..).map(|x| (x * 2) + 1),
        8 => box (0..).map(|x| (x / 2) / 2),
        9 => box (0..).map(|x| (x / 2) - 2),
        10 => box (0..).map(|x| (x + 2) * 2),
        11 => box (0..).map(|x| (x + 2) / 2),
        12 => box (0..).map(|x| (x - 1) - 2),
        13 => box (0..).map(|x| (x - 2) * 2),
        14 => box (0..).map(|x| (x - 2) / 2),
        _ => box (0..).map(|x| (x + 1) / 2),
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
fn iter_enum12(x: u32) -> impl Iterator<Item = i64> {
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
        10 => (0..).map(|x| (x / 2) + 1),
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
    bench_fold100_enum12, iter_enum12, 12, 100,
    bench_fold100_enum16, iter_enum16, 16, 100,
    bench_fold1000_enum02, iter_enum2, 2, 1000,
    bench_fold1000_enum04, iter_enum4, 4, 1000,
    bench_fold1000_enum08, iter_enum8, 8, 1000,
    bench_fold1000_enum12, iter_enum12, 12, 1000,
    bench_fold1000_enum16, iter_enum16, 16, 1000,
    bench_fold100_no_branch, iter_no_branch, 10, 100,
    bench_fold1000_no_branch, iter_no_branch, 10, 1000,
}

bench_fold_semi! {
    bench_fold100semi_boxed02, iter_boxed2, 2, 100,
    bench_fold100semi_boxed16, iter_boxed16, 16, 100,
    bench_fold1000semi_boxed02, iter_boxed2, 2, 1000,
    bench_fold1000semi_boxed16, iter_boxed16, 16, 1000,
    bench_fold100semi_enum02, iter_enum2, 2, 100,
    bench_fold100semi_enum16, iter_enum16, 16, 100,
    bench_fold1000semi_enum02, iter_enum2, 2, 1000,
    bench_fold1000semi_enum16, iter_enum16, 16, 1000,
    bench_fold100semi_no_branch, iter_no_branch, 10, 100,
    bench_fold1000semi_no_branch, iter_no_branch, 10, 1000,
}

/*
running 36 tests
test bench_fold1000_boxed02       ... bench:       2,111 ns/iter (+/- 177)
test bench_fold1000_boxed16       ... bench:       2,466 ns/iter (+/- 152)
test bench_fold1000_enum02        ... bench:         136 ns/iter (+/- 2)
test bench_fold1000_enum04        ... bench:         249 ns/iter (+/- 14)
test bench_fold1000_enum08        ... bench:       1,114 ns/iter (+/- 51)
test bench_fold1000_enum12        ... bench:       1,144 ns/iter (+/- 67)
test bench_fold1000_enum16        ... bench:       1,523 ns/iter (+/- 135)
test bench_fold1000_no_branch     ... bench:          14 ns/iter (+/- 0)
test bench_fold1000semi_boxed02   ... bench:       2,100 ns/iter (+/- 420)
test bench_fold1000semi_boxed16   ... bench:       2,133 ns/iter (+/- 193)
test bench_fold1000semi_enum02    ... bench:          19 ns/iter (+/- 0)
test bench_fold1000semi_enum16    ... bench:          19 ns/iter (+/- 0)
test bench_fold1000semi_no_branch ... bench:          14 ns/iter (+/- 0)
test bench_fold100_boxed02        ... bench:         309 ns/iter (+/- 28)
test bench_fold100_boxed16        ... bench:         320 ns/iter (+/- 35)
test bench_fold100_enum02         ... bench:          20 ns/iter (+/- 0)
test bench_fold100_enum04         ... bench:          22 ns/iter (+/- 0)
test bench_fold100_enum08         ... bench:         135 ns/iter (+/- 3)
test bench_fold100_enum12         ... bench:         185 ns/iter (+/- 14)
test bench_fold100_enum16         ... bench:         176 ns/iter (+/- 8)
test bench_fold100_no_branch      ... bench:          14 ns/iter (+/- 0)
test bench_fold100semi_boxed02    ... bench:         342 ns/iter (+/- 26)
test bench_fold100semi_boxed16    ... bench:         318 ns/iter (+/- 23)
test bench_fold100semi_enum02     ... bench:          20 ns/iter (+/- 0)
test bench_fold100semi_enum16     ... bench:          20 ns/iter (+/- 0)
test bench_fold100semi_no_branch  ... bench:          14 ns/iter (+/- 0)
test bench_next1000_boxed02       ... bench:       2,105 ns/iter (+/- 597)
test bench_next1000_boxed16       ... bench:       2,124 ns/iter (+/- 108)
test bench_next1000_enum02        ... bench:          20 ns/iter (+/- 0)
test bench_next1000_enum16        ... bench:          20 ns/iter (+/- 1)
test bench_next1000_no_branch     ... bench:          14 ns/iter (+/- 0)
test bench_next100_boxed02        ... bench:         336 ns/iter (+/- 35)
test bench_next100_boxed16        ... bench:         313 ns/iter (+/- 18)
test bench_next100_enum02         ... bench:          20 ns/iter (+/- 0)
test bench_next100_enum16         ... bench:          20 ns/iter (+/- 0)
test bench_next100_no_branch      ... bench:          14 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 0 ignored; 36 measured; 0 filtered out
*/
