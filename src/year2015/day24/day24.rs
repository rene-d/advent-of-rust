//! [Day 24: It Hangs in the Balance](https://adventofcode.com/2015/day/24)

use itertools::Itertools;

struct Puzzle {
    packages: Vec<u64>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            packages: data.lines().filter_map(|s| s.parse().ok()).collect(),
        }
    }

    fn solve(&self, ngroups: u64) -> u64 {
        let weight = self.packages.iter().sum::<u64>() / ngroups;

        for k in 0..self.packages.len() {
            let g = self
                .packages
                .iter()
                .combinations(k)
                .filter(|x| x.iter().copied().sum::<u64>() == weight);

            if let Some(m) = g.map(|p| p.iter().copied().product::<u64>()).min() {
                return m;
            }
        }

        0
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        self.solve(3)
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        self.solve(4)
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u64, u64) {
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
        assert_eq!(puzzle.part1(), 99);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 44);
    }
}
