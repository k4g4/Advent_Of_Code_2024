use crate::utils::*;
use std::{hint, mem};

const _SAMPLE: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

pub fn part1(input: &str) -> Answer {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut map = GridOwned::new(grid);
    let mut robot = map
        .iter()
        .find_map(|(i, b)| (b == '@').then_some(i))
        .unwrap();
    map.set(robot, '.');

    let dirs = moves
        .lines()
        .flat_map(str::bytes)
        .map(|b| unsafe { mem::transmute(b) });

    for dir in dirs {
        match unsafe { map.get(robot + dir).unwrap_unchecked() }.into() {
            '#' => {}
            '.' => {
                robot += dir;
            }
            'O' => {
                let mut r#box = robot + dir;
                loop {
                    match unsafe { map.get(r#box + dir).unwrap_unchecked() }.into() {
                        'O' => {
                            r#box += dir;
                        }
                        '#' => break,
                        '.' => {
                            robot += dir;
                            map.set(robot, '.');
                            map.set(r#box + dir, 'O');
                            break;
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unsafe { hint::unreachable_unchecked() },
        }
    }

    map.iter()
        .flat_map(|((i, j), b)| (b == 'O').then(|| 100 * i + j))
        .sum::<isize>()
        .into()
}

pub fn part2(input: &str) -> Answer {
    fn clearable(map: &mut GridOwned, cached: &mut HashSet<Index>, space: Index, dir: Dir) -> bool {
        match unsafe { map.get(space).unwrap_unchecked() }.into() {
            '.' => true,
            '#' => false,
            '[' if cached.contains(&space) => true,
            '[' => movable(map, cached, space, dir),
            ']' => movable(map, cached, space + Dir::West, dir),
            _ => unsafe { hint::unreachable_unchecked() },
        }
    }

    fn movable(map: &mut GridOwned, cached: &mut HashSet<Index>, lbox: Index, dir: Dir) -> bool {
        let rbox = lbox + Dir::East;
        let movable = match dir {
            Dir::North | Dir::South => {
                clearable(map, cached, lbox + dir, dir) && clearable(map, cached, rbox + dir, dir)
            }
            Dir::East => clearable(map, cached, rbox + dir, dir),
            Dir::West => clearable(map, cached, lbox + dir, dir),
        };
        if movable {
            cached.insert(lbox);
        }
        movable
    }

    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut map = {
        let bytes = grid
            .bytes()
            .flat_map(|b| match b {
                b'#' => b"##" as &[_],
                b'.' => b"..",
                b'O' => b"[]",
                b'@' => b"@.",
                b'\n' => b"\n",
                _ => unreachable!(),
            })
            .copied()
            .collect();
        let string = unsafe { String::from_utf8_unchecked(bytes) };
        GridOwned::new(&string)
    };
    let mut robot = map
        .iter()
        .find_map(|(i, b)| (b == '@').then_some(i))
        .unwrap();
    map.set(robot, '.');

    let dirs = moves
        .lines()
        .flat_map(str::bytes)
        .map(|b| unsafe { mem::transmute(b) });

    let mut cached = HashSet::default();
    for dir in dirs {
        match unsafe { map.get(robot + dir).unwrap_unchecked() }.into() {
            '#' => {}
            '.' => {
                robot += dir;
            }
            '[' | ']' => {
                cached.clear();
                if clearable(&mut map, &mut cached, robot + dir, dir) {
                    for &lbox in &cached {
                        map.set(lbox, '.');
                        map.set(lbox + Dir::East, '.');
                    }
                    for &lbox in &cached {
                        map.set(lbox + dir, '[');
                        map.set(lbox + Dir::East + dir, ']');
                    }
                    robot += dir;
                }
            }
            _ => unsafe { hint::unreachable_unchecked() },
        }
    }

    map.iter()
        .flat_map(|((i, j), b)| (b == '[').then(|| 100 * i + j))
        .sum::<isize>()
        .into()
}
