//! [Day 19: Linen Layout](https://adventofcode.com/2024/day/19)

struct Puzzle<'a> {
    patterns: Vec<&'a str>,
    designs: Vec<&'a str>,
}

impl<'a> Puzzle<'a> {
    fn new(data: &'a str) -> Self {
        let (patterns, designs) = data.split_once("\n\n").unwrap();

        Self {
            patterns: patterns.split(", ").collect(),
            designs: designs.lines().collect(),
        }
    }

    fn count_design_ways(&self, design: &str) -> u64 {
        let n = design.len();
        let mut dp = vec![0; n + 1];
        dp[0] = 1;

        for i in 1..=n {
            for pattern in &self.patterns {
                let m = pattern.len();
                if i >= m && &&design[i - m..i] == pattern {
                    dp[i] += dp[i - m];
                }
            }
        }

        dp[n]
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.designs
            .iter()
            .filter(|design| self.count_design_ways(design) != 0)
            .count()
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        self.designs
            .iter()
            .map(|design| self.count_design_ways(design))
            .sum()
    }
}

fn solve(data: &str) -> (usize, u64) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

fn main() {
    let mut args = aoc::parse_args();
    args.run(solve);
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 6);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 16);
    }
}
