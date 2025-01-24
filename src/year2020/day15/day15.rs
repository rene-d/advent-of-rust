//! [Day 15: Rambunctious Recitation](https://adventofcode.com/2020/day/15)

use rustc_hash::FxHashMap;

struct Puzzle {
    nums: Vec<usize>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            nums: data
                .trim()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect(),
        }
    }

    fn solve(&self, number_spoken: usize) -> usize {
        let mut last_spoken = FxHashMap::default();
        let mut last_last_spoken = FxHashMap::default();
        let mut first_spoken = false;
        let mut last = 0;
        let mut n = 0;

        for turn in 1.. {
            if turn <= self.nums.len() {
                n = self.nums[turn - 1];
            } else if first_spoken {
                n = 0;
            } else {
                n = last_spoken[&last] - last_last_spoken[&n];
            };

            if turn == number_spoken {
                break;
            }

            if last_spoken.contains_key(&n) {
                first_spoken = false;
                last_last_spoken.insert(n, last_spoken[&n]);
            } else {
                first_spoken = true;
            }
            last_spoken.insert(n, turn);
            last = n;
        }

        n
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.solve(2020)
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        self.solve(30_000_000)
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
    fn test_part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 436);
    }
}
