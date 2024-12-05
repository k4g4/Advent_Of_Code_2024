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
        .filter(|&(_, letter)| letter == b'X')
        .flat_map(|((row, column), _)| {
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

    grid.indices()
        .filter(|&(_, letter)| letter == b'A')
        .filter(|&(index, _)| {
            let test = |letters: &[_; 4]| {
                let diags = diags(index);
                (0..4).all(|i| grid.get(diags[i]).is_some_and(|b| b == letters[i]))
            };
            test(b"MMSS") || test(b"SMSM") || test(b"SSMM") || test(b"MSMS")
        })
        .count()
        .into()
}
