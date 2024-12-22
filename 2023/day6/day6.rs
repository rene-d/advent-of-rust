//! [Day 6: Wait For It](https://adventofcode.com/2023/day/6)

struct Puzzle {
    time: String,
    distance: String,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            time: String::new(),
            distance: String::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        let mut lines = data.lines();
        self.time = lines
            .next()
            .unwrap()
            .strip_prefix("Time:")
            .unwrap()
            .to_string();
        self.distance = lines
            .next()
            .unwrap()
            .strip_prefix("Distance:")
            .unwrap()
            .to_string();
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

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 288);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 71503);
    }
}
