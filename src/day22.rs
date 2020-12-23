use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{VecDeque, HashSet};
use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[aoc_generator(day22)]
pub fn generate_decks(input: &str) -> [VecDeque<u32>; 2] {
    input.split("\n\n").map(|block| {
        block.lines().skip(1).map(|line| line.parse::<u32>().unwrap()).collect()
    }).collect::<Vec<VecDeque<u32>>>().try_into().unwrap()
}

fn calculate_score(deck: &VecDeque<u32>) -> usize {
    deck.iter().rev().enumerate().map(|(pos, v)| (pos + 1) * *v as usize).sum()
}

fn play_combat(decks: &mut [VecDeque<u32>; 2]) -> usize {
    match (decks[0].pop_front(), decks[1].pop_front()) {
        (Some(card1), Some(card2)) => {
            if card1 > card2 {
                decks[0].push_back(card1);
                decks[0].push_back(card2);
                play_combat(decks)
            } else {
                decks[1].push_back(card2);
                decks[1].push_back(card1);
                play_combat(decks)
            }
        }
        (Some(card1), None) => {
            decks[0].push_front(card1);
            calculate_score(&decks[0])
        }
        (None, Some(card2)) => {
            decks[1].push_front(card2);
            calculate_score(&decks[1])
        }
        (None, None) => unreachable!(),
    }
}

/**
Requires non-empty decks
**/
fn p1_wins_rec_combat(decks: &mut [VecDeque<u32>; 2], depth: usize) -> bool {
    let mut p1_prev_cards: HashSet<u64> = HashSet::new();
    let mut p2_prev_cards: HashSet<u64> = HashSet::new();

    while decks[0].len() > 0 && decks[1].len() > 0 {
        // Check for Loop -- Not sure if this is valid, but .... ¯\_(ツ)_/¯
        let mut hasher = DefaultHasher::new();
        decks[0].iter().for_each(|v| v.hash(&mut hasher));
        let hash1 = hasher.finish();
        let mut hasher = DefaultHasher::new();
        decks[1].iter().for_each(|v| v.hash(&mut hasher));
        let hash2 = hasher.finish();
        if !p1_prev_cards.insert(hash1) || !p2_prev_cards.insert(hash2)  {
            return true;
        }

        // Draw Cards
        let (card1, card2) = (decks[0].pop_front().unwrap(), decks[1].pop_front().unwrap());
       // println!("P1 deck: {:?}\nP1 deck: {:?}", decks[0], decks[1]);
       // println!("P1 plays: {:?}\nP2 plays: {:?}", card1, card2);
        let p1_won = match (decks[0].len() >= card1 as usize, decks[1].len() >= card2 as usize) {
            (true, true) => {
                let p1 = decks[0].iter().cloned().take(card1 as usize).collect();
                let p2 = decks[1].iter().cloned().take(card2 as usize).collect();
                let foo = p1_wins_rec_combat(&mut [p1, p2], depth + 1);
                //println!("DONE! {:?}", depth);
                foo
            }
            (_, _) => {
                card1 > card2
            }
        };

        if p1_won {
        //    println!("P1 wins\n");
            decks[0].push_back(card1);
            decks[0].push_back(card2);
        } else {
        //    println!("P2 wins\n");
            decks[1].push_back(card2);
            decks[1].push_back(card1);
        }
    }

    //println!("END OF GAME {}", depth);

    if decks[0].len() == 0 {
        false
    } else {
        true
    }
}

fn play_rec_combat(mut decks: &mut [VecDeque<u32>; 2]) -> usize {
    if p1_wins_rec_combat(&mut decks, 1) {
        //println!("End Result: {:?}", &decks);
        calculate_score(&decks[0])
    } else {
        //println!("End Result: {:?}", &decks);
        calculate_score(&decks[1])
    }
}

#[aoc(day22, part1)]
pub fn get_combat_winners_score(input: &[VecDeque<u32>; 2]) -> String {
    let mut work_copy = input.clone();
    format!("{:?}", play_combat(&mut work_copy))
}

#[aoc(day22, part2)]
pub fn get_rec_combat_winners_score(input: &[VecDeque<u32>; 2]) -> String {
    let mut work_copy = input.clone();
    format!("{:?}", play_rec_combat(&mut work_copy))
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works_0() {
        assert_eq!(&super::play_rec_combat(&mut super::generate_decks("Player 1:
43
19

Player 2:
2
29
14")), &105);
    }

    #[test]
    fn get_result() {
        assert_eq!(&super::play_rec_combat(&mut super::generate_decks("Player 1:
18
19
16
11
47
38
6
27
9
22
15
42
3
4
21
41
14
8
23
30
40
13
35
46
50

Player 2:
39
1
29
20
45
43
12
2
37
33
49
32
10
26
36
17
34
44
25
28
24
5
48
31
7
")), &0);
    }
}
























