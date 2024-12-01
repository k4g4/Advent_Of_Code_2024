#![allow(unused)]

use std::{fs, time::Instant};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod utils;

type Puzzle = fn(&str) -> Option<i64>;
type Day = (Puzzle, Puzzle);

const DAYS: [Day; 25] = [
    (day01::part1, day01::part2),
    (day02::part1, day02::part2),
    (day03::part1, day03::part2),
    (day04::part1, day04::part2),
    (day05::part1, day05::part2),
    (day06::part1, day06::part2),
    (day07::part1, day07::part2),
    (day08::part1, day08::part2),
    (day09::part1, day09::part2),
    (day10::part1, day10::part2),
    (day11::part1, day11::part2),
    (day12::part1, day12::part2),
    (day13::part1, day13::part2),
    (day14::part1, day14::part2),
    (day15::part1, day15::part2),
    (day16::part1, day16::part2),
    (day17::part1, day17::part2),
    (day18::part1, day18::part2),
    (day19::part1, day19::part2),
    (day20::part1, day20::part2),
    (day21::part1, day21::part2),
    (day22::part1, day22::part2),
    (day23::part1, day23::part2),
    (day24::part1, day24::part2),
    (day25::part1, day25::part2),
];

fn main() {
    for ((part1, part2), day) in DAYS.iter().zip(1..) {
        println!("Day {day:02}:");
        let input = fs::read_to_string(format!("input/day{day:02}.txt")).unwrap();
        let time = Instant::now();
        if let Some(answer) = part1(&input) {
            let elapsed = time.elapsed();
            println!("\tPart 1: {answer}");
            println!("\t({elapsed:?})");
        }
        let time = Instant::now();
        if let Some(answer) = part2(&input) {
            let elapsed = time.elapsed();
            println!("\tPart 2: {answer}");
            println!("\t({elapsed:?})");
        }
    }
}
