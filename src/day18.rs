use aoc_runner_derive::{aoc, aoc_generator};
use logos::Logos;

#[aoc_generator(day18)]
pub fn generate_term(input: &str) -> Vec<Vec<MathToken>> {
    input.lines().map(|line| MathToken::lexer(line).collect()).collect()
}

#[derive(Logos, Debug, PartialEq)]
pub enum MathToken {
    #[regex("[0-9]+", | lex | lex.slice().parse::< u64 > ().unwrap())]
    Number(u64),
    #[regex(" +", logos::skip)]
    Space,
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("+")]
    Add,
    #[token("*")]
    Mul,
    #[error]
    Error,
}

#[aoc(day18, part1)]
pub fn get_sum_of_terms(input: &Vec<Vec<MathToken>>) -> u64 {
    input.iter().map(|tokens| {
        let mut token_stream = tokens.iter();
        let mut stack: Vec<(u64, fn(u64, u64) -> u64)> = vec!();
        let mut l_val = 0u64;
        let mut op: fn(u64, u64) -> u64 = |x, y| x + y;
        while let Some(next_token) = token_stream.next() {
            match next_token {
                MathToken::Number(val) => { l_val = op(l_val, *val); }
                MathToken::OpenParen => {
                    stack.push((l_val, op));
                    l_val = 0;
                    op = |x, y| x + y;
                }
                MathToken::CloseParen => {
                    let (last_val, last_op) = stack.pop().unwrap();
                    l_val = last_op(last_val, l_val);
                    op = last_op;
                }
                MathToken::Add => { op = |x, y| x + y; }
                MathToken::Mul => { op = |x, y| x * y }
                MathToken::Space => {}
                MathToken::Error => {}
            }
        }
        l_val
    }).sum()
}

pub fn calc_adv(tokens: &[MathToken]) -> u64 {
    let mut non_paren_terms = vec!();
    let mut paren_start = 0;
    let mut paren_counter = 0;

    for (i, token) in tokens.iter().enumerate() {
        if paren_counter == 0 {
            match token {
                MathToken::Number(val) => {
                    non_paren_terms.push(MathToken::Number(*val));
                }
                MathToken::OpenParen => {
                    paren_counter += 1;
                    paren_start = i;
                }
                MathToken::CloseParen => {}
                MathToken::Space => {}
                MathToken::Add => { non_paren_terms.push(MathToken::Add); }
                MathToken::Mul => { non_paren_terms.push(MathToken::Mul); }
                MathToken::Error => { unreachable!() }
            }
        } else {
            if let MathToken::OpenParen = token {
                paren_counter += 1;
            } else if let MathToken::CloseParen = token {
                paren_counter -= 1;
                if paren_counter == 0 {
                    non_paren_terms.push(MathToken::Number(calc_adv(&tokens[(paren_start + 1)..i])))
                }
            }
        }
    }
    //println!("Concerning: {:?}", non_paren_terms);
    let mut summands = vec!();
    let mut factors = vec!();


    for i in (1..non_paren_terms.len()).step_by(2) {
        //println!("Inspecting: {:?} {:?}", non_paren_terms[i-1],non_paren_terms[i]);
        match non_paren_terms[i] {
            MathToken::Add => {
                if let MathToken::Number(val) = non_paren_terms[i - 1] {
                    summands.push(val);
                } else { unreachable!() }
            }
            MathToken::Mul => {
                if let MathToken::Number(val) = non_paren_terms[i - 1] {
                    if summands.len() > 0 {
                        summands.push(val);
                        factors.push(summands.iter().sum());
                        summands = vec!();
                    } else {
                        factors.push(val);
                    }
                } else { unreachable!() }
            }
            _ => { unreachable!() }
        }
        //println!("Sum:{:?}\nFac:{:?}", summands, factors);
    }

    if let MathToken::Number(val) = non_paren_terms[non_paren_terms.len() - 1] {
        if summands.len() > 0 {
            summands.push(val);
            factors.push(summands.iter().sum());
        } else {
            factors.push(val);
        }
    } else { unreachable!() }

    //println!("{:?}\n", factors);

    factors.iter().fold(1, |x, y| x * y)
}

#[aoc(day18, part2)]
pub fn get_sum_of_advterms(input: &Vec<Vec<MathToken>>) -> u64 {
    input.iter().map(|tokens| {
        calc_adv(tokens)
    }).sum()
}






















