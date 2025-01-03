//! [Day 19: Linen Layout](https://adventofcode.com/2024/day/19)

struct Puzzle {
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            patterns: Vec::new(),
            designs: Vec::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        let (patterns, designs) = data.split_once("\n\n").unwrap();

        self.patterns = patterns
            .split(", ")
            .map(std::string::ToString::to_string)
            .collect();
        self.designs = designs
            .lines()
            .map(std::string::ToString::to_string)
            .collect();
    }

    fn count_design_ways(&self, design: &str) -> u64 {
        let n = design.len();
        let mut dp = vec![0; n + 1];
        dp[0] = 1;

        for i in 1..=n {
            for pattern in &self.patterns {
                let m = pattern.len();
                if i >= m && &design[i - m..i] == pattern {
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

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
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
        let data = aoc::load_input_data("test.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part1(), 6);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        let data = aoc::load_input_data("test.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part2(), 16);
    }
}
