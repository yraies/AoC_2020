use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Rule {
    Char(char),
    Subrules(Vec<Vec<usize>>),
}

trait MessageValidation {
    fn _valid_message(&self,
                      rule_id: usize,
                      position: usize,
                      chars: &str,
                      dp_map: &mut HashMap<(usize, usize), Result<Vec<usize>, ()>>,
                      depth: usize);
    fn valid_message(&self, chars: &str) -> bool;
}

impl MessageValidation for Vec<Rule> {
    fn _valid_message(&self,
                      rule_id: usize,
                      position: usize,
                      chars: &str,
                      dp_map: &mut HashMap<(usize, usize), Result<Vec<usize>, ()>>,
                      depth: usize) {
        if dp_map.contains_key(&(rule_id, position)) {
            return;
        }
        if depth > 200 || position > chars.len() {
            //println!("too deep");
            if !dp_map.contains_key(&(rule_id, position)) {
                dp_map.insert((rule_id, position), Err(()));
            }
            return;
        }

        //println!("({},{})", rule_id, position);

        match &self[rule_id] {
            Rule::Char(c) => {
                if Some(*c) == chars.chars().nth(position) {
                    let entry = dp_map.entry((rule_id, position)).or_insert(Ok(vec!()));
                    match entry {
                        Ok(values) => { values.push(1); }
                        Err(_) => { *entry = Ok(vec!(1usize)); }
                    };
                } else {
                    if !dp_map.contains_key(&(rule_id, position)) {
                        dp_map.insert((rule_id, position), Err(()));
                    }
                }
            }
            Rule::Subrules(subrules) => {
                let results: Vec<usize> = subrules.iter().map(|subrule| {
                    let mut positions = vec!(0);
                    for &rule in subrule {
                        positions = positions.iter().map(|&pos| {
                            //println!("Testing {} at {}", rule, pos + position);
                            self._valid_message(rule, position + pos, chars, dp_map, depth + 1);
                            if let Some(Ok(offsets)) = dp_map.get(&(rule, position + pos)) {
                                offsets.iter()
                                    .cartesian_product(vec!(pos))
                                    .map(|(&a, b)| a + b)
                                    .collect()
                            } else {
                                vec!()
                            }
                        }).flatten()
                            .collect();
                    }
                    positions
                }).flatten().map(|val| val).sorted().dedup().collect();
                if results.len() == 0 {
                    dp_map.insert((rule_id, position), Err(()));
                } else {
                    dp_map.insert((rule_id, position), Ok(results));
                }
            }
        }
        //println!("({},{}) {:?}", rule_id, position, dp_map.get(&(rule_id, position)));
    }

    fn valid_message(&self, chars: &str) -> bool {
        //println!("\n## Validating {}:", chars);
        let mut map: HashMap<(usize, usize), Result<Vec<usize>, ()>> = HashMap::new();
        self._valid_message(0, 0, chars, &mut map, 0);
        let foobarstuff = map.get(&(0, 0)).unwrap().as_ref();
        let res = match foobarstuff {
            Ok(result) => { result.iter().any(|&length| length == chars.len()) }
            Err(_) => { false }
        };
        //println!("{:?}", res);
        res
    }
}


#[aoc_generator(day19)]
pub fn generate_rules(input: &str) -> (Vec<Rule>, Vec<String>) {
    let mut split = input.split("\n\n");
    let rules_str = split.next().unwrap();
    let messages_str = split.next().unwrap();

    let rules: Vec<(usize, Rule)> = rules_str.lines().map(|line| {
        let mut linesplit = line.split(":");
        let id = linesplit.next().unwrap().parse::<usize>().unwrap();
        let rule_str = linesplit.next().unwrap().trim();

        let rule = if rule_str.contains("\"") {
            Rule::Char(rule_str.trim()[1..].chars().next().unwrap())
        } else {
            let subrules = rule_str.split("|").map(|r| {
                r.trim().split(" ").map(|v| v.parse().unwrap()).collect()
            }).collect();
            Rule::Subrules(subrules)
        };

        (id, rule)
    }).sorted_by_key(|v| v.0).collect();

    let mut final_rules = vec!(Rule::Char('#'); rules.iter().max_by_key(|v| v.0).unwrap().0 + 1);
    for (pos, rule) in rules { final_rules[pos] = rule; }
    let messages = messages_str.lines().map(|s| s.to_string()).collect();

    (final_rules, messages)
}

#[aoc(day19, part1)]
pub fn get_matching_rules(input: &(Vec<Rule>, Vec<String>)) -> usize {
    input.1.iter().filter(|message|
        input.0.valid_message(&message)
    )//.inspect(|v| println!("{}", v))
    .count()
}

#[aoc(day19, part2)]
pub fn get_matching_rules_with_8_11_replacement(input: &(Vec<Rule>, Vec<String>)) -> usize {
    let mut set = input.clone();
    set.0[8] = Rule::Subrules(vec!(vec!(42), vec!(42, 8)));
    set.0[11] = Rule::Subrules(vec!(vec!(42, 31), vec!(42, 11, 31)));
    set.1.iter().filter(|message|
        set.0.valid_message(&message)
    )//.inspect(|v| println!("{}", v))
        .count()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works_0() {
        assert_eq!(&super::get_matching_rules(&super::generate_rules(r#"0: 1 2 | 1 | 2
1: "a"
2: 1 3 | 3 1
3: "b"

ab"#)), &1);
    }

    #[test]
    fn simple_loop_all_invalid() {
        assert_eq!(&super::get_matching_rules_with_8_11_replacement(&super::generate_rules(
            r#"0: 2 0 | 2 1 | 1
1: 2 3 | 3 2 | 0
2: "a"
3: "b"

a
b
aa
bb
aaa
abb
abaa
abba"#)), &0);
    }

    #[test]
    fn simple_loop_all_valid() {
        assert_eq!(&super::get_matching_rules_with_8_11_replacement(&super::generate_rules(
            r#"0: 2 0 | 2 1 | 1
1: 2 3 | 3 2 | 0
2: "a"
3: "b"

ba
ab
aab
aba
aaab"#)), &5);
    }
}