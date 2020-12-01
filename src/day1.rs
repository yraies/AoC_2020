use aoc_runner_derive::{aoc,aoc_generator};

#[aoc_generator(day1)]
pub fn generate_int_slice(input: &str) -> Vec<i32> {
    input.split("\n").map(|v| v.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_2020_sum_2pair(values: &[i32]) -> i32 {
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
pub fn solve_2020_sum_3pair_naive(values: &[i32]) -> i32 {
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
pub fn solve_2020_sum_3pair_cached(values: &[i32]) -> i32 {
    for i1 in 0..values.len() {
        for i2 in i1..values.len() {
            let temp_sum = values[i1] + values[i2];
            let temp_mul = values[i1] * values[i2];
            for i3 in i2..values.len() {
                if temp_sum  + values[i3] == 2020 {
                    return temp_mul * values[i3];
                }
            }
        }
    }
    return 0;
}