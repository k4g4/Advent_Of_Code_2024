use crate::utils::*;
use std::{mem, str};

const _SAMPLE: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

struct Computer {
    name: [u8; 2],
    conns: HashSet<usize>,
}

fn build_graph(input: &str) -> Vec<Computer> {
    let names: HashSet<_> = input
        .lines()
        .flat_map(|line| line.split('-'))
        .map(str::as_bytes)
        .map(|name| [name[0], name[1]])
        .collect();
    let mut graph = names
        .into_iter()
        .map(|name| Computer {
            name,
            conns: Default::default(),
        })
        .collect_vec();
    graph.sort_by_key(|computer| computer.name);

    for line in input.lines() {
        let (first, second) = line.split_once('-').unwrap();
        let first_i = graph
            .binary_search_by_key(&first.as_bytes(), |computer| &computer.name)
            .unwrap();
        let second_i = graph
            .binary_search_by_key(&second.as_bytes(), |computer| &computer.name)
            .unwrap();
        graph[first_i].conns.insert(second_i);
        graph[second_i].conns.insert(first_i);
    }
    graph
}

pub fn part1(input: &str) -> Answer {
    let graph = build_graph(input);

    let (mut probes, mut new_probes) = (vec![0], vec![]);
    let mut seen = HashSet::from_iter([0]);
    let mut triangles = HashSet::default();
    while !probes.is_empty() {
        for i in probes.drain(..) {
            let first = &graph[i];
            for &j in &first.conns {
                let second = &graph[j];
                for &k in first.conns.intersection(&second.conns) {
                    let third = &graph[k];
                    let mut names = [first.name, second.name, third.name];
                    if names.iter().any(|name| name[0] == b't') {
                        names.sort();
                        triangles.insert(names);
                    }
                }
                if seen.insert(j) {
                    new_probes.push(j);
                }
            }
        }
        mem::swap(&mut probes, &mut new_probes);
    }

    triangles.len().into()
}

pub fn part2(input: &str) -> Answer {
    fn bron_kerbosch(
        graph: &[Computer],
        r: HashSet<usize>,
        mut p: HashSet<usize>,
        mut x: HashSet<usize>,
    ) -> Option<String> {
        if p.is_empty() && x.is_empty() && r.len() > 12 {
            r.iter()
                .map(|&i| str::from_utf8(&graph[i].name).unwrap())
                .sorted()
                .join(",")
                .into()
        } else {
            for v in p.clone() {
                let mut r = r.clone();
                r.insert(v);
                if let Some(answer) =
                    bron_kerbosch(graph, r, &p & &graph[v].conns, &x & &graph[v].conns)
                {
                    return Some(answer);
                }
                p.remove(&v);
                x.insert(v);
            }
            None
        }
    }

    let graph = build_graph(input);

    bron_kerbosch(
        &graph,
        Default::default(),
        (0..graph.len()).collect(),
        Default::default(),
    )
    .unwrap()
    .into()
}

pub fn part2_attempt1(input: &str) -> Answer {
    fn largest_from(graph: &[Computer], i: usize, seen: &HashSet<usize>) -> usize {
        let computer = &graph[i];
        if seen.difference(&computer.conns).next().is_none() {
            let mut seen = seen.clone();
            seen.insert(i);
            let mut max = seen.len();
            for &j in computer.conns.difference(&seen) {
                max = max.max(largest_from(graph, j, &seen));
            }
            max
        } else {
            if seen.len() > 14 {
                println!(
                    "{:?}",
                    seen.iter()
                        .map(|&i| graph[i].name)
                        .map(|name| std::str::from_utf8(&name).unwrap().to_owned())
                        .collect_vec()
                );
            }
            seen.len()
        }
    }

    let graph = build_graph(input);
    let empty = Default::default();

    (0..graph.len())
        .into_par_iter()
        .map(|i| largest_from(&graph, i, &empty))
        .max()
        .unwrap()
        .into()
}
