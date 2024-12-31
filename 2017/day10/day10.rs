//! [Day 10: Knot Hash](https://adventofcode.com/2017/day/10)

use aoc::knot;

struct Puzzle {
    data: String,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        data.trim().clone_into(&mut self.data);
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
        knot::hash(&self.data)
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
