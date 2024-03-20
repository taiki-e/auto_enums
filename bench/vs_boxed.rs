// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Note that this benchmark currently only measures a fairly narrow area.

$ cargo bench | tee bench.txt

Benchmarking bench_next100_boxed02
Benchmarking bench_next100_boxed02: Warming up for 3.0000 s
Benchmarking bench_next100_boxed02: Collecting 100 samples in estimated 5.0003 s (21M iterations)
Benchmarking bench_next100_boxed02: Analyzing
bench_next100_boxed02   time:   [241.11 ns 246.72 ns 253.88 ns]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

Benchmarking bench_next100_boxed16
Benchmarking bench_next100_boxed16: Warming up for 3.0000 s
Benchmarking bench_next100_boxed16: Collecting 100 samples in estimated 5.0002 s (21M iterations)
Benchmarking bench_next100_boxed16: Analyzing
bench_next100_boxed16   time:   [233.58 ns 234.49 ns 235.46 ns]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) high mild
  8 (8.00%) high severe

Benchmarking bench_next1000_boxed02
Benchmarking bench_next1000_boxed02: Warming up for 3.0000 s
Benchmarking bench_next1000_boxed02: Collecting 100 samples in estimated 5.0009 s (3.5M iterations)
Benchmarking bench_next1000_boxed02: Analyzing
bench_next1000_boxed02  time:   [1.4296 us 1.4377 us 1.4471 us]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

Benchmarking bench_next1000_boxed16
Benchmarking bench_next1000_boxed16: Warming up for 3.0000 s
Benchmarking bench_next1000_boxed16: Collecting 100 samples in estimated 5.0050 s (3.1M iterations)
Benchmarking bench_next1000_boxed16: Analyzing
bench_next1000_boxed16  time:   [1.6154 us 1.6235 us 1.6315 us]
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

Benchmarking bench_next100_enum02
Benchmarking bench_next100_enum02: Warming up for 3.0000 s
Benchmarking bench_next100_enum02: Collecting 100 samples in estimated 5.0001 s (109M iterations)
Benchmarking bench_next100_enum02: Analyzing
bench_next100_enum02    time:   [44.700 ns 44.918 ns 45.145 ns]
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) high mild
  8 (8.00%) high severe

Benchmarking bench_next100_enum16
Benchmarking bench_next100_enum16: Warming up for 3.0000 s
Benchmarking bench_next100_enum16: Collecting 100 samples in estimated 5.0005 s (22M iterations)
Benchmarking bench_next100_enum16: Analyzing
bench_next100_enum16    time:   [224.92 ns 226.11 ns 227.32 ns]
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

Benchmarking bench_next1000_enum02
Benchmarking bench_next1000_enum02: Warming up for 3.0000 s
Benchmarking bench_next1000_enum02: Collecting 100 samples in estimated 5.0009 s (18M iterations)
Benchmarking bench_next1000_enum02: Analyzing
bench_next1000_enum02   time:   [269.51 ns 270.73 ns 271.87 ns]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

Benchmarking bench_next1000_enum16
Benchmarking bench_next1000_enum16: Warming up for 3.0000 s
Benchmarking bench_next1000_enum16: Collecting 100 samples in estimated 5.0072 s (2.4M iterations)
Benchmarking bench_next1000_enum16: Analyzing
bench_next1000_enum16   time:   [2.0585 us 2.0707 us 2.0841 us]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

Benchmarking bench_next100_no_branch
Benchmarking bench_next100_no_branch: Warming up for 3.0000 s
Benchmarking bench_next100_no_branch: Collecting 100 samples in estimated 5.0002 s (150M iterations)
Benchmarking bench_next100_no_branch: Analyzing
bench_next100_no_branch time:   [33.272 ns 33.487 ns 33.717 ns]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

