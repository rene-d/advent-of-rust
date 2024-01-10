//! [Day 1: Inverse Captcha](https://adventofcode.com/2017/day/1)

struct Puzzle {
    data: String,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            data: String::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.data = data.trim().to_string();
    }

    #[cfg(test)]
    fn init(&mut self, data: &str) {
        self.data = data.trim().to_string();
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
        puzzle.init("1122");
        assert_eq!(puzzle.part1(), 3);

        puzzle.init("1111");
        assert_eq!(puzzle.part1(), 4);

        puzzle.init("1234");
        assert_eq!(puzzle.part1(), 0);

        puzzle.init("91212129");
        assert_eq!(puzzle.part1(), 9);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();

        puzzle.init("1212");
        assert_eq!(puzzle.part2(), 6);

        puzzle.init("1221");
        assert_eq!(puzzle.part2(), 0);

        puzzle.init("123425");
        assert_eq!(puzzle.part2(), 4);

        puzzle.init("123123");
        assert_eq!(puzzle.part2(), 12);

        puzzle.init("12131415");
        assert_eq!(puzzle.part2(), 4);
    }
}
