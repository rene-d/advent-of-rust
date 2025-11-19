//! [Day 16: Aunt Sue](https://adventofcode.com/2015/day/16)

use regex::Regex;
use rustc_hash::FxHashMap;

struct Puzzle<'a> {
    aunts: FxHashMap<u32, FxHashMap<&'a str, u32>>,
}

impl<'a> Puzzle<'a> {
    /// Parse the input data using manual string parsing (faster than regex).
    /// Parses lines in the format: "Sue 1: goldfish: 6, trees: 9, akitas: 0"
    fn new(data: &'a str) -> Self {
        let mut aunts: FxHashMap<u32, FxHashMap<&'a str, u32>> = FxHashMap::default();

        for line in data.lines() {
            let Some(rest) = line.strip_prefix("Sue ") else {
                continue;
            };

            let Some(colon_pos) = rest.find(':') else {
                continue;
            };

            let sue = rest[..colon_pos].parse::<u32>().unwrap();
            let mut props = rest[colon_pos + 1..].trim_start();

            let mut aunt = FxHashMap::default();

            while !props.is_empty() {
                // read the pair "<key>: <value>"
                let Some(key_end) = props.find(':') else {
                    break;
                };
                let key = props[..key_end].trim_end();

                props = &props[key_end + 1..];

                let value_end = props.find(',').unwrap_or(props.len());
                let value = props[..value_end].trim_start().parse::<u32>().unwrap();

                aunt.insert(key, value);

                // go to next pair
                props = props[value_end..].trim_start_matches(',').trim_start();
            }

            aunts.insert(sue, aunt);
        }
        Self { aunts }
    }

    /// Parse the input data using regex (slower but more readable).
    /// Parses lines in the format: "Sue 1: goldfish: 6, trees: 9, akitas: 0"
    fn new_regex(data: &'a str) -> Self {
        let mut aunts: FxHashMap<u32, FxHashMap<&'a str, u32>> = FxHashMap::default();

        let re = Regex::new(r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)").unwrap();

        for line in data.lines() {
            let m = re.captures(line).unwrap();
            let sue = m.get(1).unwrap().as_str().parse::<u32>().unwrap();

            let mut aunt = FxHashMap::default();

            for i in (2..=6).step_by(2) {
                let key = m.get(i).unwrap().as_str();
                let value = m.get(i + 1).unwrap().as_str().parse::<u32>().unwrap();
                aunt.insert(key, value);
            }

            aunts.insert(sue, aunt);
        }

        Self { aunts }
    }

    fn part1(&self) -> u32 {
        for (sue, aunt) in &self.aunts {
            if aunt.get("children").unwrap_or(&3) == &3
                && aunt.get("cats").unwrap_or(&7) == &7
                && aunt.get("samoyeds").unwrap_or(&2) == &2
                && aunt.get("pomeranians").unwrap_or(&3) == &3
                && aunt.get("akitas").unwrap_or(&0) == &0
                && aunt.get("vizslas").unwrap_or(&0) == &0
                && aunt.get("goldfish").unwrap_or(&5) == &5
                && aunt.get("trees").unwrap_or(&3) == &3
                && aunt.get("cars").unwrap_or(&2) == &2
                && aunt.get("perfumes").unwrap_or(&1) == &1
            {
                return *sue;
            }
        }
        0
    }

    fn part2(&self) -> u32 {
        for (sue, aunt) in &self.aunts {
            if aunt.get("children").unwrap_or(&3) == &3
                && aunt.get("cats").unwrap_or(&8) > &7          // should be greater than
                && aunt.get("samoyeds").unwrap_or(&2) == &2
                && aunt.get("pomeranians").unwrap_or(&2) < &3   // should be fewer than
                && aunt.get("akitas").unwrap_or(&0) == &0
                && aunt.get("vizslas").unwrap_or(&0) == &0
                && aunt.get("goldfish").unwrap_or(&4) < &5      // should be fewer than
                && aunt.get("trees").unwrap_or(&4) > &3         // should be greater than
                && aunt.get("cars").unwrap_or(&2) == &2
                && aunt.get("perfumes").unwrap_or(&1) == &1
            {
                return *sue;
            }
        }

        0
    }
}

#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

#[must_use]
pub fn solve_regex(data: &str) -> (u32, u32) {
    let puzzle = Puzzle::new_regex(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    if args.has_option("--use-regex") {
        args.run(solve_regex);
    } else {
        args.run(solve);
    }
}
