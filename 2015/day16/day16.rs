//! [Day 16: Aunt Sue](https://adventofcode.com/2015/day/16)

use regex::Regex;
use rustc_hash::FxHashMap;

struct Puzzle<'a> {
    aunts: FxHashMap<u32, FxHashMap<&'a str, u32>>,
}

impl<'a> Puzzle<'a> {
    fn new(data: &'a str) -> Self {
        let mut aunts: FxHashMap<u32, FxHashMap<&'a str, u32>> = FxHashMap::default();

        let re = Regex::new(r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)").unwrap();

        for line in data.lines() {
            let m = re.captures(line).unwrap();
            let sue = m.get(1).unwrap().as_str().parse::<u32>().unwrap();

            for i in (2..=6).step_by(2) {
                let key = m.get(i).unwrap().as_str();
                let value = m.get(i + 1).unwrap().as_str().parse::<u32>().unwrap();
                aunts.entry(sue).or_default().insert(key, value);
            }
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

/// main function
fn solve(data: &str) -> (u32, u32) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

fn main() {
    let mut args = aoc::parse_args();
    args.run(solve);
}
