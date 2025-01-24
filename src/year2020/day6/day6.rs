//! [Day 6: Custom Customs](https://adventofcode.com/2020/day/6)

use rustc_hash::FxHashSet;

struct Puzzle<'a> {
    data: &'a str,
}

impl<'a> Puzzle<'a> {
    const fn new(data: &'a str) -> Self {
        Self { data }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.data
            .split("\n\n")
            .map(|group| {
                // keeping only letters allows to ignore '\n'
                let group: FxHashSet<_> = group.chars().filter(char::is_ascii_alphabetic).collect();
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
                    .map(|person| person.chars().collect::<FxHashSet<_>>())
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

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, usize) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 11);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 6);
    }
}