Benchmarking bench_next1000_no_branch
Benchmarking bench_next1000_no_branch: Warming up for 3.0000 s
Benchmarking bench_next1000_no_branch: Collecting 100 samples in estimated 5.0011 s (20M iterations)
Benchmarking bench_next1000_no_branch: Analyzing
bench_next1000_no_branch
                        time:   [254.23 ns 255.49 ns 256.81 ns]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

Benchmarking bench_fold100_boxed02
Benchmarking bench_fold100_boxed02: Warming up for 3.0000 s
Benchmarking bench_fold100_boxed02: Collecting 100 samples in estimated 5.0009 s (22M iterations)
Benchmarking bench_fold100_boxed02: Analyzing
bench_fold100_boxed02   time:   [222.81 ns 223.66 ns 224.50 ns]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe

Benchmarking bench_fold100_boxed16
Benchmarking bench_fold100_boxed16: Warming up for 3.0000 s
Benchmarking bench_fold100_boxed16: Collecting 100 samples in estimated 5.0007 s (21M iterations)
Benchmarking bench_fold100_boxed16: Analyzing
bench_fold100_boxed16   time:   [236.27 ns 239.32 ns 243.56 ns]
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

Benchmarking bench_fold1000_boxed02
Benchmarking bench_fold1000_boxed02: Warming up for 3.0000 s
Benchmarking bench_fold1000_boxed02: Collecting 100 samples in estimated 5.0047 s (3.3M iterations)
Benchmarking bench_fold1000_boxed02: Analyzing
bench_fold1000_boxed02  time:   [1.5188 us 1.5321 us 1.5448 us]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

Benchmarking bench_fold1000_boxed16
Benchmarking bench_fold1000_boxed16: Warming up for 3.0000 s
Benchmarking bench_fold1000_boxed16: Collecting 100 samples in estimated 5.0053 s (3.2M iterations)
Benchmarking bench_fold1000_boxed16: Analyzing
bench_fold1000_boxed16  time:   [1.5448 us 1.5514 us 1.5580 us]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

Benchmarking bench_fold100_enum02
Benchmarking bench_fold100_enum02: Warming up for 3.0000 s
Benchmarking bench_fold100_enum02: Collecting 100 samples in estimated 5.0000 s (135M iterations)
Benchmarking bench_fold100_enum02: Analyzing
bench_fold100_enum02    time:   [37.265 ns 37.611 ns 37.972 ns]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

Benchmarking bench_fold100_enum04
Benchmarking bench_fold100_enum04: Warming up for 3.0000 s
Benchmarking bench_fold100_enum04: Collecting 100 samples in estimated 5.0002 s (71M iterations)
Benchmarking bench_fold100_enum04: Analyzing
bench_fold100_enum04    time:   [70.862 ns 71.172 ns 71.471 ns]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

Benchmarking bench_fold100_enum08
Benchmarking bench_fold100_enum08: Warming up for 3.0000 s
Benchmarking bench_fold100_enum08: Collecting 100 samples in estimated 5.0005 s (41M iterations)
Benchmarking bench_fold100_enum08: Analyzing
bench_fold100_enum08    time:   [122.96 ns 124.18 ns 125.64 ns]
Found 13 outliers among 100 measurements (13.00%)
  6 (6.00%) high mild
  7 (7.00%) high severe

Benchmarking bench_fold100_enum16
Benchmarking bench_fold100_enum16: Warming up for 3.0000 s
Benchmarking bench_fold100_enum16: Collecting 100 samples in estimated 5.0002 s (21M iterations)
Benchmarking bench_fold100_enum16: Analyzing
bench_fold100_enum16    time:   [227.24 ns 228.92 ns 231.00 ns]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

Benchmarking bench_fold1000_enum02
Benchmarking bench_fold1000_enum02: Warming up for 3.0000 s
Benchmarking bench_fold1000_enum02: Collecting 100 samples in estimated 5.0006 s (18M iterations)
Benchmarking bench_fold1000_enum02: Analyzing
bench_fold1000_enum02   time:   [269.27 ns 272.76 ns 277.08 ns]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

