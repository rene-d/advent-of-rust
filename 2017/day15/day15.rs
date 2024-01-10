//! [Day 15: Dueling Generators](https://adventofcode.com/2017/day/15)

struct Puzzle {
    a: u64,
    b: u64,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { a: 0, b: 0 }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            if let Some(a) = line.strip_prefix("Generator A starts with ") {
                self.a = a.parse().unwrap();
            } else if let Some(b) = line.strip_prefix("Generator B starts with ") {
                self.b = b.parse().unwrap();
            }
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut a: u64 = self.a;
        let mut b: u64 = self.b;
        let mut n = 0;
        for _ in 0..40_000_000 {
            a = a.wrapping_mul(16807) % 2147483647;
            b = b.wrapping_mul(48271) % 2147483647;

            if a & 0xffff == b & 0xffff {
                n += 1;
            }
        }
        n
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let mut a: u64 = self.a;
        let mut b: u64 = self.b;
        let mut n = 0;
        for _ in 0..5_000_000 {
            loop {
                a = a.wrapping_mul(16807) % 2147483647;
                if a % 4 == 0 {
                    break;
                }
            }
            loop {
                b = b.wrapping_mul(48271) % 2147483647;
                if b % 8 == 0 {
                    break;
                }
            }

            if a & 0xffff == b & 0xffff {
                n += 1;
            }
        }
        n
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
        puzzle.a = 65;
        puzzle.b = 8921;
        assert_eq!(puzzle.part1(), 588);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.a = 65;
        puzzle.b = 8921;
        assert_eq!(puzzle.part2(), 309);
    }
}
