//! [Day 11: Plutonian Pebbles](https://adventofcode.com/2024/day/11)

use std::collections::HashMap;

struct Puzzle {
    stones: Vec<u64>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { stones: Vec::new() }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.stones = data
            .split_ascii_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
    }

    fn blink(stone_counts: &HashMap<u64, u64>) -> HashMap<u64, u64> {
        let mut new_counts = HashMap::new();

        for (&stone, &count) in stone_counts {
            let new_stones = {
                // engraved with the number 0 => 1
                if stone == 0 {
                    vec![1]
                } else {
                    let s = stone.to_string();
                    let len = s.len();

                    if len % 2 == 0 {
                        // engraved with a number that has an even number of digits => split it into two stones
                        let mid = len / 2;
                        let left = s[0..mid].parse::<u64>().unwrap();
                        let right = s[mid..].parse::<u64>().unwrap();
                        vec![left, right]
                    } else {
                        // otherwise, multiply by 2024
                        vec![stone * 2024]
                    }
                }
            };

            for new_stone in new_stones {
                *new_counts.entry(new_stone).or_insert(0) += count;
            }
        }

        new_counts
    }

    fn solve(&self, blinks: usize) -> u64 {
        let mut stone_counts = HashMap::new();

        for &stone in &self.stones {
            *stone_counts.entry(stone).or_insert(0) += 1;
        }

        for _ in 0..blinks {
            stone_counts = Self::blink(&stone_counts);
        }

        stone_counts.values().sum()
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        self.solve(25)
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        self.solve(75)
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
        assert_eq!(puzzle.part1(), 55312);
    }
}
