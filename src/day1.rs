use crate::utils::*;

pub fn part1(input: &str) -> Answer {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let (left, line) = parse::<i64>(line);
            let (right, _) = parse::<i64>(line.trim_ascii_start());
            (left, right)
        })
        .unzip();
    left.sort_unstable();
    right.sort_unstable();
    left.into_iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum::<u64>()
        .into()
}

pub fn part2(input: &str) -> Answer {
    let left = input.lines().map(|line| parse(line).0);
    let right = input.lines().map(|line| {
        let (_, line) = line.split_once("   ").unwrap();
        parse(line).0
    });
    let counter: Counter<i64> = right.collect();
    left.map(|n| n * (counter.get(&n).copied().unwrap_or(0) as i64))
        .sum::<i64>()
        .into()
}
