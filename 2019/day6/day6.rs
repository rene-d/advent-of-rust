//! [Day 6: Universal Orbit Map](https://adventofcode.com/2019/day/6)

// use std::collections::{FxHashMap,FxHashSet};

use rustc_hash::FxHashMap;

struct Puzzle {
    orbits: FxHashMap<String, String>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            orbits: FxHashMap::default(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        for line in data.lines() {
            if let Some((a, b)) = line.split_once(')') {
                self.orbits.insert(b.to_string(), a.to_string());
            }
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut total = 0;

        for orbit in self.orbits.keys() {
            let mut orbit = orbit.as_str();

            while orbit != "COM" {
                orbit = &self.orbits[orbit];
                total += 1;
            }
        }

        total
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let mut you_orbits = Vec::new();
        let mut san_orbits = Vec::new();

        let mut orbit = "YOU";
        while orbit != "COM" {
            orbit = &self.orbits[orbit];
            you_orbits.push(orbit);
        }

        orbit = "SAN";
        while orbit != "COM" {
            orbit = &self.orbits[orbit];
            san_orbits.push(orbit);
        }

        let mut common = 0;
        while you_orbits[you_orbits.len() - 1 - common] == san_orbits[san_orbits.len() - 1 - common]
        {
            common += 1;
        }

        you_orbits.len() + san_orbits.len() - common * 2
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
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("sample_2.txt"));
        assert_eq!(puzzle.part1(), 42);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("sample_4.txt"));
        assert_eq!(puzzle.part2(), 4);
    }
}
