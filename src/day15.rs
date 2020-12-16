use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    map: HashMap<usize, usize>,
    last_value: usize,
    current_turn: usize,
}

impl Game {
    fn new(seed: &Vec<usize>) -> Game {
        let mut map = HashMap::new();
        seed.iter().enumerate().for_each(|(pos, &val)| { map.insert(val, pos+1); });
        Game { map, last_value: seed[seed.len() - 1], current_turn: seed.len() }
    }
}

impl Iterator for Game {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_turn += 1;
        //print!("Turn {}: Seeing {} -> ", self.current_turn, self.last_value);
        let speak_value = match self.map.get(&self.last_value) {
            None => {
                //println!("Never seen before, therefore speaking 0");
                0
            }
            Some(last_pos) => {
                //println!("Seen before in turn {}, therefore speaking {}-{}={}", last_pos, self.current_turn - 1, last_pos, self.current_turn - 1 - last_pos);
                self.current_turn - last_pos - 1
            }
        };
        self.map.insert(self.last_value, self.current_turn-1);
        self.last_value = speak_value;
        Some(self.last_value)
    }
}

#[aoc_generator(day15)]
pub fn generate_starting_numbers(input: &str) -> Vec<usize> {
    input.split(",").map(|part| part.parse().unwrap()).collect()
}

#[aoc(day15, part1)]
pub fn get_2020th_number(input: &Vec<usize>) -> String {
    let game = Game::new(input);
    format!("{:?}", game.into_iter().skip(2019 - input.len()).next().unwrap())
}

#[aoc(day15, part2)]
pub fn get_30000000th_number(input: &Vec<usize>) -> String {
    let game = Game::new(input);
    format!("{:?}", game.into_iter().skip(30_000_000 - 1 - input.len()).next().unwrap())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works_0() {
        assert_eq!(super::Game::new(&vec!(0, 3, 6)).into_iter().skip(6).next(), Some(0));
    }

    #[test]
    fn it_works_1() {
        assert_eq!(super::get_2020th_number(&vec!(1, 3, 2)), "1");
        assert_eq!(super::get_2020th_number(&vec!(2, 1, 3)), "10");
        assert_eq!(super::get_2020th_number(&vec!(1, 2, 3)), "27");
        assert_eq!(super::get_2020th_number(&vec!(2, 3, 1)), "78");
        assert_eq!(super::get_2020th_number(&vec!(3, 2, 1)), "438");
        assert_eq!(super::get_2020th_number(&vec!(3, 1, 2)), "1836");
    }
}