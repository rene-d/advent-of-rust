//! [Day 7: Bridge Repair](https://adventofcode.com/2024/day/7)

// use std::collections::{FxHashMap,FxHashSet};

struct Equation {
    test_value: u64,
    values: Vec<u64>,
}

struct Puzzle {
    equations: Vec<Equation>,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            equations: Vec::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        for line in data.lines() {
            let (test_value, values) = line.split_once(':').unwrap();

            let equation = Equation {
                test_value: test_value.parse().unwrap(),
                values: values
                    .split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            };

            self.equations.push(equation);
        }
    }

    /// Check if there is a combination of operators + and * that solves the equation
    fn check_two_operators(equation: &Equation) -> bool {
        // the number of operations
        let n = (equation.values.len() - 1).try_into().unwrap();

        // iterate over all combinations
        for mut i in 0..2_u32.pow(n) {
            let mut result = equation.values[0];

            for value in &equation.values[1..] {
                if i % 2 == 0 {
                    result += value;
                } else {
                    result *= value;
                }
                i /= 2;

                if result > equation.test_value {
                    // unnecessary to continue: result will never equal test value
                    break;
                }
            }

            if result == equation.test_value {
                return true;
            }
        }

        false
    }

    /// Check if there is a combination of operators + * || that solves the equation
    fn check_three_operators(equation: &Equation) -> bool {
        // pre-compute the power of 10 for operator ||
        let mut pow10 = Vec::new();

        for value in &equation.values[1..] {
            let mut p = 1;
            let mut value = *value;
            while value != 0 {
                p *= 10;
                value /= 10;
            }
            pow10.push(p);
        }

        // the number of operations
        let n = (equation.values.len() - 1).try_into().unwrap();

        // iterate over all combinations
        for mut i in 0..3_u32.pow(n) {
            let mut result = equation.values[0];

            for (k, value) in equation.values[1..].iter().enumerate() {
                result = match i % 3 {
                    0 => result + value,
                    1 => result * value,
                    _ => result * pow10[k] + value,
                };
                i /= 3;

                if result > equation.test_value {
                    break;
                }
            }

            if result == equation.test_value {
                return true;
            }
        }

        false
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        self.equations
            .iter()
            .filter(|e| Self::check_two_operators(e))
            .map(|x| x.test_value)
            .sum()
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        self.equations
            .iter()
            .filter(|e| Self::check_three_operators(e))
            .map(|x| x.test_value)
            .sum()
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
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
        let data = aoc::load_input_data("test.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part1(), 3749);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        let data = aoc::load_input_data("test.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part2(), 11387);
    }
}
