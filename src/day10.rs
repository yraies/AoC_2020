use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
pub fn get_adapter_list(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect()
}

#[aoc(day10, part1)]
pub fn get_diff_distribution(input: &Vec<u64>) -> String {
    let mut adapters = input.clone();
    adapters.sort();
    let diffs = adapters
        .iter()
        .fold([0u64, 0u64, 0u64, 1u64], |mut acc, next| {
            let diff = next - acc[0];
            if diff > 3 {
                unreachable!()
            }
            acc[diff as usize] += 1;
            acc[0] = *next;
            acc
        });
    format!("{:?} {}", diffs, diffs[1] * diffs[3])
}

fn trib(i: usize) -> u64 {
    if i == 1 {
        return 1;
    } else if i == 2 {
        return 1;
    } else if i == 3 {
        return 2;
    } else {
        return trib(i - 1) + trib(i - 2) + trib(i - 3);
    }
}

#[aoc(day10, part2)]
pub fn get_arrangement_count(input: &Vec<u64>) -> u64 {
    let mut adapters = input.clone();
    adapters.push(*adapters.iter().max().unwrap());
    adapters.sort();

    let mut acc = vec![vec![0]];
    for next in adapters {
        if next == *acc.last().unwrap().last().unwrap() + 1 {
            acc.last_mut().unwrap().push(next);
        } else {
            acc.push(vec![next]);
        }
    }
    println!("{:?}", acc);
    let mut result = 1;
    for vec in acc {
        result *= trib(vec.len());
    }
    result
}
