use aoc_2024::*;
use std::{env, fs, time::Instant};
use utils::*;

type Puzzle = fn(&str) -> Answer;

const DAYS: [(Puzzle, Puzzle); 26] = [
    (|_| unreachable!(), |_| unreachable!()),
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
    let get_input = |day| fs::read_to_string(format!("input/day{day:02}.txt")).unwrap();

    if let Some(arg) = env::args().nth(1) {
        let day = arg.parse().expect("Invalid argument, expected a number");
        let input = get_input(day);
        let (part1, part2) = DAYS[day];

        print!("Part 1: ");
        run_puzzle(&input, part1);
        print!("Part 2: ");
        run_puzzle(&input, part2);
    } else {
        for (day, &(part1, part2)) in DAYS.iter().enumerate().skip(1) {
            println!("Day {day:02}:");
            let input = get_input(day);

            print!("\tPart 1: ");
            run_puzzle(&input, part1);
            print!("\tPart 2: ");
            run_puzzle(&input, part2);
        }
    }
}

fn run_puzzle(input: &str, puzzle: Puzzle) {
    let time = Instant::now();
    if let Answer::Finished(answer) = puzzle(input) {
        let elapsed = time.elapsed();
        println!("{answer} ({elapsed:?})");
    } else {
        println!();
    }
}
