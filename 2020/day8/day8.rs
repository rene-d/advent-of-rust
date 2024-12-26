//! [Day 8: Handheld Halting](https://adventofcode.com/2020/day/8)

use std::collections::HashSet;

fn run(boot_code: &[String]) -> (i32, bool) {
    let mut acc = 0;
    let mut ip = 0;
    let mut seen = HashSet::new();

    while ip < boot_code.len() && seen.insert(ip) {
        let instr = &boot_code[ip];
        ip += 1;

        let (op, imm) = instr.split_once(' ').unwrap();

        match op {
            "nop" => continue,
            "acc" => {
                acc += imm.parse::<i32>().unwrap();
            }
            "jmp" => {
                ip = ip.wrapping_add_signed(imm.parse::<isize>().unwrap()) - 1;
            }
            _ => panic!("invalid instr {instr}"),
        };
    }

    (acc, ip == boot_code.len())
}

struct Puzzle {
    boot_code: Vec<String>,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            boot_code: Vec::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap_or_else(|_| {
            eprintln!("cannot read input file {path}");
            std::process::exit(1);
        });

        self.boot_code = data.lines().map(std::string::ToString::to_string).collect();
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        run(&self.boot_code).0
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        let mut boot_code = self.boot_code.clone();
        for ip in 0..boot_code.len() {
            let instr = &boot_code[ip].to_string();

            let (op, imm) = instr.split_once(' ').unwrap();

            if op == "acc" {
                continue;
            }

            boot_code[ip] = if op == "nop" {
                format!("jmp {imm}")
            } else {
                format!("nop {imm}")
            };

            let (acc, terminated) = run(&boot_code);
            if terminated {
                return acc;
            }

            boot_code[ip] = instr.to_string();
        }
        0
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
    fn test_part1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 5);
    }

    #[test]
    fn test_part2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 8);
    }
}
