//! [Day 15: Dueling Generators](https://adventofcode.com/2017/day/15)

struct Puzzle {
    a: u64,
    b: u64,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut a = 0;
        let mut b = 0;

        for line in data.lines() {
            if let Some(gen_a) = line.strip_prefix("Generator A starts with ") {
                a = gen_a.parse().unwrap();
            } else if let Some(gen_b) = line.strip_prefix("Generator B starts with ") {
                b = gen_b.parse().unwrap();
            }
        }

        Self { a, b }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut a: u64 = self.a;
        let mut b: u64 = self.b;
        let mut n = 0;
        for _ in 0..40_000_000 {
            a = a.wrapping_mul(16807) % 2_147_483_647;
            b = b.wrapping_mul(48271) % 2_147_483_647;

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
                a = a.wrapping_mul(16807) % 2_147_483_647;
                if a % 4 == 0 {
                    break;
                }
            }
            loop {
                b = b.wrapping_mul(48271) % 2_147_483_647;
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

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
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
        let mut puzzle = Puzzle::new("");
        puzzle.a = 65;
        puzzle.b = 8921;
        assert_eq!(puzzle.part1(), 588);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new("");
        puzzle.a = 65;
        puzzle.b = 8921;
        assert_eq!(puzzle.part2(), 309);
    }
}