Benchmarking bench_fold1000_enum04
Benchmarking bench_fold1000_enum04: Warming up for 3.0000 s
Benchmarking bench_fold1000_enum04: Collecting 100 samples in estimated 5.0027 s (9.5M iterations)
Benchmarking bench_fold1000_enum04: Analyzing
bench_fold1000_enum04   time:   [521.78 ns 524.98 ns 528.64 ns]
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe

Benchmarking bench_fold1000_enum08
Benchmarking bench_fold1000_enum08: Warming up for 3.0000 s
Benchmarking bench_fold1000_enum08: Collecting 100 samples in estimated 5.0012 s (5.7M iterations)
Benchmarking bench_fold1000_enum08: Analyzing
bench_fold1000_enum08   time:   [871.89 ns 877.03 ns 883.76 ns]
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe

Benchmarking bench_fold1000_enum16
Benchmarking bench_fold1000_enum16: Warming up for 3.0000 s
Benchmarking bench_fold1000_enum16: Collecting 100 samples in estimated 5.0061 s (2.2M iterations)
Benchmarking bench_fold1000_enum16: Analyzing
bench_fold1000_enum16   time:   [2.2774 us 2.2936 us 2.3108 us]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

Benchmarking bench_fold100_no_branch
Benchmarking bench_fold100_no_branch: Warming up for 3.0000 s
Benchmarking bench_fold100_no_branch: Collecting 100 samples in estimated 5.0001 s (154M iterations)
Benchmarking bench_fold100_no_branch: Analyzing
bench_fold100_no_branch time:   [32.217 ns 32.431 ns 32.671 ns]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

Benchmarking bench_fold1000_no_branch
Benchmarking bench_fold1000_no_branch: Warming up for 3.0000 s
Benchmarking bench_fold1000_no_branch: Collecting 100 samples in estimated 5.0012 s (19M iterations)
Benchmarking bench_fold1000_no_branch: Analyzing
bench_fold1000_no_branch
                        time:   [262.30 ns 264.91 ns 267.71 ns]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

*/

use std::hint::black_box;

use auto_enums::auto_enum;
use criterion::{criterion_group, criterion_main, Criterion};

fn iter_no_branch(_x: u32) -> impl Iterator<Item = i64> {
    (0..).map(|x| black_box(x + 2 - 1))
}

fn iter_boxed2(x: u32) -> Box<dyn Iterator<Item = i64>> {
    match x % 2 {
        0 => Box::new((0..).map(|x| black_box((x * 2) - 1))),
        _ => Box::new((0..).map(|x| black_box((x + 1) / 2))),
    }
}

fn iter_boxed16(x: u32) -> Box<dyn Iterator<Item = i64>> {
    match x % 16 {
        0 => Box::new((0..).map(|x| black_box((x * 2) * 2))),
        1 => Box::new((0..).map(|x| black_box((x - 1) / 2))),
        2 => Box::new((0..).map(|x| black_box((x * 2) - 1))),
        3 => Box::new((0..).map(|x| black_box((x + 1) + 2))),
        4 => Box::new((0..).map(|x| black_box((x / 2) - 1))),
        5 => Box::new((0..).map(|x| black_box((x * 2) - 1))),
        6 => Box::new((0..).map(|x| black_box((x / 2) + 1))),
        7 => Box::new((0..).map(|x| black_box((x * 2) + 1))),
        8 => Box::new((0..).map(|x| black_box((x / 2) / 2))),
        9 => Box::new((0..).map(|x| black_box((x / 2) - 2))),
        10 => Box::new((0..).map(|x| black_box((x + 2) * 2))),
        11 => Box::new((0..).map(|x| black_box((x + 2) / 2))),
        12 => Box::new((0..).map(|x| black_box((x - 1) - 2))),
        13 => Box::new((0..).map(|x| black_box((x - 2) * 2))),
        14 => Box::new((0..).map(|x| black_box((x - 2) / 2))),
        _ => Box::new((0..).map(|x| black_box((x + 1) / 2))),
    }
}

