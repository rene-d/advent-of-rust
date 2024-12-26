//! [Day 10: Adapter Array](https://adventofcode.com/2020/day/10)

use std::collections::HashMap;

struct Puzzle {
    adapters: Vec<i64>,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            adapters: Vec::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap_or_else(|_| {
            eprintln!("cannot read input file {path}");
            std::process::exit(1);
        });

        self.adapters
            .extend(data.lines().map_while(|line| line.parse::<i64>().ok()));

        self.adapters.sort_unstable();
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut diffs: HashMap<i64, u32> = HashMap::new();

        for w in self.adapters.windows(2) {
            let d = w[1] - w[0];

            *diffs.entry(d).or_default() += 1;
        }

        *diffs.entry(self.adapters[0] /*- 0*/).or_default() += 1; // charging outlet has an effective rating of 0 jolts
        *diffs.entry(3).or_default() += 1; // device's built-in adapter is always 3 higher

        diffs[&1] * diffs[&3]
    }

    /// Solve part two.
    fn part2(&self) -> i64 {
        let mut adapters = self.adapters.clone();

        adapters.insert(0, 0); // add the charging outlet

        let mut n = (0, 0, 1);

        for w in adapters.windows(2) {
            n = match w[1] - w[0] {
                1 => (n.1, n.2, n.0 + n.1 + n.2),
                2 => (n.2, 0, n.1 + n.2),
                3 => (0, 0, n.2),
                _ => n,
            }
        }

        n.2
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
    fn test_part1_1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_1.txt");
        assert_eq!(puzzle.part1(), 7 * 5);
    }

    #[test]
    fn test_part1_2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_2.txt");
        assert_eq!(puzzle.part1(), 22 * 10);
    }

    #[test]
    fn test_part2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_2.txt");
        assert_eq!(puzzle.part2(), 19208);
    }
}
