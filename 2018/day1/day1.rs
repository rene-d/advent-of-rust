//! [Day 1: Chronal Calibration](https://adventofcode.com/2018/day/1)

use rustc_hash::FxHashSet;

struct Puzzle<'a> {
    data: &'a str,
}

impl<'a> Puzzle<'a> {
    const fn new(data: &'a str) -> Self {
        Self { data }
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        self.data.lines().map(|x| x.parse::<i32>().unwrap()).sum()
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        let mut frequencies = FxHashSet::default();
        let mut sum = 0;
        loop {
            for i in self.data.lines() {
                sum += i.parse::<i32>().unwrap();
                if frequencies.contains(&sum) {
                    return sum;
                }
                frequencies.insert(sum);
            }
        }
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (i32, i32) {
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

    const TEST_INPUT_1: &str = include_str!("test1.txt");
    const TEST_INPUT_2: &str = include_str!("test2.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT_1);
        assert_eq!(puzzle.part1(), -6);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT_2);
        assert_eq!(puzzle.part2(), 14);
    }
}
