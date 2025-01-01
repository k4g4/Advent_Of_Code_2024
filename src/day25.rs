use crate::utils::*;

const _SAMPLE: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

fn parse_locks_and_keys(input: &str) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let (mut locks, mut keys) = (vec![], vec![]);
    for schematic in input.split("\n\n") {
        let mut lines = schematic.lines();
        let top = lines.next().unwrap();
        let mut row = [0; 5];
        for line in lines.take(5) {
            let pins = line
                .chars()
                .enumerate()
                .filter_map(|(i, c)| (c == '#').then_some(i));
            for pin in pins {
                row[pin] += 1;
            }
        }
        if top == "#####" {
            locks.push(row);
        } else {
            keys.push(row);
        }
    }
    (locks, keys)
}

pub fn part1(input: &str) -> Answer {
    let (locks, keys) = parse_locks_and_keys(input);

    locks
        .into_iter()
        .cartesian_product(keys)
        .filter(|&(lock, key)| lock.iter().zip(key).all(|(l, k)| l + k <= 5))
        .count()
        .into()
}

pub fn part2(_input: &str) -> Answer {
    "".into()
}
