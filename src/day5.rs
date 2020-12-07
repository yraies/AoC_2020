use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn generate_map(input: &str) -> Vec<(u8, u8)> {
    input.lines().map(|line| {
        let mut row: (u8, u8) = (0, 127);
        let mut seat: (u8, u8) = (0, 7);
        line.chars().for_each(|c| {
            match c {
                'F' => row.1 -= (row.1 - row.0) / 2 + 1,
                'B' => row.0 += (row.1 - row.0) / 2 + 1,
                'L' => seat.1 -= (seat.1 - seat.0) / 2 + 1,
                'R' => seat.0 += (seat.1 - seat.0) / 2 + 1,
                _ => (),
            }
        });
        (row.0, seat.0)
    }).collect()
}

#[aoc(day5, part1)]
pub fn highest_seat_id(map: &[(u8, u8)]) -> usize {
    map.iter().map(|(row, seat)| *row as usize * 8 as usize + *seat as usize).max().unwrap()
}

#[aoc(day5, part2)]
pub fn find_missing(map: &[(u8, u8)]) -> usize {
    let mut seats: Vec<usize> = map.iter().map(|(row, seat)| *row as usize * 8 as usize + *seat as usize).collect();
    seats.sort();
    println!("{:?}",seats);
    for index in 1..seats.len() {
        if seats[index] - seats[index-1] != 1 { return seats[index]-1; }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("F       {:?}", super::generate_map("F"));
        println!("FB      {:?}", super::generate_map("FB"));
        println!("FBF     {:?}", super::generate_map("FBF"));
        println!("FBFB    {:?}", super::generate_map("FBFB"));
        println!("FBFBB   {:?}", super::generate_map("FBFBB"));
        println!("FBFBBF  {:?}", super::generate_map("FBFBBF"));
        println!("FBFBBFF {:?}", super::generate_map("FBFBBF"));
        println!("FBFBBFF {:?}", super::generate_map("FBFBBF"));
        println!("R   {:?}", super::generate_map("FBFBBFR"));
        println!("RL  {:?}", super::generate_map("FBFBBFRL"));
        println!("RLR {:?}", super::generate_map("FBFBBFRLR"));
    }
}