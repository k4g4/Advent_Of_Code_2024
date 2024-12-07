use crate::utils::*;
use std::{hint::unreachable_unchecked, iter};

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

pub fn part1(input: &str) -> Answer {
    fn parse_line(line: &str) -> (u64, ArrayVec<[u64; 16]>) {
        let (test_val, line) = parse(line);
        let nums = line[2..]
            .split_ascii_whitespace()
            .map(|l| parse(l).0)
            .collect();
        (test_val, nums)
    }

    input
        .lines()
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

pub fn part2(input: &str) -> Answer {
    fn parse_line(line: &str) -> (u64, ArrayVec<[(u64, u8); 16]>) {
        let (test_val, line) = parse(line);
        let nums = line[2..]
            .split_ascii_whitespace()
            .map(|l| (parse(l).0, l.len() as _))
            .collect();
        (test_val, nums)
    }

    input
        .lines()
        .map(parse_line)
        .filter(|&(test_val, nums)| {
            (0..3u64.pow((nums.len() - 1) as _)).any(|mut ops| {
                let val = iter::from_fn(|| {
                    let op = ops % 3;
                    ops /= 3;
                    Some(op)
                })
                .take(nums.len() - 1)
                .zip(&nums[1..])
                .fold(nums[0].0, |val, (op, &(num, num_len))| match op % 3 {
                    0 => val + num,
                    1 => val * num,
                    2 => val * 10u64.pow(num_len.into()) + num,
                    _ => unsafe { unreachable_unchecked() },
                });
                val == test_val
            })
        })
        .map(|(test_val, _)| test_val)
        .sum::<u64>()
        .into()
}
