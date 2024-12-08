use crate::utils::*;

const SPACE: u8 = b'.';
const _SAMPLE: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

fn extrapolate((a_i, a_j): Index, (b_i, b_j): Index) -> Index {
    let (i_delta, j_delta) = (b_i - a_i, b_j - a_j);
    (b_i + i_delta, b_j + j_delta)
}

pub fn part1(input: &str) -> Answer {
    let grid = Grid::new(input);
    let (rows, cols) = grid.bounds();
    let mut antenna_freqs = HashMap::<_, HashSet<_>>::default();
    for (index, freq) in grid.indices().filter(|&(_, freq)| freq != SPACE) {
        antenna_freqs.entry(freq).or_default().insert(index);
    }
    let antinodes: HashSet<_> = antenna_freqs
        .iter()
        .flat_map(|(_, antennas)| {
            antennas
                .iter()
                .tuple_combinations()
                .flat_map(|(&first, &second)| {
                    [extrapolate(first, second), extrapolate(second, first)]
                })
                .filter(|(i, j)| (0..rows).contains(i) && (0..cols).contains(j))
        })
        .collect();

    antinodes.len().into()
}

fn antinodes(
    (row, col): Index,
    (a_i, a_j): Index,
    (b_i, b_j): Index,
) -> impl Iterator<Item = Index> {
    let (i_delta, j_delta) = (b_i - a_i, b_j - a_j);
    let gcd = gcd_isize(i_delta, j_delta);
    let (i_delta, j_delta) = (i_delta / gcd, j_delta / gcd);
    let in_bounds = move |(i, j): &_| (0..row).contains(i) && (0..col).contains(j);

    let forward = (0..)
        .zip(0..)
        .map(move |(i, j)| (a_i + (i_delta * i), a_j + (j_delta * j)))
        .take_while(in_bounds);
    let backward = (1..)
        .zip(1..)
        .map(move |(i, j)| (a_i - (i_delta * i), a_j - (j_delta * j)))
        .take_while(in_bounds);

    forward.chain(backward)
}

pub fn part2(input: &str) -> Answer {
    let grid = Grid::new(input);
    let mut antenna_freqs = HashMap::<_, HashSet<_>>::default();
    for (index, freq) in grid.indices().filter(|&(_, freq)| freq != SPACE) {
        antenna_freqs.entry(freq).or_default().insert(index);
    }
    let antinodes: HashSet<_> = antenna_freqs
        .iter()
        .flat_map(|(_, antennas)| {
            antennas
                .iter()
                .tuple_combinations()
                .flat_map(|(&first, &second)| antinodes(grid.bounds(), first, second))
        })
        .collect();

    antinodes.len().into()
}
