use crate::utils::*;

const ROCK: char = '#';
const _SAMPLE: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

pub fn part1(input: &str) -> Answer {
    let grid = Grid::new(input);
    let mut indices = HashSet::default();
    let mut pos = grid
        .iter()
        .find_map(|(pos, tile)| (tile == Dir::North as u8).then_some(pos))
        .unwrap();
    let mut dir = Dir::North;

    indices.insert(pos);

    while let Some(tile) = grid.get(pos + dir) {
        if tile == ROCK {
            dir = dir.clockwise();
        } else {
            pos += dir;
            indices.insert(pos);
        }
    }
    indices.len().into()
}

pub fn part2(input: &str) -> Answer {
    let grid = Grid::new(input);
    let mut pos = grid
        .iter()
        .find_map(|(pos, tile)| (tile == Dir::North as u8).then_some(pos))
        .unwrap();
    let start = pos;
    let mut dir = Dir::North;
    let mut indices = HashSet::default();
    let mut obstacle_indices = HashSet::default();
    let mut obstacles = HashSet::default();

    while let Some(tile) = grid.get(pos + dir) {
        if tile == ROCK {
            dir = dir.clockwise();
        } else {
            let obstacle = pos + dir;
            if !indices.contains(&obstacle) && !obstacles.contains(&obstacle) && obstacle != start {
                let (mut pos, mut dir) = (pos, dir.clockwise());

                obstacle_indices.clear();
                while let Some(tile) = grid.get(pos + dir) {
                    if tile == ROCK || pos + dir == obstacle {
                        dir = dir.clockwise();
                    } else if obstacle_indices.contains(&(pos + dir, dir)) {
                        obstacles.insert(obstacle);
                        break;
                    } else {
                        pos += dir;
                        obstacle_indices.insert((pos, dir));
                    }
                }
            }

            pos += dir;
            indices.insert(pos);
        }
    }

    obstacles.len().into()
}

pub fn part2_attempt1(input: &str) -> Answer {
    let grid = Grid::new(input);
    let mut indices = HashSet::default();
    let mut pos = grid
        .iter()
        .find_map(|(pos, tile)| (tile == Dir::North as u8).then_some(pos))
        .unwrap();
    let mut dir = Dir::North;

    fn backwards(mut pos: Index, dir: Dir, grid: &Grid, indices: &mut HashSet<(Index, Dir)>) {
        while grid.get(pos).is_some_and(|tile| tile != ROCK) && indices.insert((pos, dir)) {
            pos -= dir;
            if grid.get(pos + dir.counter_clockwise()).map(char::from) == Some(ROCK) {
                backwards(pos, dir.counter_clockwise(), grid, indices);
            }
        }
    }

    backwards(pos, dir, &grid, &mut indices);

    let mut obstacles = 0;
    while let Some(tile) = grid.get(pos + dir) {
        if tile == ROCK {
            dir = dir.clockwise();
            println!("{}", indices.len());
            backwards(pos, dir, &grid, &mut indices);
            grid.print_with(|index| {
                [Dir::North, Dir::South, Dir::East, Dir::West]
                    .iter()
                    .find(|&&dir| indices.contains(&(index, dir)))
                    .map(|&dir| dir as _)
            });
            sleep(100);
        } else {
            pos += dir;
            indices.insert((pos, dir));
            if indices.contains(&(pos, dir.clockwise())) {
                obstacles += 1;
            }
        }
    }
    obstacles.into()
}
