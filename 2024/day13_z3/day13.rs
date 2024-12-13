//! [Day 13: Claw Contraption](https://adventofcode.com/2024/day/13)

use regex::Regex;
use z3::ast::{Ast, Int};

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
        let cfg = z3::Config::new();
        let ctx = z3::Context::new(&cfg);
        let solver: z3::Solver<'_> = z3::Solver::new(&ctx);

        // unknowns
        let a = Int::new_const(&ctx, "a");
        let b = Int::new_const(&ctx, "b");

        // prize position
        let p_x = &Int::from_u64(&ctx, self.p_x + position_offset);
        let p_y = &Int::from_u64(&ctx, self.p_y + position_offset);

        // constraints
        solver.assert(&(&a * self.a_x + &b * self.b_x)._eq(p_x));
        solver.assert(&(&a * self.a_y + &b * self.b_y)._eq(p_y));

        // find a solution
        if solver.check() == z3::SatResult::Sat {
            if let Some(model) = solver.get_model() {
                let a_value = model.eval(&a, true).unwrap().as_u64().unwrap();
                let b_value = model.eval(&b, true).unwrap().as_u64().unwrap();

                return 3 * (a_value) + (b_value);
            }
        }

        0
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
