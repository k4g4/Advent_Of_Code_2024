use crate::utils::*;
use std::{
    cmp::{Ordering, Reverse},
    collections::{binary_heap::PeekMut, BinaryHeap},
    mem,
};

type Cost = u64;

const TURN_COST: Cost = 1000;

const _SAMPLE: &str = "\
###############
#.......#....E#
#.#.###.#.###^#
#.....#.#...#^#
#.###.#####.#^#
#.#.#.......#^#
#.#.#####.###^#
#..>>>>>>>>v#^#
###^#.#####v#^#
#>>^#.....#v#^#
#^#.#.###.#v#^#
#^....#...#v#^#
#^###.#.#.#v#^#
#S..#.....#>>^#
###############";

pub fn part1(input: &str) -> Answer {
    #[derive(Copy, Clone, Default, Debug)]
    struct Route {
        pos: Index,
        dir: Dir,
        cost: Cost,
    }

    impl PartialEq for Route {
        fn eq(&self, other: &Self) -> bool {
            self.cost.eq(&other.cost)
        }
    }

    impl Eq for Route {}

    impl PartialOrd for Route {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Reverse(self.cost).partial_cmp(&Reverse(other.cost))
        }
    }

    impl Ord for Route {
        fn cmp(&self, other: &Self) -> Ordering {
            Reverse(self.cost).cmp(&Reverse(other.cost))
        }
    }

    unsafe fn paths(maze: &GridOwned, Route { pos, dir, cost }: Route) -> ArrayVec<[Route; 3]> {
        [
            (dir.counter_clockwise(), cost + TURN_COST + 1),
            (dir, cost + 1),
            (dir.clockwise(), cost + TURN_COST + 1),
        ]
        .into_iter()
        .filter_map(|(dir, cost)| {
            (unsafe { maze.get(pos + dir).unwrap_unchecked() } != '#').then(|| Route {
                pos: pos + dir,
                dir,
                cost,
            })
        })
        .collect()
    }

    let mut maze = GridOwned::new(input);
    let start = maze
        .iter()
        .find_map(|(i, b)| (b == 'S').then_some(i))
        .unwrap();
    let mut routes = BinaryHeap::new();
    routes.push(Route {
        pos: start,
        dir: Dir::East,
        cost: 0,
    });

    loop {
        let mut cheapest = unsafe { routes.peek_mut().unwrap_unchecked() };
        if unsafe { maze.get(cheapest.pos).unwrap_unchecked() } == 'E' {
            break cheapest.cost.into();
        }
        maze.set(cheapest.pos, '#');
        let mut paths = unsafe { paths(&maze, *cheapest) };
        if paths.is_empty() {
            PeekMut::pop(cheapest);
        } else {
            let first = paths.pop().unwrap();
            *cheapest = first;
            drop(cheapest);
            routes.extend(paths);
        }
    }
}

pub fn part2(input: &str) -> Answer {
    #[derive(Clone, Default, Debug)]
    struct Route {
        pos: Index,
        dir: Dir,
        cost: Cost,
        seen: HashSet<Index>,
    }

    impl PartialEq for Route {
        fn eq(&self, other: &Self) -> bool {
            self.cost.eq(&other.cost)
        }
    }

    impl Eq for Route {}

    impl PartialOrd for Route {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Reverse(self.cost).partial_cmp(&Reverse(other.cost))
        }
    }

    impl Ord for Route {
        fn cmp(&self, other: &Self) -> Ordering {
            Reverse(self.cost).cmp(&Reverse(other.cost))
        }
    }

    unsafe fn paths(maze: &Grid, route: Route) -> ArrayVec<[Route; 3]> {
        let dirs = [
            (route.dir, route.cost + 1),
            (route.dir.clockwise(), route.cost + TURN_COST + 1),
            (route.dir.counter_clockwise(), route.cost + TURN_COST + 1),
        ];
        let mut valid_dirs = dirs
            .into_iter()
            .filter(|&(dir, _)| {
                let pos = route.pos + dir;
                !route.seen.contains(&pos) && unsafe { maze.get(pos).unwrap_unchecked() != '#' }
            })
            .collect::<ArrayVec<[_; 3]>>();
        let mut paths = array_vec![];
        if valid_dirs.len() > 1 {
            paths.extend(valid_dirs.drain(1..).map(|(dir, cost)| {
                let pos = route.pos + dir;
                let mut seen = route.seen.clone();
                seen.insert(pos);
                Route {
                    pos,
                    dir,
                    cost,
                    seen,
                }
            }))
        }
        if let Some((dir, cost)) = valid_dirs.pop() {
            let pos = route.pos + dir;
            let mut seen = route.seen;
            seen.insert(pos);
            paths.push(Route {
                pos,
                dir,
                cost,
                seen,
            });
        }
        paths
    }

    let maze = Grid::new(input);
    let start = maze
        .iter()
        .find_map(|(i, b)| (b == 'S').then_some(i))
        .unwrap();
    let mut routes = BinaryHeap::new();
    routes.push(Route {
        pos: start,
        dir: Dir::East,
        cost: 0,
        seen: HashSet::from_iter([start]).into(),
    });
    let mut seen = HashMap::default();
    let mut finished = vec![];
    let mut cost = None;

    while !routes.is_empty() {
        let mut cheapest = unsafe { routes.peek_mut().unwrap_unchecked() };

        if cost.is_some_and(|cost| cost < cheapest.cost) {
            break;
        } else if unsafe { maze.get(cheapest.pos).unwrap_unchecked() } == 'E' {
            cost = Some(cheapest.cost);
            finished.push(PeekMut::pop(cheapest));
        } else if seen
            .get(&(cheapest.pos, cheapest.dir))
            .is_some_and(|&cost| cost < cheapest.cost)
        {
            PeekMut::pop(cheapest);
        } else {
            let mut paths = unsafe { paths(&maze, mem::take(&mut *cheapest)) };
            for &Route { pos, dir, cost, .. } in &paths {
                seen.entry((pos, dir))
                    .and_modify(|c: &mut Cost| *c = (*c).min(cost))
                    .or_insert(cost);
            }
            if let Some(first) = paths.pop() {
                *cheapest = first;
                drop(cheapest);
                routes.extend(paths);
            } else {
                PeekMut::pop(cheapest);
            }
        }
    }

    let cheapest_cost = finished.iter().map(|route| route.cost).min().unwrap();
    finished
        .iter()
        .filter(|route| route.cost == cheapest_cost)
        .flat_map(|route| &route.seen)
        .collect::<HashSet<_>>()
        .len()
        .into()
}
