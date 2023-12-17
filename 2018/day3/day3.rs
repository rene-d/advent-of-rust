//! [Day 3: No Matter How You Slice It](https://adventofcode.com/2018/day/3)

use clap::Parser;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Puzzle {
    data: String,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            data: "".to_string(),
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

                let w = s[2];
                let h = s[3];

                for i in x..(x + w) {
                    for j in y..(y + h) {
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

                let w = s[3];
                let h = s[4];

                intact.insert(id);

                for i in x..(x + w) {
                    for j in y..(y + h) {
                        let key = (i, j);

                        if squares_id.contains_key(&key) {
                            intact.remove(&id);

                            let o = squares_id[&key];
                            intact.remove(&o);
                        } else {
                            squares_id.insert(key, id);
                        }
                    }
                }
            }
        }

        *intact.iter().next().unwrap()
    }
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

fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
