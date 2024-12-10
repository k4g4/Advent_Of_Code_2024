use crate::utils::*;
use std::mem;

const _SAMPLE: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

pub fn part1(input: &str) -> Answer {
    type TrailMap = HashMap<Index, HashSet<usize>>;

    let grid = Grid::new(input);
    let mut trails: TrailMap = grid
        .indices()
        .filter(|&(_, tile)| tile == b'0')
        .enumerate()
        .map(|(trailhead, (index, _))| (index, HashSet::from_iter([trailhead])))
        .collect();
    let mut new_trails = TrailMap::default();

    for step in b'1'..=b'9' {
        for (index, trailheads) in trails.drain() {
            for new_index in dirs(index) {
                if grid.get(new_index) == Some(step) {
                    new_trails
                        .entry(new_index)
                        .or_default()
                        .extend(trailheads.iter().copied());
                }
            }
        }
        mem::swap(&mut trails, &mut new_trails);
    }

    trails.values().flatten().count().into()
}

pub fn part2(input: &str) -> Answer {
    type TrailMap = HashMap<Index, usize>;

    let grid = Grid::new(input);
    let mut trails: TrailMap = grid
        .indices()
        .filter(|&(_, tile)| tile == b'0')
        .map(|(index, _)| (index, 1))
        .collect();
    let mut new_trails = TrailMap::default();

    for step in b'1'..=b'9' {
        for (index, trailheads) in trails.drain() {
            for new_index in dirs(index) {
                if grid.get(new_index) == Some(step) {
                    *new_trails.entry(new_index).or_default() += trailheads;
                }
            }
        }
        mem::swap(&mut trails, &mut new_trails);
    }

    trails.values().sum::<usize>().into()
}
