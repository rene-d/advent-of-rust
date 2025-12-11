//! [Day 10: Factory](https://adventofcode.com/2025/day/10)

use z3::ast::Int;

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

    /// Return the fewest button presses required to correctly configure the joltage level counters .
    fn optimize(&self) -> u64 {
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
        self.machines.iter().filter_map(Machine::score).sum()
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        self.machines.iter().map(Machine::optimize).sum()
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, u64) {
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
    fn part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 7);
    }

    #[test]
    fn part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 33);
    }
}
