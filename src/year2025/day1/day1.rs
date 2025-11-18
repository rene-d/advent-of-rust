//!

struct Puzzle {
    //
}

impl Puzzle {
    /// Initialize from the puzzle input.
    const fn new(data: &str) -> Self {
        Self {}
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        0
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        0
    }
}

/// # Panics
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

    // const TEST_INPUT: &str = include_str!("test.txt");
    // const SAMPLE_*: &str = include_str!("sample_*.txt");

    // #[test]
    // fn part1() {
    //     let puzzle = Puzzle::new(SAMPLE_1);
    //     assert_eq!(puzzle.part1(), 0);
    // }

    // #[test]
    // fn part2() {
    //     let puzzle = Puzzle::new(TEST_INPUT);
    //     assert_eq!(puzzle.part2(), 0);
    // }
}
