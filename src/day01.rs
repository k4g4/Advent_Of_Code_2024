use crate::utils::Freqs;

pub fn part1(input: &str) -> Option<i64> {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut columns = line.split_whitespace();
            (
                columns.next().unwrap().parse::<i64>().unwrap(),
                columns.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .unzip();
    left.sort_unstable();
    right.sort_unstable();
    let answer = left
        .into_iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right) as i64)
        .sum();
    Some(answer)
}

pub fn part2(input: &str) -> Option<i64> {
    let left = input.lines().map(|line| {
        line.split_whitespace()
            .next()
            .unwrap()
            .parse::<i64>()
            .unwrap()
    });
    let right = input.lines().map(|line| {
        line.split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<i64>()
            .unwrap()
    });
    let freqs: Freqs<_> = right.collect();
    let answer = left
        .map(|n| n * (*freqs.get(&n).unwrap_or(&0) as i64))
        .sum();
    Some(answer)
}
