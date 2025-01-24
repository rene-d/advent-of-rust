//! [Day 7: Handy Haversacks](https://adventofcode.com/2020/day/7)

use regex::Regex;
use rustc_hash::FxHashMap;

struct Puzzle {
    // data: String,
    bags: FxHashMap<String, FxHashMap<String, u32>>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut bags = FxHashMap::default();

        let pat = Regex::new(r"^\s?(\d+) (.+) bags?$").unwrap();

        for line in data.lines() {
            let (bag, contain) = line.split_once(" bags contain ").unwrap();

            let contain = contain.strip_suffix('.').unwrap();

            let mut contained = FxHashMap::default();

            for s in contain.split(',') {
                if s == "no other bags" {
                    continue;
                }

                let caps = pat.captures(s).unwrap();

                let n: u32 = caps[1].parse().unwrap();
                let sub_bag = &caps[2];

                contained.insert(sub_bag.to_string(), n);
            }

            bags.insert(bag.to_string(), contained);
        }

        Self { bags }
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

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_1: &str = include_str!("sample_1.txt");
    const SAMPLE_2: &str = include_str!("sample_2.txt");

    #[test]
    fn test_part1() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part1(), 4);
    }

    #[test]
    fn test_part2() {
        let puzzle = Puzzle::new(SAMPLE_2);
        assert_eq!(puzzle.part2(), 126);
    }
}
