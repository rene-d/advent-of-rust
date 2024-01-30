//! [Day 3: No Matter How You Slice It](https://adventofcode.com/2018/day/3)

use regex::Regex;
use std::collections::{HashMap, HashSet};

struct Puzzle {
    data: String,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            data: String::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.data = data;
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let re = Regex::new(r"^#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)$").unwrap();

        let mut squares = HashMap::new();

        for line in self.data.lines() {
            if let Some(caps) = re.captures(line) {
                let s = caps
                    .iter()
                    .skip(2)
                    .map(|c| c.unwrap().as_str().parse::<u32>().unwrap())
                    .collect::<Vec<_>>();

                let x = s[0];
                let y = s[1];

                let width = s[2];
                let height = s[3];

                for i in x..(x + width) {
                    for j in y..(y + height) {
                        *squares.entry((i, j)).or_insert(0) += 1u32;
                    }
                }
            }
        }

        squares.values().filter(|&&v| v > 1).count()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let re = Regex::new(r"^#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)$").unwrap();

        let mut squares_id: HashMap<(u32, u32), u32> = HashMap::new();
        let mut intact = HashSet::new();

        for line in self.data.lines() {
            if let Some(caps) = re.captures(line) {
                let s = caps
                    .iter()
                    .skip(1)
                    .map(|c| c.unwrap().as_str().parse::<u32>().unwrap())
                    .collect::<Vec<_>>();

                let id = s[0];

                let x = s[1];
                let y = s[2];

                let width = s[3];
                let height = s[4];

                intact.insert(id);

                for i in x..(x + width) {
                    for j in y..(y + height) {
                        let key = (i, j);

                        if let std::collections::hash_map::Entry::Vacant(e) = squares_id.entry(key)
                        {
                            e.insert(id);
                        } else {
                            intact.remove(&id);

                            let o = squares_id[&key];
                            intact.remove(&o);
                        }
                    }
                }
            }
        }

        *intact.iter().next().unwrap()
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
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
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 4);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 3);
    }
}
