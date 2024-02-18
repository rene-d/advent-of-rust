//! [Day 23: Opening the Turing Lock](https://adventofcode.com/2015/day/23)

use std::{ops::Index, ops::IndexMut};

struct Registers {
    a: u32,
    b: u32,
}

impl Index<&str> for Registers {
    type Output = u32;
    fn index(&self, reg: &str) -> &u32 {
        match reg {
            "a" => &self.a,
            "b" => &self.b,
            _ => panic!(),
        }
    }
}

impl IndexMut<&str> for Registers {
    fn index_mut(&mut self, reg: &str) -> &mut Self::Output {
        match reg {
            "a" => &mut self.a,
            "b" => &mut self.b,
            _ => panic!(),
        }
    }
}

struct Puzzle {
    program: Vec<Vec<String>>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { program: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.program = data
            .lines()
            .map(|s| s.replace(',', " "))
            .map(|s| {
                s.split_ascii_whitespace()
                    .map(str::to_string)
                    .collect::<Vec<String>>()
            })
            .collect();
    }

    fn run_instr(&self, ip: usize, regs: &mut Registers) -> isize {
        let op = &self.program[ip];

        match op[0].as_str() {
            "hlf" => {
                let reg = &mut regs[&op[1]];
                *reg /= 2;
            }
            "tpl" => {
                let reg = &mut regs[&op[1]];
                *reg *= 3;
            }
            "inc" => {
                let reg = &mut regs[&op[1]];
                *reg += 1;
            }
            "jmp" => {
                return op[1].parse().unwrap();
            }
            "jie" => {
                if regs[&op[1]] % 2 == 0 {
                    return op[2].parse().unwrap();
                }
            }
            "jio" => {
                if regs[&op[1]] == 1 {
                    return op[2].parse().unwrap();
                }
            }
            _ => (),
        };
        1
    }

    fn run(&self, a: u32) -> Registers {
        let mut regs = Registers { a, b: 0 };

        let mut ip = 0;
        while ip < self.program.len() {
            let rel = self.run_instr(ip, &mut regs);

            if rel == 0 {
                // prevent infinite loop
                break;
            }

            ip = ip.checked_add_signed(rel).unwrap();
        }

        regs
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        self.run(0).b
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        self.run(1).b
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
        assert_eq!(puzzle.run(0).a, 2);
    }
}
