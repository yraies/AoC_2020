use aoc_runner_derive::{aoc,aoc_generator};

type IntType = i32;

#[aoc_generator(day1)]
pub fn generate_int_slice(input: &str) -> Vec<IntType> {
    input.split("\n").map(|v| v.parse::<IntType>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_2020_sum_2pair(values: &[IntType]) -> IntType {
    for i1 in 0..values.len() {
        for i2 in i1..values.len() {
            if values[i1] + values[i2] == 2020 {
                return values[i1] * values[i2];
            }
        }
    }
    return 0;
}

#[aoc(day1, part2, naive)]
pub fn solve_2020_sum_3pair_naive(values: &[IntType]) -> IntType {
    for i1 in 0..values.len() {
        for i2 in i1..values.len() {
            for i3 in i2..values.len() {
                if values[i1] + values[i2]  + values[i3] == 2020 {
                    return values[i1] * values[i2] * values[i3];
                }
            }
        }
    }
    return 0;
}

#[aoc(day1, part2, cached)]
pub fn solve_2020_sum_3pair_cached(values: &[IntType]) -> IntType {
    for i1 in 0..values.len() {
        for i2 in i1+1..values.len() {
            let temp_sum = values[i1] + values[i2];
            for i3 in i2+1..values.len() {
                if temp_sum  + values[i3] == 2020 {
                    return values[i1] * values[i2] * values[i3];
                }
            }
        }
    }
    return 0;
}