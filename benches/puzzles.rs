use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! day_bench {
    ($title:ident) => {
        pub fn $title(c: &mut Criterion) {
            let input = black_box(include_str!(concat!(
                "../input/",
                stringify!($title),
                ".txt"
            )));
            c.bench_function(concat!(stringify!($title), " part 1"), |b| {
                b.iter(|| aoc_2024::$title::part1(input))
            });
            c.bench_function(concat!(stringify!($title), " part 2"), |b| {
                b.iter(|| aoc_2024::$title::part2(input))
            });
        }
    };
}

day_bench!(day01);
day_bench!(day02);
day_bench!(day03);
day_bench!(day04);
day_bench!(day05);
day_bench!(day06);
day_bench!(day07);
day_bench!(day08);
day_bench!(day09);
day_bench!(day10);
day_bench!(day11);
day_bench!(day12);
day_bench!(day13);
day_bench!(day14);
day_bench!(day15);
day_bench!(day16);
day_bench!(day17);
day_bench!(day18);
day_bench!(day19);
day_bench!(day20);
day_bench!(day21);
day_bench!(day22);
day_bench!(day23);
day_bench!(day24);
day_bench!(day25);

criterion_group! {
    name = puzzles;
    config = Criterion::default();
    targets =
        day01,
        day02,
        day03,
        day04,
        day05,
        day06,
        day07,
        day08,
        day09,
        day10,
        day11,
        day12,
        day13,
        day14,
        day15,
        day16,
        day17,
        day18,
        day19,
        day20,
        day21,
        day22,
        day23,
        day24,
        day25,
}
criterion_main!(puzzles);
