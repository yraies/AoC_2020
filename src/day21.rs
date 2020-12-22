use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use derive_more::*;
use std::iter::FromIterator;
use itertools::Itertools;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, AsRef, Constructor)]
pub struct Ingredient(String);

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, AsRef, Constructor)]
pub struct Allergen(String);

#[aoc_generator(day21)]
pub fn generate_ingredient_allergen_lists(input: &str) -> Vec<(Vec<Ingredient>, Vec<Allergen>)> {
    let regex = Regex::new("([a-z ]+) \\(contains ([a-z, ]+)\\)").unwrap();

    input.lines().map(|line| {
        let caputres = regex.captures(line).unwrap();
        let ingredients = caputres.get(1).unwrap().as_str();
        let allergen_vec = caputres.get(2).unwrap()
            .as_str().split(", ")
            .map(|s| Allergen::new(s.to_string())).collect::<Vec<_>>();
        let ingredient_vec = ingredients.split_whitespace().map(|s| Ingredient(s.to_string())).collect::<Vec<_>>();
        (ingredient_vec, allergen_vec)
    }).collect()

    /*
    may_contain_map.into_iter().map(|(ingredient, allergen_sets)| {
        let mut allergens = None;
        for next_set in allergen_sets.into_iter() {
            allergens = match allergens {
                None => Some(next_set),
                Some(set) => {
                    let mut new_set = HashSet::new();
                    for allergen in set {
                        if next_set.contains(&allergen) {
                            new_set.insert(allergen);
                        }
                    }
                    Some(new_set)
                }
            };
        }
        (ingredient, allergens.unwrap_or(HashSet::new()).iter().map(|&x| x.to_string()).collect())
    }).collect()*/
}

#[aoc(day21, part1)]
pub fn get_allergen_free_ingredients(input: &Vec<(Vec<Ingredient>, Vec<Allergen>)>) -> String {
    let possible_causes = get_possible_causes(input);
    let all_ingredients = get_all_ingredients(input);

    let allergen_free_ingredients = all_ingredients.iter().filter(|&ing|
        !possible_causes.values().any(|causes| causes.contains(ing))
    ).collect::<Vec<_>>();

    format!("{:?}", input.iter()
        .map(|(ings, _)| ings.iter().filter(|ing| allergen_free_ingredients.contains(&ing)).count())
        .sum::<usize>()
    )
}

#[aoc(day21, part2)]
pub fn get_cdil(input: &Vec<(Vec<Ingredient>, Vec<Allergen>)>) -> String {
    let mut possible_causes = get_possible_causes(input);
    let all_ingredients = get_all_ingredients(input);

    let allergen_free_ingredients = all_ingredients.iter().filter(|&ing|
        !possible_causes.values().any(|causes| causes.contains(ing))
    ).map(|&ing| ing).collect::<Vec<_>>();

    println!("{:?}", possible_causes);

    possible_causes.iter_mut().for_each(|(_, causes)| {
        for &allergen_free_ingredient in &allergen_free_ingredients {
            causes.remove(allergen_free_ingredient);
        }
    });

    let mut cdil = vec!();

    while possible_causes.len() > 0 {
        let deducible_allergen = *possible_causes.iter().find(|(_, causes)| {
            if causes.len() == 1 { true } else { false }
        }).unwrap().0;

        let cause = possible_causes.remove(deducible_allergen).map(|causes|
            (deducible_allergen,
             *causes.iter().find(|_| true).unwrap()
            )).unwrap();

        possible_causes.values_mut().for_each(|causes| { causes.remove(cause.1); });
        cdil.push(cause);
    }

    format!("{:?}", cdil.iter().sorted_by_key(|(allergen, _)| *allergen)
        .map(|(_, ing)| ing.0.as_str()).join(","))
}

fn get_all_ingredients(input: &Vec<(Vec<Ingredient>, Vec<Allergen>)>) -> HashSet<&Ingredient> {
    let mut all_ingredients = HashSet::new();
    input.iter().for_each(|(ings, _)| ings.iter().for_each(|ing| { all_ingredients.insert(ing); }));
    all_ingredients
}

fn get_possible_causes(input: &Vec<(Vec<Ingredient>, Vec<Allergen>)>) -> HashMap<&Allergen, HashSet<&Ingredient>> {
    let mut possible_causes: HashMap<&Allergen, Vec<&Ingredient>> = HashMap::new();

    input.iter().for_each(|(ingredients, allergens)| {
        allergens.iter().for_each(|allergen| {
            let prev_ingredients_opt = possible_causes.get(allergen);

            match prev_ingredients_opt {
                None => { possible_causes.insert(allergen, ingredients.iter().collect()); }
                Some(prev_causes) => {
                    let narrowed_causes = prev_causes.iter()
                        .map(|&ing| ing)
                        .filter(|&ing| ingredients.contains(ing))
                        .collect::<Vec<&Ingredient>>();
                    possible_causes.insert(allergen, narrowed_causes);
                }
            }
        })
    });

    possible_causes.into_iter()
        .map(|(allergen, ings)|
            (allergen, HashSet::from_iter(ings.into_iter()))
        ).collect()
}

/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works_0() {
        assert_eq!(&super::get_3d_cube_count(&super::generate_plane(".#.\n..#\n###")), "0");
    }
}
*/































