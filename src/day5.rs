use crate::utils::*;
use std::cmp::Ordering;

const _SAMPLE: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

fn parse_num(input: &str) -> u64 {
    let bytes = input.as_bytes();
    (bytes[0] - b'0') as u64 * 10 + (bytes[1] - b'0') as u64
}

fn get_deps<'a>(lines: impl Iterator<Item = &'a str>) -> HashMap<u64, Vec<u64>> {
    let mut deps = HashMap::<_, Vec<_>>::default();
    for line in lines.take_while(|line| !line.is_empty()) {
        let (first, second) = line.split_once('|').unwrap();
        let (first, second) = (parse_num(first), parse_num(second));
        deps.entry(second).or_default().push(first);
    }
    deps
}

pub fn part1(input: &str) -> Answer {
    let mut lines = input.lines();
    let deps = get_deps(&mut lines);
    let mut banned = HashSet::default();
    let mut sum = 0;
    'lines: for line in lines {
        banned.clear();
        let mut count = 0;
        for num in line.split(',').map(parse_num) {
            count += 1;
            if banned.contains(&num) {
                continue 'lines;
            }
            if let Some(ban) = deps.get(&num) {
                banned.extend(ban.iter().copied());
            }
        }
        sum += line.split(',').map(parse_num).nth(count / 2).unwrap();
    }
    sum.into()
}

pub fn part2(input: &str) -> Answer {
    let mut lines = input.lines();
    let deps = get_deps(&mut lines);
    let mut banned = HashSet::default();
    let mut nums = vec![];
    let mut sum = 0;
    'lines: for line in lines {
        banned.clear();
        for num in line.split(',').map(parse_num) {
            if banned.contains(&num) {
                nums.clear();
                nums.extend(line.split(',').map(parse_num));
                let mid = nums.len() / 2;
                sum += *nums
                    .select_nth_unstable_by(mid, |lhs, rhs| {
                        if deps.get(lhs).is_some_and(|lhs_deps| lhs_deps.contains(rhs)) {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }
                    })
                    .1;
                continue 'lines;
            }
            if let Some(ban) = deps.get(&num) {
                banned.extend(ban.iter().copied());
            }
        }
    }
    sum.into()
}
