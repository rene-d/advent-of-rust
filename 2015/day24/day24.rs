//! [Day 24: It Hangs in the Balance](https://adventofcode.com/2015/day/24)

use itertools::Itertools;

struct Puzzle {
    packages: Vec<u64>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { packages: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.packages = data.lines().filter_map(|s| s.parse().ok()).collect();
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
        assert_eq!(puzzle.part1(), 99);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 44);
    }
}
