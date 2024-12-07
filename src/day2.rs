use crate::utils::*;

enum Dir {
    Inc,
    Dec,
}

fn valid_report(report: &[u32]) -> bool {
    report
        .iter()
        .copied()
        .try_fold((None, None), |(prev, dir), level| {
            if let Some(prev) = prev {
                if (1..=3).contains(&level.abs_diff(prev)) {
                    if let Some(dir) = dir {
                        match dir {
                            Dir::Inc if level > prev => Ok((Some(level), Some(Dir::Inc))),
                            Dir::Dec if level < prev => Ok((Some(level), Some(Dir::Dec))),
                            _ => Err(()),
                        }
                    } else {
                        Ok((
                            Some(level),
                            Some(if level > prev { Dir::Inc } else { Dir::Dec }),
                        ))
                    }
                } else {
                    Err(())
                }
            } else {
                Ok((Some(level), None))
            }
        })
        .is_ok()
}

pub fn part1(input: &str) -> Answer {
    input
        .lines()
        .filter(|levels| {
            let report: ArrayVec<[u32; 8]> = levels
                .split_ascii_whitespace()
                .map(|level| parse::<u32>(level).0)
                .collect();
            valid_report(&report)
        })
        .count()
        .into()
}

pub fn part2(input: &str) -> Answer {
    input
        .lines()
        .filter(|levels| {
            let mut report: ArrayVec<[u32; 8]> = levels
                .split_ascii_whitespace()
                .map(|level| parse::<u32>(level).0)
                .collect();
            valid_report(&report)
                || (0..report.len()).any(|i| {
                    let removed = report.remove(i);
                    let valid = valid_report(&report);
                    report.insert(i, removed);
                    valid
                })
        })
        .count()
        .into()
}