#[auto_enum(Iterator)]
fn iter_enum2(x: u32) -> impl Iterator<Item = i64> {
    match x % 2 {
        0 => (0..).map(|x| black_box((x * 2) - 1)),
        _ => (0..).map(|x| black_box((x + 1) / 2)),
    }
}

#[auto_enum(Iterator)]
fn iter_enum4(x: u32) -> impl Iterator<Item = i64> {
    match x % 4 {
        0 => (0..).map(|x| black_box((x * 2) * 2)),
        1 => (0..).map(|x| black_box((x - 1) / 2)),
        2 => (0..).map(|x| black_box((x * 2) - 1)),
        _ => (0..).map(|x| black_box((x + 1) / 2)),
    }
}

#[auto_enum(Iterator)]
fn iter_enum8(x: u32) -> impl Iterator<Item = i64> {
    match x % 8 {
        0 => (0..).map(|x| black_box((x * 2) * 2)),
        1 => (0..).map(|x| black_box((x - 1) / 2)),
        2 => (0..).map(|x| black_box((x * 2) - 1)),
        3 => (0..).map(|x| black_box((x + 1) + 2)),
        4 => (0..).map(|x| black_box((x / 2) - 1)),
        5 => (0..).map(|x| black_box((x * 2) - 1)),
        6 => (0..).map(|x| black_box((x / 2) + 1)),
        _ => (0..).map(|x| black_box((x + 1) / 2)),
    }
}

#[auto_enum(Iterator)]
fn iter_enum16(x: u32) -> impl Iterator<Item = i64> {
    match x % 16 {
        0 => (0..).map(|x| black_box((x * 2) * 2)),
        1 => (0..).map(|x| black_box((x - 1) / 2)),
        2 => (0..).map(|x| black_box((x * 2) - 1)),
        3 => (0..).map(|x| black_box((x + 1) + 2)),
        4 => (0..).map(|x| black_box((x / 2) - 1)),
        5 => (0..).map(|x| black_box((x * 2) - 1)),
        6 => (0..).map(|x| black_box((x / 2) + 1)),
        7 => (0..).map(|x| black_box((x * 2) + 1)),
        8 => (0..).map(|x| black_box((x / 2) / 2)),
        9 => (0..).map(|x| black_box((x / 2) - 2)),
        10 => (0..).map(|x| black_box((x + 2) * 2)),
        11 => (0..).map(|x| black_box((x + 2) / 2)),
        12 => (0..).map(|x| black_box((x - 1) - 2)),
        13 => (0..).map(|x| black_box((x - 2) * 2)),
        14 => (0..).map(|x| black_box((x - 2) / 2)),
        _ => (0..).map(|x| black_box((x + 1) / 2)),
    }
}

macro_rules! bench_next {
    ($($fn:ident, $iter:ident, $max:expr, $num:expr,)*) => {
        $(
            fn $fn(c: &mut Criterion) {
                let mut rng = fastrand::Rng::new();
                c.bench_function(stringify!($fn), |b| {
                    b.iter(|| {
                        let mut iter = $iter(rng.u32(0..$max));
                        (0..$num).for_each(|_| assert!(iter.next().is_some()))
                    })
                });
            }
        )*
        criterion_group!(bench_next, $($fn),*);
    };
}

macro_rules! bench_fold {
    ($($fn:ident, $iter:ident, $max:expr, $num:expr,)*) => {
        $(
            fn $fn(c: &mut Criterion) {
                let mut rng = fastrand::Rng::new();
                c.bench_function(stringify!($fn), |b| {
                    b.iter(|| {
                        let iter = $iter(rng.u32(0..$max));
                        iter.take($num).fold(0, |sum, x| sum + x)
                    })
                });
            }
        )*
        criterion_group!(bench_fold, $($fn),*);
    };
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

criterion_main!(bench_next, bench_fold);
