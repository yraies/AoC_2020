use aoc_runner_derive::{aoc, aoc_generator};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct VM {
    pub code: Rc<RefCell<Vec<Instruction>>>,
    pub pc: usize,
    pub acc: i64,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

impl VM {
    fn next(&self) -> VM {
        //println!("Next: {:?}", self);
        let mut clone = VM { code: self.code.clone(), pc: self.pc, acc: self.acc };
        match self.code.borrow()[self.pc] {
            Instruction::Acc(param) => {
                clone.acc += param;
                clone.pc += 1;
            }
            Instruction::Jmp(param) => {
                clone.pc = (param + self.pc as i64) as usize;
            }
            Instruction::Nop(_) => {
                clone.pc += 1;
            }
        }
        clone
    }
    fn code_len(&self) -> usize {
        self.code.borrow().len()
    }
}

#[aoc_generator(day8)]
pub fn generate_vm(input: &str) -> VM {
    let code = input.lines().map(|line| {
        let mut parts = line.splitn(2, " ");
        let instr = parts.next().unwrap();
        let param = parts.next().unwrap().parse::<i64>().unwrap();
        match instr {
            "acc" => Instruction::Acc(param),
            "jmp" => Instruction::Jmp(param),
            "nop" => Instruction::Nop(param),
            _ => unreachable!(),
        }
    }).collect::<Vec<Instruction>>();
    VM { code: Rc::new(RefCell::new(code)), pc: 0, acc: 0 }
}

pub fn terminates(input: &VM) -> Result<VM, (VM, usize)> {
    let mut vm = input.next();
    let length = vm.code_len();
    let mut visited = vec!(false; length);
    visited[0] = true;
    let mut last_pc = 0;
    while vm.pc != length && !visited[vm.pc] {
        visited[vm.pc] = true;
        last_pc = vm.pc;
        vm = vm.next();
    }
    if vm.pc == length {
        Ok(vm)
    } else {
        Err((vm, last_pc))
    }
}

#[aoc(day8, part1)]
pub fn get_acc_before_2nd_exec(input: &VM) -> i64 {
    terminates(input).expect_err("Expected non terminating VM").0.acc
}

#[aoc(day8, part2)]
pub fn fix_jmp_or_noop(input: &VM) -> i64 {
    for i in 0..input.code_len() {
        let vm = input.clone();
        {
            let mut instrs = vm.code.borrow_mut();
            match instrs[i] {
                Instruction::Nop(param) => instrs[i] = Instruction::Jmp(param),
                Instruction::Jmp(param) => instrs[i] = Instruction::Nop(param),
                Instruction::Acc(_) => continue,
            }
            //println!("changed {} to {:?}", i, instrs[i])
        }
        let term = terminates(&vm);
        match term {
            Ok(ret_vm) => {
                //println!("Terminated by changing line {}", i);
                return ret_vm.acc;
            },
            Err((_,_)) => {
                //println!("Did not terminate with acc {} and pc {} caused by pc {}", ret_vm.acc, ret_vm.pc, last_pc);
                {
                    let mut instrs = vm.code.borrow_mut();
                    match instrs[i] {
                        Instruction::Nop(param) => instrs[i] = Instruction::Jmp(param),
                        Instruction::Jmp(param) => instrs[i] = Instruction::Nop(param),
                        Instruction::Acc(_) => continue,
                    }
                    //println!("changed {} to {:?}", i, instrs[i])
                }
            },
        }
    }
    println!("unreachable!");
    unreachable!()
}