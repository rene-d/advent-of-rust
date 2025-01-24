//! [Day 4: Scratchcards](https://adventofcode.com/2023/day/4)

use rustc_hash::FxHashSet;

struct Puzzle {
    matching_cards: Vec<usize>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            matching_cards: data
                .lines()
                .map(|line| {
                    let line = line[line.find(':').unwrap() + 1..]
                        .split('|')
                        .collect::<Vec<_>>();

                    let winning = line[0]
                        .split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<FxHashSet<_>>();

                    let nums = line[1]
                        .split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<FxHashSet<_>>();

                    winning.intersection(&nums).count()
                })
                .collect(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let mut sum = 0;
        for n in &self.matching_cards {
            if n >= &1 {
                sum += 2usize.pow(u32::try_from(*n).unwrap() - 1);
            }
        }
        sum
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let mut copies = vec![0usize; self.matching_cards.len()];

        for i in 0..self.matching_cards.len() {
            copies[i] += 1;

            let m = self.matching_cards[i];
            for j in (i + 1)..(i + 1 + m) {
                copies[j] += copies[i];
            }
        }

        copies.iter().sum()
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
        assert_eq!(puzzle.part1(), 13);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 30);
    }
}
