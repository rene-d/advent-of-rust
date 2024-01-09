//! [Day 1: Chronal Calibration](https://adventofcode.com/2018/day/1)

use std::collections::HashSet;

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
    fn part1(&self) -> i32 {
        self.data.lines().map(|x| x.parse::<i32>().unwrap()).sum()
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        let mut frequencies = HashSet::new();
        let mut sum = 0;
        loop {
            for i in self.data.lines() {
                sum += i.parse::<i32>().unwrap();
                if frequencies.contains(&sum) {
                    return sum;
                }
                frequencies.insert(sum);
            }
        }
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test1.txt");
        assert_eq!(puzzle.part1(), -6);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test2.txt");
        assert_eq!(puzzle.part2(), 14);
    }
}
