//! [Day 10: Adapter Array](https://adventofcode.com/2020/day/10)

use rustc_hash::FxHashMap;

struct Puzzle {
    adapters: Vec<i64>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut adapters = Vec::new();
        adapters.extend(data.lines().map_while(|line| line.parse::<i64>().ok()));
        adapters.sort_unstable();
        Self { adapters }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut diffs: FxHashMap<i64, u32> = FxHashMap::default();

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

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, i64) {
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

    const SAMPLE_1: &str = include_str!("sample_1.txt");
    const SAMPLE_2: &str = include_str!("sample_2.txt");

    #[test]
    fn test_part1_1() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part1(), 7 * 5);
    }

    #[test]
    fn test_part1_2() {
        let puzzle = Puzzle::new(SAMPLE_2);
        assert_eq!(puzzle.part1(), 22 * 10);
    }

    #[test]
    fn test_part2() {
        let puzzle = Puzzle::new(SAMPLE_2);
        assert_eq!(puzzle.part2(), 19208);
    }
}
