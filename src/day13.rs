use crate::utils::*;
use std::array;

const _SAMPLE: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

fn parse_line(line: &str) -> (i64, i64) {
    let (_, x_and_y) = line.split_once(": ").unwrap();
    let (x, y) = x_and_y.split_once(", ").unwrap();
    let (x, _) = parse(&x[2..]);
    let (y, _) = parse(&y[2..]);
    (x, y)
}

fn calc_claw([(a_x, a_y), (b_x, b_y), (prize_x, prize_y)]: [(i64, i64); 3]) -> Option<i64> {
    let (numer, denom) = (a_x * prize_y - a_y * prize_x, a_x * b_y - a_y * b_x);
    (numer % denom == 0).then(|| numer / denom).and_then(|b| {
        let (numer, denom) = (-b_x * b + prize_x, a_x);
        (numer % denom == 0).then(|| 3 * numer / denom + b)
    })
}

pub fn part1(input: &str) -> Answer {
    let mut lines = input.lines();
    let mut tokens = 0;
    loop {
        let claw = array::from_fn(|_| parse_line(lines.next().unwrap()));
        tokens += calc_claw(claw).unwrap_or(0);
        if lines.next().is_none() {
            break;
        }
    }
    tokens.into()
}

const CORRECTION: i64 = 10_000_000_000_000;

pub fn part2(input: &str) -> Answer {
    let mut lines = input.lines();
    let mut tokens = 0;
    loop {
        let mut claw = array::from_fn(|_| parse_line(lines.next().unwrap()));
        claw[2].0 += CORRECTION;
        claw[2].1 += CORRECTION;
        tokens += calc_claw(claw).unwrap_or(0);
        if lines.next().is_none() {
            break;
        }
    }
    tokens.into()
}
