//! [Day 13: Claw Contraption](https://adventofcode.com/2024/day/13)

use regex::Regex;
type F = fraction::GenericFraction<i64>;

struct ClawMachine {
    a_x: F,
    a_y: F,
    b_x: F,
    b_y: F,
    p_x: F,
    p_y: F,
}

impl ClawMachine {
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"\d+").unwrap();

        let values = re
            .find_iter(s)
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        Self {
            a_x: F::from(values[0]),
            a_y: F::from(values[1]),
            b_x: F::from(values[2]),
            b_y: F::from(values[3]),
            p_x: F::from(values[4]),
            p_y: F::from(values[5]),
        }
    }

    fn price(&self, position_offset: i64) -> i64 {
        let p_x = self.p_x + position_offset;
        let p_y = self.p_y + position_offset;

        let a = (p_y - self.b_y * p_x / self.b_x) / (self.a_y - self.b_y * self.a_x / self.b_x);
        let b = (p_x - a * self.a_x) / self.b_x;

        if a.denom() != Some(&1) || a.is_sign_negative() {
            return 0;
        }
        if b.denom() != Some(&1) || b.is_sign_negative() {
            return 0;
        }

        *(a * 3 + b).numer().unwrap()
    }
}

struct Puzzle {
    machines: Vec<ClawMachine>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { machines: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for s in data.split("\n\n") {
            self.machines.push(ClawMachine::parse(s));
        }
    }

    /// Solve part one.
    fn part1(&self) -> i64 {
        self.machines.iter().map(|machine| machine.price(0)).sum()
    }

    /// Solve part two.
    fn part2(&self) -> i64 {
        self.machines
            .iter()
            .map(|machine| machine.price(10_000_000_000_000))
            .sum()
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
        assert_eq!(puzzle.part1(), 480);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 875318608908);
    }
}
