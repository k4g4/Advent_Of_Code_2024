use crate::utils::*;

const SAMPLE: &str = "125 17";

#[derive(Debug)]
struct Stone {
    n: u64,
    next: usize,
}

fn parse_stones(input: &str) -> Vec<Stone> {
    input
        .split_ascii_whitespace()
        .map(str::parse::<u64>)
        .zip(1..)
        .map(|(result, next)| result.map(|n| Stone { n, next }))
        .try_collect()
        .unwrap()
}

fn print(stones: &[Stone]) {
    let mut at = 0;
    while let Some(Stone { n, next }) = stones.get(at) {
        at = *next;
        print!("{n} ");
    }
    println!();
}

fn update(mut stones: Vec<Stone>) -> Vec<Stone> {
    let mut at = 0;
    while let Some(Stone { n, next }) = stones.get_mut(at) {
        at = *next;
        if *n == 0 {
            *n = 1;
        } else {
            let digits = digits(*n);
            if digits % 2 == 0 {
                todo!();
            } else {
                *n *= 2024;
            }
        }
    }
    stones
}

pub fn part1(_input: &str) -> Answer {
    let mut stones = parse_stones(SAMPLE);
    for _ in 0..6 {
        print(&stones);
        stones = update(stones);
    }
    Answer::Unfinished
}

pub fn part2(_input: &str) -> Answer {
    Answer::Unfinished
}
