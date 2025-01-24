//! [Day 17: Spinlock](https://adventofcode.com/2017/day/17)

struct Puzzle {
    step: usize,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            step: data.trim().parse().unwrap(),
        }
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

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, usize) {
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

    #[test]
    fn test01() {
        let puzzle = Puzzle::new("3");
        assert_eq!(puzzle.part1(), 638);
    }
}
