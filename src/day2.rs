use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Password {
    c: char,
    v1: usize,
    v2: usize,
    p: String,
}

#[aoc_generator(day2)]
pub fn generate_policy_pairs(input: &str) -> Vec<Password> {
    input.split("\n").map(|v| {
        let splits: Vec<&str> = v.split_whitespace().collect();
        let bounds: Vec<usize> = splits[0].split("-").map(|s| s.parse::<usize>().unwrap()).collect();
        let c = splits[1].chars().next().unwrap();
        Password { c, v1: bounds[0], v2: bounds[1], p: splits[2].to_string() }
    }).collect()
}

#[aoc(day2, part1)]
pub fn num_of_valid_passwords_oldplace(values: &[Password]) -> usize {
    values.iter().filter(|pass| {
        let count = pass.p.chars().filter(|&c| char::eq(&c, &pass.c)).count();
        pass.v1 <= count && count <= pass.v2
    }).count()
}

#[aoc(day2, part2)]
pub fn num_of_valid_passwords_newplace(values: &[Password]) -> usize {
    values.iter().filter(|pass| {
        pass.p.chars()
            .enumerate()
            .filter(|(index, c)| {
                let i = index + 1;
                if pass.v1.eq(&i) || pass.v2.eq(&i) {
                    pass.c.eq(c)
                } else {
                    false
                }
            }).count() == 1
    }).count()
}