use crate::utils::*;
use std::iter;

const LIMIT: usize = 2000;
const _SAMPLE: &str = "\
1
2
3
2024";

fn secrets(secret: u64) -> impl Iterator<Item = u64> {
    const MASK: u64 = 0xFFFFFF;

    iter::successors(Some(secret), move |&secret| {
        let secret = ((secret << 6) ^ secret) & MASK;
        let secret = ((secret >> 5) ^ secret) & MASK;
        Some(((secret << 11) ^ secret) & MASK)
    })
}

pub fn part1(input: &str) -> Answer {
    input
        .par_lines()
        .map(|line| parse(line).0)
        .map(|secret| secrets(secret).nth(LIMIT).unwrap())
        .sum::<u64>()
        .into()
}

pub fn part2(input: &str) -> Answer {
    input
        .par_lines()
        .map(|line| parse(line).0)
        .map(secrets)
        .map(|secrets| {
            secrets
                .take(LIMIT)
                .map(|secret| (secret % 10) as i64)
                .tuple_windows()
        })
        .flat_map(|windows| {
            windows.fold(HashMap::default(), |mut diffs, (s0, s1, s2, s3, s4)| {
                diffs
                    .entry([s1 - s0, s2 - s1, s3 - s2, s4 - s3])
                    .or_insert(s4);
                diffs
            })
        })
        .fold(HashMap::default, |mut collected, (changes, bananas)| {
            *collected.entry(changes).or_insert(0) += bananas;
            collected
        })
        .reduce(HashMap::default, |mut left, right| {
            for (changes, bananas) in right {
                *left.entry(changes).or_insert(0) += bananas;
            }
            left
        })
        .values()
        .copied()
        .max()
        .unwrap()
        .into()
}
