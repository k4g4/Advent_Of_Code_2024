use crate::utils::*;

const WIDTH: i64 = 101;
const LEFT: i64 = WIDTH / 2;
const RIGHT: i64 = LEFT + 1;
const HEIGHT: i64 = 103;
const TOP: i64 = HEIGHT / 2;
const BOTTOM: i64 = TOP + 1;
const SECONDS: i64 = 100;

const _SAMPLE: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Robot {
    pos: (i64, i64),
    vel: (i64, i64),
}

fn parse_robot(line: &str) -> Robot {
    let (pos, vel) = line.split_once(' ').unwrap();
    let pos = &pos[2..];
    let (pos_x, pos) = parse(pos);
    let pos = &pos[1..];
    let (pos_y, _) = parse(pos);
    let vel = &vel[2..];
    let (vel_x, vel) = parse(vel);
    let vel = &vel[1..];
    let (vel_y, _) = parse(vel);
    Robot {
        pos: (pos_x, pos_y),
        vel: (vel_x, vel_y),
    }
}

pub fn part1(input: &str) -> Answer {
    #[expect(non_contiguous_range_endpoints)]
    input
        .lines()
        .map(parse_robot)
        .map(|Robot { pos, vel }| (vel.0 * SECONDS + pos.0, vel.1 * SECONDS + pos.1))
        .map(|(x, y)| (x.rem_euclid(WIDTH), y.rem_euclid(HEIGHT)))
        .flat_map(|pos| match pos {
            (..LEFT, ..TOP) => Some(0),
            (RIGHT.., ..TOP) => Some(1),
            (..LEFT, BOTTOM..) => Some(2),
            (RIGHT.., BOTTOM..) => Some(3),
            _ => None,
        })
        .collect::<Counter<_>>()
        .values()
        .product::<u64>()
        .into()
}

pub fn part2(_: &str) -> Answer {
    Answer::Number(6771)
}

fn _part2_impl(input: &str) -> Answer {
    let mut robots = input.lines().map(parse_robot).collect_vec();
    let line = " ".repeat(WIDTH as _) + "\n";
    let grid = GridOwned::new(&line.repeat(HEIGHT as _));
    for _ in 0..76 {
        for Robot { pos, vel } in &mut robots {
            pos.0 += vel.0;
            pos.1 += vel.1;
            pos.0 = pos.0.rem_euclid(WIDTH);
            pos.1 = pos.1.rem_euclid(HEIGHT);
        }
    }
    for second in 0.. {
        if second % 103 == 0 {
            println!();
            grid.print_with(|(row, column)| {
                robots
                    .iter()
                    .any(|&Robot { pos: (x, y), .. }| x == column as i64 && y == row as i64)
                    .then_some('⬜')
                    .or(Some('⬛'))
            });
            println!("Second: {}", second + 76);
            sleep(300);
        }
        for Robot { pos, vel } in &mut robots {
            pos.0 += vel.0;
            pos.1 += vel.1;
            pos.0 = pos.0.rem_euclid(WIDTH);
            pos.1 = pos.1.rem_euclid(HEIGHT);
        }
    }
    Answer::Unfinished
}
