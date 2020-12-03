use aoc_runner_derive::{aoc, aoc_generator};

pub struct Policy {
    c: char,
    v1: usize,
    v2: usize,
}

#[aoc_generator(day2)]
pub fn generate_policy_pairs(input: &str) -> Vec<(Policy, String)> {
    input.split("\n").map(|v| {
        let splits: Vec<&str> = v.split_whitespace().collect();
        let bounds: Vec<usize> = splits[0].split("-").map(|s| s.parse::<usize>().unwrap()).collect();
        let c = splits[1].chars().next().unwrap();
        let policy = Policy { c, v1: bounds[0], v2: bounds[1] };
        (policy, splits[2].to_string())
    }).collect()
}

#[aoc(day2, part1)]
pub fn num_of_valid_passwords_oldplace(values: &[(Policy, String)]) -> usize {
    values.iter().filter(|(policy, password)| {
        let count = password.chars().filter(|&c| char::eq(&c, &policy.c)).count();
        policy.v1 <= count && count <= policy.v2
    }).count()
}

#[aoc(day2, part2)]
pub fn num_of_valid_passwords_newplace(values: &[(Policy, String)]) -> usize {
    values.iter().filter(|(policy, password)| {

        password.chars()
            .enumerate()
            .filter(|(index, c)| {
                let i = index +1;
                if policy.v1.eq(&i) || policy.v2.eq(&i) {
                    policy.c.eq(c)
                } else {
                    false
                }
            }).count() == 1

    }).count()
}