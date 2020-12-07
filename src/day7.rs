use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use regex::Regex;

#[aoc_generator(day7)]
pub fn generate_map(input: &str) -> (HashMap<String,usize>,Vec<Vec<(usize,usize)>>) {
    let bag_regex = Regex::new(r"([a-z]+ [a-z]+) bags contain").unwrap();
    let mut color_map :HashMap<String,usize> = HashMap::new();
    let mut colors = bag_regex.captures_iter(input)
        .map(|capture| capture.get(1).unwrap().as_str().to_string())
        .collect::<Vec<String>>();
    colors.sort();
    colors.dedup();
    colors.iter().enumerate()
        .for_each(|(idx, color)| {color_map.insert(color.to_owned(), idx);});

    let mut bag_capacities : Vec<Vec<(usize,usize)>> = vec!(Vec::with_capacity(4); color_map.len());
    let line_regex = Regex::new(r"([0-9]+) ([a-z]+ [a-z]+) bag[s]?[,\.]").unwrap();
    input.lines().for_each(|line| {
        let bag_color = bag_regex.captures(line).unwrap().get(1).unwrap().as_str();
        let bag_idx = *color_map.get(bag_color).unwrap();

        let subbags = line_regex.captures_iter(line)
            .map(|c|
                c.iter().skip(1).map(|sc|
                    sc.map(|m| m.as_str())
                ).collect::<Vec<Option<&str>>>()
            ).flatten()
            .map(|o| o.unwrap())
            .collect::<Vec<&str>>();

        for i in (0..subbags.len()).step_by(2){
            bag_capacities[bag_idx].push((*color_map.get(subbags[i+1]).unwrap(), subbags[i].parse::<usize>().unwrap()))
        }
    });
    (color_map,bag_capacities)
}

#[aoc(day7, part1)]
pub fn count_colors_which_fit_shiny_gold(input: &(HashMap<String,usize>,Vec<Vec<(usize,usize)>>)) -> usize {
    let (color_map,bag_caps) = input;
    let shiny_gold_idx = *color_map.get("shiny gold").unwrap();
    let mut reaches_sg : Vec<bool> = vec!(false; color_map.len());
    for _j in 0..color_map.len() {
        for i in 0..color_map.len() {
            let subbags = &bag_caps[i];
            if !reaches_sg[i]{
                reaches_sg[i] = subbags.iter().any(|(bag,_)| *bag == shiny_gold_idx || reaches_sg[*bag]);
            }
        }
    }
    reaches_sg.iter().filter(|b| **b).count()
}

#[aoc(day7, part2)]
pub fn count_bags_in_shiny_gold_bag(input: &(HashMap<String,usize>,Vec<Vec<(usize,usize)>>)) -> usize {
    fn count_bags_in(idx: usize, caps: &Vec<Vec<(usize,usize)>>) -> usize {
        let mut sum = 1;
        for (color, num) in &caps[idx] {
            sum += *num * count_bags_in(*color, caps);
        }
        sum
    }

    let (color_map,bag_caps) = input;
    let shiny_gold_idx = *color_map.get("shiny gold").unwrap();
    count_bags_in(shiny_gold_idx,bag_caps)-1
}
