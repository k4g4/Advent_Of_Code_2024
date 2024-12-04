use crate::utils::*;

const _SAMPLE_1: &str = "\
..X...
.SAMX.
.A..A.
XMAS.S
.X....";

pub fn part1(input: &str) -> Answer {
    let grid = Grid::new(input);

    grid.indices()
        .filter(|&index| grid.get(index).unwrap() == b'X')
        .flat_map(|(row, column)| {
            let grid = &grid;
            DIRS.iter().filter(move |&(r, c)| {
                b"MAS".iter().zip(1..).all(|(&letter, i)| {
                    grid.get((row + r * i, column + c * i))
                        .is_some_and(|b| b == letter)
                })
            })
        })
        .count()
        .into()
}

const _SAMPLE_2: &str = "\
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

pub fn part2(input: &str) -> Answer {
    let grid = Grid::new(input);

    let is = |row, column, l| grid.get((row, column)).is_some_and(|b| b == l);

    grid.indices()
        .filter(|&index| grid.get(index).unwrap() == b'A')
        .filter(|&(r, c)| {
            (is(r - 1, c - 1, b'M')
                && is(r - 1, c + 1, b'M')
                && is(r + 1, c - 1, b'S')
                && is(r + 1, c + 1, b'S'))
                || (is(r - 1, c - 1, b'S')
                    && is(r - 1, c + 1, b'M')
                    && is(r + 1, c - 1, b'S')
                    && is(r + 1, c + 1, b'M'))
                || (is(r - 1, c - 1, b'S')
                    && is(r - 1, c + 1, b'S')
                    && is(r + 1, c - 1, b'M')
                    && is(r + 1, c + 1, b'M'))
                || (is(r - 1, c - 1, b'M')
                    && is(r - 1, c + 1, b'S')
                    && is(r + 1, c - 1, b'M')
                    && is(r + 1, c + 1, b'S'))
        })
        .count()
        .into()
}
