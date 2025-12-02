//! [Day 2: Gift Shop](https://adventofcode.com/2025/day/2)

use rustc_hash::FxHashSet;

struct Puzzle {
    ranges: Vec<(u64, u64)>,
    max_end: u64,
    max_digits: usize,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        let mut ranges = Vec::new();

        for range in data.trim().split(',') {
            if let Some((a, b)) = range.split_once('-') {
                let a: u64 = a.parse().unwrap();
                let b: u64 = b.parse().unwrap();
                ranges.push((a, b));
            }
        }

        ranges.sort_unstable_by_key(|r| r.0);

        let max_end = ranges.iter().map(|r| r.1).max().unwrap();
        let max_digits = max_end.to_string().len();

        Self {
            ranges,
            max_end,
            max_digits,
        }
    }

    fn in_ranges(&self, n: u64) -> bool {
        match self.ranges.binary_search_by(|(a, _)| a.cmp(&n)) {
            Ok(i) => self.ranges[i].0 <= n && n <= self.ranges[i].1,
            Err(i) => {
                if i == 0 {
                    return false;
                }
                self.ranges[i - 1].0 <= n && n <= self.ranges[i - 1].1
            }
        }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        let mut answer = 0;

        let mut h_min = 1;
        let mut h_max = 10;

        for _ in 1..=self.max_digits / 2 {
            for h in h_min..self.max_end.min(h_max) {
                let n = h * h_max + h;

                if self.in_ranges(n) {
                    answer += n;
                }
            }

            h_min *= 10;
            h_max *= 10;
        }

        answer
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        let mut found = FxHashSet::default();

        let mut s_min = 1;
        let mut s_max = 10;

        for h in 1..=self.max_digits / 2 {
            for s in s_min..s_max {
                let mut n = s;

                for _ in 1..=(self.max_digits / h) {
                    n = n * s_max + s;

                    if n > self.max_end {
                        break;
                    }

                    if self.in_ranges(n) {
                        found.insert(n);
                    }
                }
            }

            s_min *= 10;
            s_max *= 10;
        }

        found.iter().sum()
    }
}

/// # Panics
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
    fn part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 1227775554);
    }

    #[test]
    fn part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 4174379265);
    }
}
