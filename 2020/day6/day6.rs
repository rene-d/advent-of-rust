//! [Day 6: Custom Customs](https://adventofcode.com/2020/day/6)

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
    fn part1(&self) -> usize {
        self.data
            .split("\n\n")
            .map(|group| {
                // keeping only letters allows to ignore '\n'
                let group: HashSet<_> = group.chars().filter(char::is_ascii_alphabetic).collect();
                group.len()
            })
            .sum()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        self.data
            .split("\n\n")
            // for each group
            .map(|group| {
                group
                    // split lines
                    .lines()
                    // set of unique letters
                    .map(|person| person.chars().collect::<HashSet<_>>())
                    // intersection of all of them
                    .reduce(|a, b| a.intersection(&b).copied().collect())
                    .unwrap()
                    // get the length of this intersection
                    .len()
            })
            // the answer is the sum of lengths
            .sum()
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
        assert_eq!(puzzle.part1(), 11);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 6);
    }
}
