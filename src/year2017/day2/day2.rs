//! [Day 2: Corruption Checksum](https://adventofcode.com/2017/day/2)

struct Puzzle {
    rows: Vec<Vec<u32>>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut rows = vec![];
        for line in data.lines() {
            rows.push(
                line.split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            );
        }
        Self { rows }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        self.rows
            .iter()
            .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
            .sum()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        self.rows
            .iter()
            .map(|row| {
                for a in row {
                    for b in row {
                        if a > b && a % b == 0 {
                            return a / b;
                        }
                    }
                }
                0
            })
            .sum()
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
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
    fn test01() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part1(), 18);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(SAMPLE_2);
        assert_eq!(puzzle.part2(), 9);
    }
}
