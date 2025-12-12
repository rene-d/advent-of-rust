//! [Day 10: Factory](https://adventofcode.com/2025/day/10)

use good_lp::{Expression, Solution, SolverModel, default_solver, variable, variables};
use rayon::prelude::*;
use z3::ast::Int;

// this is only to make clippy happy... cf. https://stackoverflow.com/a/74629224
const F64_BITS: u64 = 64;
const F64_EXPONENT_BITS: u64 = 11;
const F64_EXPONENT_MAX: u64 = (1 << F64_EXPONENT_BITS) - 1;
const F64_EXPONENT_BIAS: u64 = 1023;
const F64_FRACTION_BITS: u64 = 52;

pub fn f64_to_u64(f: f64) -> Option<u64> {
    let bits = f.to_bits();
    let sign = bits & (1 << (F64_EXPONENT_BITS + F64_FRACTION_BITS)) != 0;
    let exponent = (bits >> F64_FRACTION_BITS) & ((1 << F64_EXPONENT_BITS) - 1);
    let fraction = bits & ((1 << F64_FRACTION_BITS) - 1);

    match (sign, exponent, fraction) {
        (_, 0, 0) => {
            debug_assert!(f == 0.0);
            Some(0)
        }
        (true, _, _) => {
            debug_assert!(f < 0.0);
            None
        }
        (_, F64_EXPONENT_MAX, 0) => {
            debug_assert!(f.is_infinite());
            None
        }
        (_, F64_EXPONENT_MAX, _) => {
            debug_assert!(f.is_nan());
            None
        }
        (_, 0, _) => {
            debug_assert!(f.is_subnormal());
            None
        }
        _ => {
            if exponent < F64_EXPONENT_BIAS {
                debug_assert!(f < 1.0);
                None
            } else {
                let mantissa = fraction | (1 << F64_FRACTION_BITS);
                let left_shift =
                    exponent.cast_signed() - (F64_EXPONENT_BIAS + F64_FRACTION_BITS).cast_signed();
                if left_shift < 0 {
                    let right_shift = (-left_shift).cast_unsigned();
                    if mantissa & (1 << (right_shift - 1)) != 0 {
                        debug_assert!(f.fract() != 0.0);
                        None
                    } else {
                        Some(mantissa >> right_shift)
                    }
                } else if left_shift > (F64_BITS - F64_FRACTION_BITS - 1).cast_signed() {
                    debug_assert!(f > 2.0f64.powi(63));
                    None
                } else {
                    Some(mantissa << left_shift)
                }
            }
        }
    }
}

struct Machine {
    lights: u32,
    wirings: Vec<u32>,
    joltages: Vec<u32>,
}

impl Machine {
    /// Returns fewest required presses to correctly configure the indicator lights of the machine.
    fn score(&self) -> Option<usize> {
        (0..(1u32 << self.wirings.len()))
            .filter_map(|k| {
                let mut result = 0;
                let mut k_score = 0;

                for (i, wiring) in self.wirings.iter().enumerate() {
                    if k & (1 << i) != 0 {
                        // press button, xor toggles the lights
                        result ^= wiring;
                        k_score += 1;
                    }
                }

                if result == self.lights {
                    Some(k_score)
                } else {
                    None
                }
            })
            .min()
    }

    /// Return the fewest button presses required to correctly configure the joltage level counters.
    /// Use Integer Linear Programming solver.
    fn optimize_lp(&self) -> u64 {
        let n = self.wirings.len();

        // 1) Create the container for problem variables
        let mut vars = variables!();

        // 2) Create `n` integer variables and collect them
        let presses: Vec<_> = (0..n)
            .map(|_| vars.add(variable().integer().min(0)))
            .collect();

        //  3) Build the objective: minimize sum(w_i)
        let objective: Expression = presses.iter().sum();

        // 4) Create the problem with that objective
        let mut problem = vars.minimise(objective).using(default_solver);

        // 5) Add the equations.
        for (i, &target) in self.joltages.iter().enumerate() {
            let mut lhs = Expression::from(0);

            for (j, w) in self.wirings.iter().enumerate() {
                if w & (1 << i) != 0 {
                    lhs += presses[j];
                }
            }

            problem = problem.with(lhs.eq(target));
        }

        // 6) Solve with the default solver
        // 7) Extract integer values (solution.value returns f64)
        problem.solve().map_or(0, |solution| {
            presses
                .iter()
                .map(|&v| f64_to_u64(solution.value(v).round()).unwrap())
                .sum()
        })
    }

