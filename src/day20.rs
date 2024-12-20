use crate::utils::*;

const THRESHOLD: i64 = 100;
const BACKWARDS: Dir = Dir::West;

const _SAMPLE: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

pub fn part1(input: &str) -> Answer {
    let grid = Grid::new(input);
    let mut tiles = HashMap::default();
    let mut pos = grid
        .iter()
        .find_map(|(i, b)| (b == 'S').then_some(i))
        .unwrap();
    let mut backwards = BACKWARDS;
    let start = pos;

    for dist in 0.. {
        tiles.insert(pos, dist);
        if unsafe { grid.get(pos).unwrap_unchecked() } == 'E' {
            break;
        }
        for &dir in [Dir::North, Dir::East, Dir::South, Dir::West]
            .iter()
            .filter(|&&dir| dir != backwards)
        {
            if unsafe { grid.get(pos + dir).unwrap_unchecked() } != '#' {
                pos += dir;
                backwards = dir.clockwise().clockwise();
                break;
            }
        }
    }

    pos = start;
    backwards = BACKWARDS;

    let mut count = 0;
    while unsafe { grid.get(pos).unwrap_unchecked() } != 'E' {
        let &current = tiles.get(&pos).unwrap();
        let mut progress = Dir::North;
        for &dir in [Dir::North, Dir::East, Dir::South, Dir::West]
            .iter()
            .filter(|&&dir| dir != backwards)
        {
            if unsafe { grid.get(pos + dir).unwrap_unchecked() } == '#' {
                if grid
                    .get(pos + dir + dir)
                    .map(Into::<char>::into)
                    .is_some_and(|b| b != '#')
                {
                    let &other = tiles.get(&(pos + dir + dir)).unwrap();
                    if other - current - 2 >= THRESHOLD {
                        count += 1;
                    }
                }
            } else {
                progress = dir;
            }
        }
        pos += progress;
        backwards = progress.clockwise().clockwise();
    }

    count.into()
}

pub fn part2(input: &str) -> Answer {
    let grid = Grid::new(input);
    let mut tiles = HashMap::default();
    let mut pos = grid
        .iter()
        .find_map(|(i, b)| (b == 'S').then_some(i))
        .unwrap();
    let mut backwards = BACKWARDS;

    for dist in 0.. {
        tiles.insert(pos, dist);
        if unsafe { grid.get(pos).unwrap_unchecked() } == 'E' {
            break;
        }
        for &dir in [Dir::North, Dir::East, Dir::South, Dir::West]
            .iter()
            .filter(|&&dir| dir != backwards)
        {
            if unsafe { grid.get(pos + dir).unwrap_unchecked() } != '#' {
                pos += dir;
                backwards = dir.clockwise().clockwise();
                break;
            }
        }
    }

    tiles
        .par_iter()
        .flat_map(|(&(i_y, i_x), &a)| {
            tiles.par_iter().filter(move |&(&(j_y, j_x), &b)| {
                let dist = i_y.abs_diff(j_y) + i_x.abs_diff(j_x);
                a - b - dist as i64 >= THRESHOLD && dist <= 20
            })
        })
        .count()
        .into()
}
