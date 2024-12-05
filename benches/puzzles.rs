use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! day_bench {
    ($day:ident) => {
        pub fn $day(c: &mut Criterion) {
            let input = black_box(include_str!(concat!("../input/", stringify!($day), ".txt")));
            c.bench_function(concat!(stringify!($day), "_part1"), |b| {
                b.iter(|| aoc_2024::$day::part1(input))
            });
            c.bench_function(concat!(stringify!($day), "_part2"), |b| {
                b.iter(|| aoc_2024::$day::part2(input))
            });
        }
    };
}

day_bench!(day1);
day_bench!(day2);
day_bench!(day3);
day_bench!(day4);
day_bench!(day5);
day_bench!(day6);
day_bench!(day7);
day_bench!(day8);
day_bench!(day9);
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
        day1,
        day2,
        day3,
        day4,
        day5,
        day6,
        day7,
        day8,
        day9,
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
