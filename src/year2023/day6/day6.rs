//! [Day 6: Wait For It](https://adventofcode.com/2023/day/6)

struct Puzzle {
    time: String,
    distance: String,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut lines = data.lines();

        Self {
            time: lines
                .next()
                .unwrap()
                .strip_prefix("Time:")
                .unwrap()
                .to_string(),
            distance: lines
                .next()
                .unwrap()
                .strip_prefix("Distance:")
                .unwrap()
                .to_string(),
        }
    }

    fn win(t: u64, d: u64) -> u64 {
        // nota: see Python version for an elegant math solution 🤓
        let mut win = 0;
        for hold in 1..t {
            let speed = hold;
            let remaining = t - hold;
            let travelled = speed * remaining;
            if travelled > d {
                win += 1;
            }
        }
        win
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        let time = self
            .time
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap());
        let distance = self
            .distance
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap());

        let mut result = 1;
        for (t, d) in time.zip(distance) {
            result *= Self::win(t, d);
        }
        result
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        let time = self.time.replace(' ', "").parse::<u64>().unwrap();
        let distance = self.distance.replace(' ', "").parse::<u64>().unwrap();

        Self::win(time, distance)
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
        assert_eq!(puzzle.part1(), 288);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 71503);
    }
}
