//! [Day 13: Claw Contraption](https://adventofcode.com/2024/day/13)

use regex::Regex;
use z3::ast::Int;

struct ClawMachine {
    a_x: u64,
    a_y: u64,
    b_x: u64,
    b_y: u64,
    p_x: u64,
    p_y: u64,
}

impl ClawMachine {
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"\d+").unwrap();

        let values = re
            .find_iter(s)
            .map(|m| m.as_str().parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        Self {
            a_x: values[0],
            a_y: values[1],
            b_x: values[2],
            b_y: values[3],
            p_x: values[4],
            p_y: values[5],
        }
    }

    fn price(&self, position_offset: u64) -> u64 {
        let solver: z3::Solver = z3::Solver::new();

        // unknowns
        let a = Int::new_const("a");
        let b = Int::new_const("b");

        // prize position
        let p_x = &Int::from_u64(self.p_x + position_offset);
        let p_y = &Int::from_u64(self.p_y + position_offset);

        // constraints
        solver.assert((&a * self.a_x + &b * self.b_x).eq(p_x));
        solver.assert((&a * self.a_y + &b * self.b_y).eq(p_y));

        // find a solution
        if solver.check() == z3::SatResult::Sat
            && let Some(model) = solver.get_model() {
                let a_value = model.eval(&a, true).unwrap().as_u64().unwrap();
                let b_value = model.eval(&b, true).unwrap().as_u64().unwrap();

                return 3 * (a_value) + (b_value);
            }

        0
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
    fn part1(&self) -> u64 {
        self.machines.iter().map(|machine| machine.price(0)).sum()
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        self.machines
            .iter()
            .map(|machine| machine.price(10_000_000_000_000))
            .sum()
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
        assert_eq!(puzzle.part1(), 480);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 875318608908);
    }
}