    /// Return the fewest button presses required to correctly configure the joltage level counters.
    /// Use Z3 solver (complicates parallelism).
    fn optimize_z3(&self) -> u64 {
        let solver = z3::Optimize::new();

        let vars: Vec<_> = (0..self.wirings.len())
            .map(|i| Int::new_const(format!("w{i}")))
            .collect();

        for v in &vars {
            solver.assert(&v.ge(Int::from_u64(0)));
        }

        for (i, &target) in self.joltages.iter().enumerate() {
            let mut terms = Vec::new();
            for (j, w) in self.wirings.iter().enumerate() {
                if w & (1 << i) != 0 {
                    terms.push(vars[j].clone());
                }
            }

            let sum = if terms.is_empty() {
                Int::from_u64(0)
            } else {
                Int::add(&terms.iter().collect::<Vec<_>>())
            };
            solver.assert(&sum.eq(Int::from_u64(target.into())));
        }

        let sum_vars = z3::ast::Int::add(&vars.iter().collect::<Vec<_>>());
        solver.minimize(&sum_vars);
        solver.minimize(&sum_vars); // Sometimes, the solver fails to find the optimal solution on the first attempt.

        match solver.check(&[]) {
            z3::SatResult::Sat => {
                let model = solver.get_model().unwrap();

                vars.iter()
                    .filter_map(|v| {
                        let val = model.get_const_interp(v).unwrap();
                        val.as_u64()
                    })
                    .sum()
            }
            _ => 0,
        }
    }
}

struct Puzzle {
    machines: Vec<Machine>,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        let mut machines = vec![];
        for line in data.lines() {
            let p: Vec<_> = line.split_ascii_whitespace().collect();

            // parse the indicator lights
            let mut lights = 0;
            for (k, light) in p[0]
                .strip_prefix('[')
                .unwrap()
                .strip_suffix(']')
                .unwrap()
                .chars()
                .enumerate()
            {
                if light == '#' {
                    lights += 1 << k;
                }
            }

            // parse the button wiring schematics
            let mut wirings = vec![];
            for w in &p[1..p.len() - 1] {
                wirings.push(
                    w.strip_prefix('(')
                        .unwrap()
                        .strip_suffix(')')
                        .unwrap()
                        .split(',')
                        .map(|s: &str| 1u32 << s.parse::<u32>().unwrap())
                        .sum(),
                );
            }

            // parse the joltage requirements
            let joltages = p
                .last()
                .unwrap()
                .strip_prefix('{')
                .unwrap()
                .strip_suffix('}')
                .unwrap()
                .split(',')
                .map(|s: &str| s.parse::<u32>().unwrap())
                .collect();

            machines.push(Machine {
                lights,
                wirings,
                joltages,
            });
        }
        Self { machines }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.machines.par_iter().filter_map(Machine::score).sum()
    }

    /// Solve part two.
    fn part2_lp(&self) -> u64 {
        self.machines.par_iter().map(Machine::optimize_lp).sum()
    }

    /// Solve part two too.
    fn part2_z3(&self) -> u64 {
        self.machines.par_iter().map(Machine::optimize_z3).sum()
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, u64) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2_lp())
}
/// # Panics
#[must_use]
pub fn solve_z3(data: &str) -> (usize, u64) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2_z3())
}

pub fn main() {
    let args = aoc::parse_args();

    if args.has_option("--z3") {
        args.run(solve_z3);
    } else {
        args.run(solve);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 7);
    }

    #[test]
    fn part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2_lp(), 33);
    }
}
