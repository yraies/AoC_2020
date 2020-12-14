use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum Instruction {
    Mask(Mask),
    Mem(usize, u64),
}

#[derive(Debug, Clone)]
pub struct Mask {
    ones: u64,
    zeros: u64,
}

impl Mask {
    fn get_floating_iter(&self) -> Vec<u64> {
        let floating = self.ones ^ self.zeros;
        let mut vals = vec!(0u64);
        for i in 0..36 {
            if (floating >> i) & 1 == 1 {
                vals.push(2u64.pow(i))
            }
        }
        let len = vals.len();
        vals.iter()
            .combinations_with_replacement(len)
            //.inspect(|f| println!("{:?}",f))
            .map(|combs| {
                combs.iter().fold(0u64, |acc, &&v| (acc | v))
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct VM {
    memory: HashMap<usize, u64>,
    current_mask: Mask,
}

impl VM {
    fn apply_v1(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Mask(mask) => {
                //println!("Setting mask:\n1: {:#038b}\n0: {:#038b}", mask.ones, mask.zeros);
                self.current_mask = mask.clone();
            }
            Instruction::Mem(pos, val) => {
                let mut res = val | self.current_mask.ones;
                res = res & self.current_mask.zeros;
                //println!("Inserting {} at mem[{}]", res, pos);
                self.memory.insert(*pos, res);
            }
        }
    }
    fn apply_v2(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Mask(mask) => {
                //println!("Setting mask:\n1: {:#038b}\n0: {:#038b}", mask.ones, mask.zeros);
                self.current_mask = mask.clone();
            }
            Instruction::Mem(pos, val) => {
                let temp_pos = *pos as u64 | self.current_mask.ones;
                for i in (self.current_mask.get_floating_iter()) {
                    let p = temp_pos ^ i;
                    self.memory.insert(p as usize,*val);
                    //println!("Inserting {} at mem[{:#038b}]", val, p);
                }
            }
        }
    }
    fn sum_memory(&self) -> u64 {
        self.memory.iter().map(|(_, &val)| val).sum()
    }
}


#[aoc_generator(day14)]
pub fn generate_instructions(input: &str) -> Vec<Instruction> {
    let mask_regex = Regex::new(r"mask = ([01X]{36})").unwrap();
    let mem_regex = Regex::new(r"mem\[([0-9]+)\] = ([0-9]+)").unwrap();
    input.lines().map(|line| {
        match &line[0..3] {
            "mas" => {
                let captures = mask_regex.captures(line).unwrap();
                Instruction::Mask(
                    Mask {
                        ones: (2u64.pow(36) - 1) & u64::from_str_radix(&captures.get(1).unwrap().as_str().replace("X", "0"), 2).unwrap(),
                        zeros: (2u64.pow(36) - 1) &  u64::from_str_radix(&captures.get(1).unwrap().as_str().replace("X", "1"), 2).unwrap(),
                    }
                )
            }
            "mem" => {
                let captures = mem_regex.captures(line).unwrap();
                Instruction::Mem(
                    captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                    captures.get(2).unwrap().as_str().parse::<u64>().unwrap(),
                )
            }
            _ => unreachable!(),
        }
    }).collect()
}

#[aoc(day14, part1)]
pub fn get_memory_sum_v1(input: &Vec<Instruction>) -> u64 {
    let mut vm = VM {
        memory: HashMap::new(),
        current_mask: Mask { ones: 0, zeros: 0 },
    };

    input.iter().for_each(|instr| vm.apply_v1(instr));

    vm.sum_memory()
}

#[aoc(day14, part2)]
pub fn get_memory_sum_v2(input: &Vec<Instruction>) -> u64 {
    let mut vm = VM {
        memory: HashMap::new(),
        current_mask: Mask { ones: 0, zeros: 0 },
    };

    input.iter().for_each(|instr| vm.apply_v2(instr));

    vm.sum_memory()
}
