use crate::utils::*;

const _SAMPLE: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

fn parse_towels(input: &str) -> (HashSet<&[u8]>, impl Iterator<Item = &[u8]>) {
    let mut lines = input.lines();
    let patterns = lines
        .next()
        .unwrap()
        .split(", ")
        .map(str::as_bytes)
        .collect();
    lines.next();
    (patterns, lines.map(str::as_bytes))
}

pub fn part1(input: &str) -> Answer {
    fn valid_towel(towel: &[u8], patterns: &HashSet<&[u8]>, pattern_max: usize) -> bool {
        towel.is_empty()
            || (1..=pattern_max)
                .rev()
                .filter_map(|pattern_len| towel.split_at_checked(pattern_len))
                .any(|(pattern, rest)| {
                    patterns.contains(pattern) && valid_towel(rest, patterns, pattern_max)
                })
    }

    let (patterns, towels) = parse_towels(input);
    let pattern_max = patterns.iter().map(|pattern| pattern.len()).max().unwrap();
    towels
        .filter(|towel| valid_towel(towel, &patterns, pattern_max))
        .count()
        .into()
}

pub fn part2(input: &str) -> Answer {
    fn count_towels<'a>(
        towel: &'a [u8],
        patterns: &HashSet<&[u8]>,
        pattern_max: usize,
        memo: &mut HashMap<&'a [u8], u64>,
    ) -> u64 {
        if let Some(&count) = memo.get(towel) {
            count
        } else {
            let count = if towel.is_empty() {
                1
            } else {
                (1..=pattern_max)
                    .rev()
                    .filter_map(|pattern_len| towel.split_at_checked(pattern_len))
                    .filter_map(|(pattern, rest)| {
                        patterns
                            .contains(pattern)
                            .then(|| count_towels(rest, patterns, pattern_max, memo))
                    })
                    .sum()
            };
            memo.insert(towel, count);
            count
        }
    }

    let (patterns, towels) = parse_towels(input);
    let pattern_max = patterns.iter().map(|pattern| pattern.len()).max().unwrap();
    let mut memo = HashMap::default();
    towels
        .map(|towel| count_towels(towel, &patterns, pattern_max, &mut memo))
        .sum::<u64>()
        .into()
}
