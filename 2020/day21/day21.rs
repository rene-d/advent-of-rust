//! [Day 21: Allergen Assessment](https://adventofcode.com/2020/day/21)

use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug)]
struct Menu<'a> {
    ingr: FxHashSet<&'a str>,
    allerg: FxHashSet<&'a str>,
}

#[derive(Debug)]
struct Puzzle<'a> {
    menus: Vec<Menu<'a>>,
    allergens: FxHashMap<&'a str, FxHashSet<&'a str>>,
    ingredients: FxHashSet<&'a str>,
}

impl<'a> Puzzle<'a> {
    /// Initialize from the puzzle input.
    fn new(data: &'a str) -> Self {
        let mut menus = Vec::new();

        for line in data.lines() {
            let (ingr, allerg) = line.split_once(" (contains ").unwrap();
            let ingr = ingr.split_ascii_whitespace().collect();

            let allerg = allerg.strip_suffix(")").unwrap().split(", ").collect();

            let menu = Menu { ingr, allerg };

            menus.push(menu);
        }

        let mut allergens: FxHashMap<&str, FxHashSet<&str>> = FxHashMap::default();
        for menu in &menus {
            for allergen in &menu.allerg {
                let x = allergens
                    .entry(allergen)
                    .or_insert_with(|| menu.ingr.clone());
                *x = x.intersection(&menu.ingr).copied().collect();
            }
        }

        let mut ingredients: FxHashSet<&str> = FxHashSet::default();
        for menu in &menus {
            ingredients.extend(&menu.ingr);
        }

        Self {
            menus,
            allergens,
            ingredients,
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let mut no_allergens = self.ingredients.clone();

        for allerg in self.allergens.values() {
            no_allergens = no_allergens.difference(allerg).copied().collect();
        }

        self.menus
            .iter()
            .map(|Menu { ingr, allerg: _ }| no_allergens.intersection(ingr).count())
            .sum()
    }

    /// Solve part two.
    fn part2(&self) -> String {
        let mut dangerous = Vec::new();

        let mut allergens = self.allergens.clone();

        loop {
            if allergens.is_empty() {
                break;
            }

            for (&k, v) in &mut allergens {
                if v.len() == 1 {
                    v.drain().for_each(|ingr| dangerous.push((k, ingr)));
                    break;
                }
            }

            let last_dangerous = dangerous.last().unwrap();

            allergens.remove(last_dangerous.0);
            for v in allergens.values_mut() {
                v.remove(last_dangerous.1);
            }
        }

        dangerous.sort_unstable();
        dangerous
            .into_iter()
            .map(|(_, i)| i)
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn main() {
    let args = aoc::parse_args();
    let puzzle = Puzzle::new(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let data = aoc::load_input_data("test.txt");
        let puzzle = Puzzle::new(&data);
        assert_eq!(puzzle.part1(), 5);
    }

    #[test]
    fn part2() {
        let data = aoc::load_input_data("test.txt");
        let puzzle = Puzzle::new(&data);
        assert_eq!(puzzle.part2(), "mxmxvkd,sqjhc,fvjkl");
    }
}
