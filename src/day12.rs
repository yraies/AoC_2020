use aoc_runner_derive::{aoc, aoc_generator};
use self::NavInstruction::*;
use std::ops::Div;

#[derive(Debug)]
pub enum NavInstruction {
    North(usize),
    East(usize),
    South(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

pub struct Ship {
    position: (i64, i64),
    waypoint: (i64, i64),
    rotation: u8,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            position: (0, 0),
            waypoint: (10, 1),
            rotation: 0,
        }
    }

    fn forward(&mut self, dist: usize) {
        match self.rotation {
            0 => self.position.0 += dist as i64,
            1 => self.position.1 -= dist as i64,
            2 => self.position.0 -= dist as i64,
            3 => self.position.1 += dist as i64,
            _ => unreachable!(),
        }
    }

    fn apply_interp1(&mut self, instr: &NavInstruction) {
        match instr {
            North(dist) => { self.position.1 += *dist as i64 }
            East(dist) => { self.position.0 += *dist as i64 }
            South(dist) => { self.position.1 -= *dist as i64 }
            West(dist) => { self.position.0 -= *dist as i64 }
            Left(deg) => { self.rotation = (self.rotation - deg.div(90) as u8) % 4 }
            Right(deg) => { self.rotation = (self.rotation + deg.div(90) as u8) % 4 }
            Forward(dist) => { self.forward(*dist) }
        }
    }

    fn rotate_waypoint_right(&mut self, turns: u8) {
        let w = self.waypoint;
        match turns {
            0 => self.waypoint = (w.0, w.1),
            1 => self.waypoint = (w.1, -w.0),
            2 => self.waypoint = (-w.0, -w.1),
            3 => self.waypoint = (-w.1, w.0),
            _ => unreachable!(),
        }
    }

    fn rotate_waypoint_left(&mut self, turns: u8) {
        let w = self.waypoint;
        match turns {
            0 => self.waypoint = (w.0, w.1),
            1 => self.waypoint = (-w.1, w.0),
            2 => self.waypoint = (-w.0, -w.1),
            3 => self.waypoint = (w.1, -w.0),
            _ => unreachable!(),
        }
    }

    fn apply_interp2(&mut self, instr: &NavInstruction) {
        match instr {
            North(dist) => { self.waypoint.1 += *dist as i64 }
            East(dist) => { self.waypoint.0 += *dist as i64 }
            South(dist) => { self.waypoint.1 -= *dist as i64 }
            West(dist) => { self.waypoint.0 -= *dist as i64 }
            Left(deg) => { self.rotate_waypoint_left((deg.div(90) as u8) % 4) }
            Right(deg) => { self.rotate_waypoint_right((deg.div(90) as u8) % 4) }
            Forward(dist) => {
                self.position.0 += self.waypoint.0 * *dist as i64;
                self.position.1 += self.waypoint.1 * *dist as i64;
            }
        }
    }
}


#[aoc_generator(day12)]
pub fn generate_navigation_instructions(input: &str) -> Vec<NavInstruction> {
    input.lines().map(|line| {
        let instruction = &line[0..1];
        let param = line[1..].parse::<usize>().unwrap();
        match instruction {
            "N" => North(param),
            "E" => East(param),
            "S" => South(param),
            "W" => West(param),
            "L" => Left(param),
            "R" => Right(param),
            "F" => Forward(param),
            _ => unreachable!(),
        }
    }).collect()
}

#[aoc(day12, part1)]
pub fn get_manhattan_distance_travelled(input: &Vec<NavInstruction>) -> i64 {
    let mut ship = Ship::new();
    input.iter().for_each(|instr| {
        ship.apply_interp1(instr);
        //println!("{:?}", ship.position)
    });
    ship.position.0.abs() + ship.position.1.abs()
}

#[aoc(day12, part2)]
pub fn get_manhattan_distance_travelled_interp_2(input: &Vec<NavInstruction>) -> i64 {
    let mut ship = Ship::new();
    input.iter().for_each(|instr| {
        ship.apply_interp2(instr);
        //println!("{:?}\np{:?}\nw{:?}\n", instr,ship.position,ship.waypoint);
    });
    ship.position.0.abs() + ship.position.1.abs()
}
