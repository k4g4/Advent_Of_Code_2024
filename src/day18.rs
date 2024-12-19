use crate::utils::*;
use std::mem;

const DIMS: Index = (71, 71);
const BOTTOM_RIGHT: Index = (DIMS.0 - 1, DIMS.1 - 1);
const STEPS: usize = 1024;

const _SAMPLE: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

fn bytes(input: &str) -> impl Iterator<Item = Index> + use<'_> {
    input.lines().map(|line| {
        let (x, line) = parse(line);
        let line = &line[1..];
        let (y, _) = parse(line);
        (y, x)
    })
}

pub fn part1(input: &str) -> Answer {
    let mut space = GridOwned::new_dims(DIMS);
    for byte in bytes(input).take(STEPS) {
        space.set(byte, '#');
    }
    let mut probes = HashSet::from_iter([(0, 0)]);
    let mut new_probes = HashSet::default();
    for step in 1.. {
        for neighbor in probes.drain().flat_map(dirs) {
            if space.get(neighbor).map(Into::into) == Some(' ') {
                new_probes.insert(neighbor);
                space.set(neighbor, '#');
            }
        }
        mem::swap(&mut probes, &mut new_probes);
        if probes.contains(&BOTTOM_RIGHT) {
            return Answer::Number(step);
        }
    }
    unreachable!()
}

pub fn part2(input: &str) -> Answer {
    let space = GridOwned::new_dims(DIMS);
    let (_, (col, row)) = bytes(input)
        .scan(space, |space, byte| {
            space.set(byte, '#');
            Some((space.clone(), byte))
        })
        .collect_vec()
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|(space, _)| {
            let mut space = space.clone();
            let (mut probes, mut new_probes) = (HashSet::from_iter([(0, 0)]), HashSet::default());
            while !probes.contains(&BOTTOM_RIGHT) {
                for neighbor in probes.drain().flat_map(dirs) {
                    if space.get(neighbor).map(Into::into) == Some(' ') {
                        new_probes.insert(neighbor);
                        space.set(neighbor, 'X');
                    }
                }
                mem::swap(&mut probes, &mut new_probes);
                if probes.is_empty() {
                    return true;
                }
            }
            false
        })
        .unwrap();
    Answer::String(format!("{row},{col}"))
}
