use crate::utils::*;
use std::str;

const _SAMPLE: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn parse_num(input: &[u8]) -> Option<(&[u8], u32)> {
    match input {
        [first, second, third, input @ ..]
            if first.is_ascii_digit() && second.is_ascii_digit() && third.is_ascii_digit() =>
        {
            Some((
                input,
                (first - b'0') as u32 * 100 + (second - b'0') as u32 * 10 + (third - b'0') as u32,
            ))
        }
        [first, second, input @ ..] if first.is_ascii_digit() && second.is_ascii_digit() => {
            Some((input, (first - b'0') as u32 * 10 + (second - b'0') as u32))
        }
        [first, input @ ..] if first.is_ascii_digit() => Some((input, (first - b'0') as _)),
        _ => None,
    }
}

enum State {
    ExpectOpen,
    ExpectFirst,
    ExpectComma(u32),
    ExpectSecond(u32),
    ExpectClose(u32, u32),
}

pub fn part1(input: &str) -> Answer {
    let mut state = State::ExpectFirst;
    let mut input = input.as_bytes();
    let mut sum = 0;
    while !input.is_empty() {
        match (state, input) {
            (State::ExpectOpen, [b'm', b'u', b'l', b'(', rest @ ..]) => {
                state = State::ExpectFirst;
                input = rest;
            }
            (State::ExpectFirst, _) => {
                if let Some((rest, first)) = parse_num(input) {
                    state = State::ExpectComma(first);
                    input = rest;
                } else {
                    state = State::ExpectOpen;
                    input = &input[1..];
                }
            }
            (State::ExpectComma(first), [b',', rest @ ..]) => {
                state = State::ExpectSecond(first);
                input = rest;
            }
            (State::ExpectSecond(first), _) => {
                if let Some((rest, second)) = parse_num(input) {
                    state = State::ExpectClose(first, second);
                    input = rest;
                } else {
                    state = State::ExpectOpen;
                    input = &input[1..];
                }
            }
            (State::ExpectClose(first, second), [b')', rest @ ..]) => {
                sum += first * second;
                state = State::ExpectOpen;
                input = rest;
            }
            _ => {
                state = State::ExpectOpen;
                input = &input[1..];
            }
        }
    }
    sum.into()
}

pub fn part2(input: &str) -> Answer {
    let mut state = State::ExpectFirst;
    let mut input = input.as_bytes();
    let mut sum = 0;
    while !input.is_empty() {
        match (state, input) {
            (_, [b'd', b'o', b'n', b'\'', b't', b'(', b')', ..]) => {
                state = State::ExpectOpen;
                input = if let Some(do_at) = unsafe { str::from_utf8_unchecked(input) }.find("do()")
                {
                    &input[do_at..]
                } else {
                    &[]
                };
            }
            (State::ExpectOpen, [b'm', b'u', b'l', b'(', rest @ ..]) => {
                state = State::ExpectFirst;
                input = rest;
            }
            (State::ExpectFirst, _) => {
                if let Some((rest, first)) = parse_num(input) {
                    state = State::ExpectComma(first);
                    input = rest;
                } else {
                    state = State::ExpectOpen;
                    input = &input[1..];
                }
            }
            (State::ExpectComma(first), [b',', rest @ ..]) => {
                state = State::ExpectSecond(first);
                input = rest;
            }
            (State::ExpectSecond(first), _) => {
                if let Some((rest, second)) = parse_num(input) {
                    state = State::ExpectClose(first, second);
                    input = rest;
                } else {
                    state = State::ExpectOpen;
                    input = &input[1..];
                }
            }
            (State::ExpectClose(first, second), [b')', rest @ ..]) => {
                sum += first * second;
                state = State::ExpectOpen;
                input = rest;
            }
            _ => {
                state = State::ExpectOpen;
                input = &input[1..];
            }
        }
    }
    sum.into()
}
