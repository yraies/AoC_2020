use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::Range;
use logos::{Logos, Lexer};
use itertools::Itertools;

fn lex_range(lex: &mut Lexer<FileToken>) -> Result<Range<u64>, String> {
    let val = lex.slice().to_string();
    let mut split = val.split("-");
    let start = split.next()
        .ok_or("Error parsing Range".to_string())?
        .parse::<u64>().map_err(|e| e.to_string())?;
    let end = split.next()
        .ok_or("Error parsing Range".to_string())?
        .parse::<u64>().map_err(|e| e.to_string())?;
    Ok(start..end + 1)
}

fn lex_ticket(lex: &mut Lexer<FileToken>) -> Result<Vec<u64>, String> {
    lex.slice().to_string()
        .split(",").map(|split| {
        split.parse::<u64>().map_err(|e| e.to_string())
    })
        .collect()
}

#[derive(Logos, Debug, PartialEq)]
enum FileToken {
    #[token("your ticket:", logos::skip)]
    MyTicket,
    #[token("nearby tickets:", logos::skip)]
    NearbyTickets,
    #[regex("(\n| | or )", logos::skip)]
    Space,
    #[regex("[A-Za-z ]+:", | lex | {lex.slice()[0..lex.slice().len() - 1].to_string()})]
    RuleHead(String),
    #[regex("[0-9]+-[0-9]+", lex_range)]
    Range(Range<u64>),
    #[regex("([0-9+],?)+", lex_ticket)]
    Ticket(Vec<u64>),
    #[error]
    Error,
}

#[derive(Debug, Clone)]
pub struct Rule {
    subrules: Vec<Range<u64>>,
    name: String,
}

impl Rule {
    fn valid(&self, num: &u64) -> bool {
        self.subrules.iter().any(|r| r.contains(num))
    }
}

pub type Ticket = Vec<u64>;

#[aoc_generator(day16)]
pub fn generate_tickets_and_rules(input: &str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let tokens: Vec<FileToken> = FileToken::lexer(input).collect();
    let mut tokens = tokens.into_iter();

    let mut current_rule = Rule { subrules: vec!(), name: "".to_string() };
    let mut rules = vec!();
    let mut my_ticket = None;
    let mut near_tickets = vec!();

    while let Some(token) = tokens.next() {
        match token {
            FileToken::RuleHead(name) => {
                rules.push(current_rule.clone());
                current_rule = Rule { name, subrules: vec!() };
            }
            FileToken::Range(range) => { current_rule.subrules.push(range); }
            FileToken::Ticket(t) => {
                if my_ticket.is_none() { my_ticket = Some(t); } else { near_tickets.push(t) }
            }
            FileToken::MyTicket => (),
            FileToken::NearbyTickets => (),
            FileToken::Space => (),
            FileToken::Error => ()
        }
    }
    rules.push(current_rule);
    rules.remove(0);

    (rules, my_ticket.unwrap(), near_tickets)
}

#[aoc(day16, part1)]
pub fn get_error_rate(input: &(Vec<Rule>, Ticket, Vec<Ticket>)) -> u64 {
    //println!("{:?}\n", input);
    let rules = input.0.clone();
    input.2.iter()
        .map(|t| {
            t.iter()
                .filter_map(|num| {
                    if rules.iter()
                        //.inspect(|r| println!("{:?} -> {}", num, r.valid(num)))
                        .any(|r| r.valid(num)) {
                        None
                    } else {
                        //println!("Adding {}", num);
                        Some(*num)
                    }
                }
                )
        })
        .flatten()
        .sum()
}

#[aoc(day16, part2)]
pub fn get_depature_multiplied(input: &(Vec<Rule>, Ticket, Vec<Ticket>)) -> u64 {
    //println!("{:?}\n", input);
    let rules = input.0.clone();
    let mut filtered_tickets: Vec<_> = input.2.iter()
        .filter(|t|
            t.iter().all(|num| rules.iter().any(|r| r.valid(num)))
        ).collect();
    let tick = input.1.clone();
    filtered_tickets.push(&tick);

    let mut analysed_rules: Vec<_> = rules.iter().enumerate()
        .map(|(idx, rule)| {
            let mut vals = vec!();
            for i in 0..filtered_tickets[0].len() {
                vals.push(
                    filtered_tickets.iter()
                        .filter(|t| !rule.valid(&t[i])).count()
                )
            }
            (idx, rule, vals.iter().filter(|&&v| v == 0).count(), vals)
        })
        .sorted_by_key(|v| v.2)
        /*.inspect(|(idx, rule, zeros, val)| {
            println!("{}. rule: {:20}  counts: {:?} zeros: {}",
                     idx, rule.name, val, zeros);
        })*/
        .map(|(_,rule,_,vals)| (rule,vals)).collect();

    let mut sorted_rules = vec!(None;analysed_rules.len());

    for i in 0..analysed_rules.len() {
        let zero_idx = analysed_rules[i].1.iter().position(|v| *v == 0).unwrap();
        sorted_rules[zero_idx] = Some(analysed_rules[i].0);
        for j in 0..analysed_rules.len(){
            analysed_rules[j].1[zero_idx] = 1;
        }
    }

    //println!("{:?}", sorted_rules);

    let mut mult = 1;

    for i in 0..sorted_rules.len() {
        if sorted_rules[i].unwrap().name.contains("departure") {
            mult *= input.1[i];
        }
    }

    mult
}





































