//! [Day 13: Packet Scanners](https://adventofcode.com/2017/day/13)

use rustc_hash::FxHashMap;

struct Puzzle {
    heights: FxHashMap<u32, u32>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut heights = FxHashMap::default();

        for line in data.lines() {
            let mut line = line.split(": ");

            let pos: u32 = line.next().unwrap().parse().unwrap();
            let height: u32 = line.next().unwrap().parse().unwrap();

            heights.insert(pos, height);
        }

        Self { heights }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        self.heights
            .iter()
            .filter(|&(&pos, &height)| pos % (2 * (height - 1)) == 0)
            .map(|(&pos, &height)| pos * height)
            .sum()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        (0..10_000_000)
            .find(|wait| {
                !self
                    .heights
                    .iter()
                    .any(|(&pos, &height)| (wait + pos) % (2 * (height - 1)) == 0)
            })
            .unwrap()
    }
}

/// # Panics
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

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 24);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 10);
    }
}
