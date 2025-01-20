//! [Day 1: Inverse Captcha](https://adventofcode.com/2017/day/1)

struct Puzzle<'a> {
    data: &'a str,
}

impl<'a> Puzzle<'a> {
    const fn new(data: &'a str) -> Self {
        Self {
            data: data.trim_ascii(),
        }
    }

    fn compute(&self, offset: usize) -> u32 {
        let mut sum = 0;

        let length = self.data.len();

        for i in 0..length {
            let a = self.data.chars().nth(i).unwrap();
            let b = self.data.chars().nth((i + offset) % length).unwrap();
            if a == b {
                sum += a.to_digit(10).unwrap();
            }
        }

        sum
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        self.compute(1)
    }
    /// Solve part two.
    fn part2(&self) -> u32 {
        self.compute(self.data.len() / 2)
    }
}

fn solve(data: &str) -> (u32, u32) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

fn main() {
    let mut args = aoc::parse_args();
    args.run(solve);
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let puzzle = Puzzle::new("1122");
        assert_eq!(puzzle.part1(), 3);

        let puzzle = Puzzle::new("1111");
        assert_eq!(puzzle.part1(), 4);

        let puzzle = Puzzle::new("1234");
        assert_eq!(puzzle.part1(), 0);

        let puzzle = Puzzle::new("91212129");
        assert_eq!(puzzle.part1(), 9);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new("1212");
        assert_eq!(puzzle.part2(), 6);

        let puzzle = Puzzle::new("1221");
        assert_eq!(puzzle.part2(), 0);

        let puzzle = Puzzle::new("123425");
        assert_eq!(puzzle.part2(), 4);

        let puzzle = Puzzle::new("123123");
        assert_eq!(puzzle.part2(), 12);

        let puzzle = Puzzle::new("12131415");
        assert_eq!(puzzle.part2(), 4);
    }
}
