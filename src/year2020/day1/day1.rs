//! [Day 1: Report Repair](https://adventofcode.com/2020/day/1)

struct Puzzle {
    expenses: Vec<u64>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            expenses: data.lines().map(|s| s.parse().unwrap()).collect(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        for i in &self.expenses {
            for j in &self.expenses {
                if i + j == 2020 {
                    return i * j;
                }
            }
        }
        0
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        for i in &self.expenses {
            for j in &self.expenses {
                for k in &self.expenses {
                    if i + j + k == 2020 {
                        return i * j * k;
                    }
                }
            }
        }
        0
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
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 514579);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 241861950);
    }
}
