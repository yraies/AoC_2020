use aoc_runner_derive::aoc;

use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn count_any_answer_in_group(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|s| {
            let mut set = HashSet::new();
            for l in s.lines() {
                for c in l.chars() {
                    set.insert(c);
                }
            }
            set.len()
        })
        .sum::<usize>()
}

#[aoc(day6, part2)]
pub fn count_all_answer_in_group(input: &str) -> usize {
    let base = ('a'..='z').collect::<HashSet<char>>();
    input
        .split("\n\n")
        .map(|s| {
            let mut group = base.clone();
            for l in s.lines() {
                let mut individual = HashSet::new();
                for c in l.chars() {
                    individual.insert(c);
                }
                group = group.intersection(&individual).map(|c| *c).collect();
            }
            group.len()
        })
        .sum::<usize>()
}
