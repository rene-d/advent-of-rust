//! [Day 17: Spinlock](https://adventofcode.com/2017/day/17)

struct Puzzle {
    step: usize,
}

impl Puzzle {
    const fn new() -> Self {
        Self { step: 0 }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        self.step = data.trim().parse().unwrap();
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut buf = vec![0];
        let mut pos = 0;

        for i in 1..=2017 {
            pos = (pos + self.step) % buf.len() + 1;
            buf.insert(pos, i);
        }

        buf[pos + 1]
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let mut buflen = 1;
        let mut pos = 0;
        let mut result = 0;

        for i in 1..50_000_000 {
            pos = (pos + self.step) % buflen + 1;
            buflen += 1;
            if pos == 1 {
                result = i;
            }
        }

        result
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
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
        puzzle.step = 3;
        assert_eq!(puzzle.part1(), 638);
    }
}
