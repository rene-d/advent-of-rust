//! [Day 7: Handy Haversacks](https://adventofcode.com/2020/day/7)

use regex::Regex;
use std::collections::HashMap;

struct Puzzle {
    // data: String,
    bags: HashMap<String, HashMap<String, u32>>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            bags: HashMap::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        // self.data = data;

        let pat = Regex::new(r"^\s?(\d+) (.+) bags?$").unwrap();

        for line in data.lines() {
            let (bag, contain) = line.split_once(" bags contain ").unwrap();

            let contain = contain.strip_suffix('.').unwrap();

            let mut contained = HashMap::new();

            for s in contain.split(',') {
                if s == "no other bags" {
                    continue;
                }

                let caps = pat.captures(s).unwrap();

                let n: u32 = caps[1].parse().unwrap();
                let sub_bag = &caps[2];

                contained.insert(sub_bag.to_string(), n);
            }

            self.bags.insert(bag.to_string(), contained);
        }
    }

    fn contains_color(&self, bag: &str, color: &str) -> bool {
        for sub_bag in self.bags[bag].keys() {
            if sub_bag == color || self.contains_color(sub_bag, color) {
                return true;
            }
        }

        false
    }

    fn count_bag(&self, bag: &str) -> u32 {
        self.bags[bag]
            .iter()
            .fold(0, |acc, (b, n)| acc + (self.count_bag(b) + 1) * n)
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        self.bags
            .keys()
            .map(|bag| u32::from(self.contains_color(bag, "shiny gold")))
            .sum()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        self.count_bag("shiny gold")
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("sample_1.txt"));
        assert_eq!(puzzle.part1(), 4);
    }

    #[test]
    fn test_part2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("sample_2.txt"));
        assert_eq!(puzzle.part2(), 126);
    }
}
