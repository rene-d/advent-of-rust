//! [Day 9: Mirage Maintenance](https://adventofcode.com/2023/day/9)

struct Puzzle {
    histories: Vec<Vec<i64>>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            histories: data
                .lines()
                .map(|line| {
                    let history: Vec<_> = line
                        .split_whitespace()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect();
                    history
                })
                .collect(),
        }
    }

    fn history_diffs(history: &[i64]) -> Vec<Vec<i64>> {
        let mut diffs = vec![];

        let mut history = history.to_vec();

        loop {
            // stop when the sequence of differences has only zeros
            if history.iter().min().unwrap() == &0 && history.iter().max().unwrap() == &0 {
                return diffs;
            }

            diffs.push(history.clone());

            history = history
                .iter()
                .zip(history.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect();
        }
    }

    /// Solve part one.
    fn part1(&self) -> i64 {
        let mut result = 0;

        for history in &self.histories {
            let diffs = Self::history_diffs(history);

            // sum of numbers at the right
            let n = diffs.iter().fold(0, |acc, x| acc + x.last().unwrap());

            result += n;
        }

        result
    }

    /// Solve part two.
    fn part2(&self) -> i64 {
        self.histories
            .iter()
            .map(|history| {
                Self::history_diffs(history)
                    .iter()
                    .rev()
                    .fold(0, |acc, x| x.first().unwrap() - acc)
            })
            .sum::<i64>()
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (i64, i64) {
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
        assert_eq!(puzzle.part1(), 114);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 2);
    }
}
