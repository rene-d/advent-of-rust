//! [Day 11: Plutonian Pebbles](https://adventofcode.com/2024/day/11)

use rustc_hash::FxHashMap;

struct Puzzle {
    stones: Vec<u64>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            stones: data
                .split_ascii_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect(),
        }
    }

    /// Blink all stone within the frequency map according to the blink process:
    /// - stone 0 becomes stone 1
    /// - stone with a even length number splits into two
    /// - otherwise stone number is multiplied by 2024
    fn blink(stone_counts: &FxHashMap<u64, u64>) -> FxHashMap<u64, u64> {
        let mut new_counts = FxHashMap::default();

        for (&stone, &count) in stone_counts {
            let new_stones = {
                // engraved with the number 0 => 1
                if stone == 0 {
                    vec![1]
                } else {
                    let s = stone.to_string();
                    let len = s.len();

                    if len % 2 == 0 {
                        // engraved with a number that has an even number of digits => split it into two stones
                        let mid = len / 2;
                        let left = s[0..mid].parse::<u64>().unwrap();
                        let right = s[mid..].parse::<u64>().unwrap();
                        vec![left, right]
                    } else {
                        // otherwise, multiply by 2024
                        vec![stone * 2024]
                    }
                }
            };

            for new_stone in new_stones {
                *new_counts.entry(new_stone).or_insert(0) += count;
            }
        }

        new_counts
    }

    /// Blink `blinks` times and returns the total number of stones.
    fn solve(&self, blinks: usize) -> u64 {
        let mut stone_counts = FxHashMap::default();

        for &stone in &self.stones {
            *stone_counts.entry(stone).or_insert(0) += 1;
        }

        for _ in 0..blinks {
            stone_counts = Self::blink(&stone_counts);
        }

        stone_counts.values().sum()
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        self.solve(25)
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        self.solve(75)
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
        assert_eq!(puzzle.part1(), 55312);
    }
}
