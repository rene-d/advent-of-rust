//! [Day 10: Knot Hash](https://adventofcode.com/2017/day/10)

use aoc::knot;

struct Puzzle<'a> {
    data: &'a str,
}

impl<'a> Puzzle<'a> {
    const fn new(data: &'a str) -> Self {
        Self {
            data: data.trim_ascii(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let lengths: Vec<_> = self
            .data
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();

        let mut skip = 0;
        let mut pos = 0;
        let mut sparse: Vec<u8> = (0..=255).collect();

        knot::tie(&lengths, &mut sparse, &mut skip, &mut pos);

        u32::from(sparse[0]) * u32::from(sparse[1])
    }

    /// Solve part two.
    fn part2(&self) -> String {
        knot::hash(self.data)
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u32, String) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}
