use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct XMASCypher {
    history: Vec<u64>,
    position: usize,
    length: usize,
}

impl XMASCypher {
    fn new(preamble: &[u64]) -> XMASCypher {
        XMASCypher {
            length: preamble.len(),
            history: Vec::from(preamble),
            position: 0,
        }
    }

    fn push(&mut self, next: u64) -> Result<(), ()> {
        //println!("Pushing {}", next);
        if let Some(_vals) = self.history.iter().find_map(|v1| self.history.iter().find(|v2| v1 + *v2 == next).map(|v2| (v1, v2))) {
            //println!("found : {:?}", _vals);
            self.history[self.position] = next;
            self.position += 1;
            if self.position == self.length {
                self.position = 0;
            }
            //println!("New Cypher State: {:?}", self);
            Ok(())
        } else {
            Err(())
        }
    }
}

#[aoc_generator(day9)]
pub fn generate_cypherpoints(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse::<u64>().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn find_invalid_number(input: &Vec<u64>) -> u64 {
    let preamble_length = 25;
    let preamble = &input[0..preamble_length];
    //println!("Setting preamble {:?}", preamble);
    let mut cypher = XMASCypher::new(preamble);

    for i in input[preamble_length..].iter() {
        if cypher.push(*i).is_err() {
            return *i;
        }
    }
    unreachable!()
}

#[aoc(day9, part2)]
pub fn find_weakness(input: &Vec<u64>) -> String {
    let invalid_number = find_invalid_number(input);
    let (mut l, mut r, mut acc) = (0, 0, 0);
    loop {
        if acc < invalid_number {
            acc += input[r];
            r += 1;
        } else {
            if acc == invalid_number {
                break;
            }
            acc -= input[l];
            l += 1;
        }
        //println!("Init {:?} -> {}", &input[l..r], acc);
    }
    let (min,max) = (input[l..r].iter().min().unwrap(),input[l..r].iter().max().unwrap());
    format!("l:{} r:{} min:{} max:{} result:{}",l, r, min, max, min + max)
}
