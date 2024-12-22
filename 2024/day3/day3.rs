//! [Day 3: Mull It Over](https://adventofcode.com/2024/day/3)

use regex::Regex;

struct Puzzle {
    data: String,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        self.data = std::fs::read_to_string(path).unwrap();
    }

    /// Compute valid `mul()` operations.
    /// if part2 is true, take care of `do()`/`don't()` statements.
    fn solve(data: &str, part2: bool) -> i32 {
        let mut enabled = true;
        let mut total_sum = 0;

        let pattern = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

        for m in pattern.captures_iter(data) {
            if m.get(0).unwrap().as_str() == "do()" {
                enabled = true;
            } else if m.get(0).unwrap().as_str() == "don't()" {
                enabled = false;
            } else if enabled || !part2 {
                let x = m[1].parse::<i32>().unwrap();
                let y = m[2].parse::<i32>().unwrap();
                total_sum += x * y;
            }
        }

        total_sum
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        Self::solve(&self.data, false)
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        Self::solve(&self.data, true)
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
