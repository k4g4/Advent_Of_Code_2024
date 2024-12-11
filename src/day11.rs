use crate::utils::*;

const _SAMPLE: &str = "125 17";

fn blink(stone: u64, blinks: u32, memo: &mut HashMap<(u64, u32), u64>) -> u64 {
    if blinks == 0 {
        1
    } else if stone == 0 {
        blink(1, blinks - 1, memo)
    } else if let Some(&stones) = memo.get(&(stone, blinks)) {
        stones
    } else {
        let digits = digits(stone);
        let stones = if digits % 2 == 0 {
            let mask = 10u64.pow(digits / 2);
            blink(stone / mask, blinks - 1, memo) + blink(stone % mask, blinks - 1, memo)
        } else {
            blink(stone * 2024, blinks - 1, memo)
        };
        memo.insert((stone, blinks), stones);
        stones
    }
}

fn count_stones(input: &str, blinks: u32) -> u64 {
    let mut memo = HashMap::default();
    input
        .split_ascii_whitespace()
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .map(|stone| blink(stone, blinks, &mut memo))
        .sum()
}

pub fn part1(input: &str) -> Answer {
    count_stones(input, 25).into()
}

pub fn part2(input: &str) -> Answer {
    count_stones(input, 75).into()
}
