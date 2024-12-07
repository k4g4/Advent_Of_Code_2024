use crate::utils::*;
use std::{hint::unreachable_unchecked, iter};

const ARRAY_LEN: usize = 16;
const _SAMPLE: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

fn parse_line(line: &str) -> (u64, ArrayVec<[u64; ARRAY_LEN]>) {
    let (test_val, line) = parse(line);
    let nums = line[2..]
        .split_ascii_whitespace()
        .map(|l| parse(l).0)
        .collect();
    (test_val, nums)
}

pub fn part1(input: &str) -> Answer {
    input
        .par_lines()
        .map(parse_line)
        .filter(|&(test_val, nums)| {
            (0..2u64.pow((nums.len() - 1) as _)).any(|binary| {
                let val = (0..nums.len() - 1).fold(nums[0], |val, i| {
                    if (binary >> i) % 2 == 0 {
                        val * nums[i + 1]
                    } else {
                        val + nums[i + 1]
                    }
                });
                val == test_val
            })
        })
        .map(|(test_val, _)| test_val)
        .sum::<u64>()
        .into()
}

const CACHE_LEN: usize = 8;

fn digits(mut num: u64) -> u32 {
    if num == 0 {
        1
    } else {
        let mut num_len = 0;
        while num > 0 {
            num /= 10;
            num_len += 1;
        }
        num_len
    }
}

fn calibrations(nums: &[u64]) -> impl Iterator<Item = u64> + use<'_> {
    (0..3u64.pow((nums.len() - 1) as _)).map(|mut ops| {
        iter::from_fn(|| {
            let op = ops % 3;
            ops /= 3;
            Some(op)
        })
        .zip(&nums[1..])
        .fold(nums[0], |val, (op, &num)| match op % 3 {
            0 => val + num,
            1 => val * num,
            2 => val * 10u64.pow(digits(num)) + num,
            _ => unsafe { unreachable_unchecked() },
        })
    })
}

pub fn part2(input: &str) -> Answer {
    input
        .par_lines()
        .map(parse_line)
        .filter(|&(test_val, nums)| {
            if nums.len() > CACHE_LEN {
                let cached: HashSet<_> = calibrations(&nums[..CACHE_LEN]).collect();
                let short_nums: ArrayVec<[u64; ARRAY_LEN - CACHE_LEN]> =
                    nums[CACHE_LEN..].iter().copied().collect();

                cached.iter().any(|&num| {
                    let nums: ArrayVec<[u64; ARRAY_LEN - CACHE_LEN + 1]> =
                        iter::once(num).chain(short_nums).collect();
                    let mut cals = calibrations(&nums);
                    cals.any(|val| val == test_val)
                })
            } else {
                calibrations(&nums).any(|val| val == test_val)
            }
        })
        .map(|(test_val, _)| test_val)
        .sum::<u64>()
        .into()
}
