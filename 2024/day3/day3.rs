//! [Day 3: Mull It Over](https://adventofcode.com/2024/day/3)

use regex::Regex;

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
        self.data = std::fs::read_to_string(path).unwrap();
    }

    /// Compute valid mul() operations.
    /// if part1 is false, take care of do()/don't() statements.
    fn solve(data: &str, part1: bool) -> u64 {
        let mut enabled = true;
        let mut total_sum = 0;
        let mut i = 0;

        let pattern = Regex::new(r"^mul\((\d+),(\d+)\).*").unwrap();

        while i < data.len() {
            if i + 4 < data.len() && &data[i..i + 4] == "do()" {
                enabled = true;
                i += 4;
            } else if i + 7 < data.len() && &data[i..i + 7] == "don't()" {
                enabled = false;
                i += 7;
            } else if i + 4 < data.len() && &data[i..i + 4] == "mul(" {
                if let Some(caps) = pattern.captures(&data[i..]) {
                    let x: u64 = caps[1].parse().unwrap();
                    let y: u64 = caps[2].parse().unwrap();
                    if enabled || part1 {
                        total_sum += x * y;
                    }
                    i += caps.len();
                } else {
                    i += 4;
                }
            } else {
                i += 1;
            }
        }

        total_sum
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        Self::solve(&self.data, true)
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        Self::solve(&self.data, false)
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
        puzzle.configure("sample_1.txt");
        assert_eq!(puzzle.part1(), 161);
        assert_eq!(puzzle.part2(), 161);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_2.txt");
        assert_eq!(puzzle.part1(), 161);
        assert_eq!(puzzle.part2(), 48);
    }
}
