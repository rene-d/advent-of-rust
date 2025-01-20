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
    fn new(data: &str) -> Self {
        Self {
            machines: data.split("\n\n").map(ClawMachine::parse).collect(),
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

fn solve(data: &str) -> (i64, i64) {
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

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 480);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 875318608908);
    }
}
