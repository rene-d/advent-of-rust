//! [Day 8: Handheld Halting](https://adventofcode.com/2020/day/8)

use rustc_hash::FxHashSet;

fn run(boot_code: &[String]) -> (i32, bool) {
    let mut acc = 0;
    let mut ip = 0;
    let mut seen = FxHashSet::default();

    while ip < boot_code.len() && seen.insert(ip) {
        let instr = &boot_code[ip];
        ip += 1;

        let (op, imm) = instr.split_once(' ').unwrap();

        match op {
            "nop" => {}
            "acc" => {
                acc += imm.parse::<i32>().unwrap();
            }
            "jmp" => {
                ip = ip.wrapping_add_signed(imm.parse::<isize>().unwrap()) - 1;
            }
            _ => panic!("invalid instr {instr}"),
        }
    }

    (acc, ip == boot_code.len())
}

struct Puzzle {
    boot_code: Vec<String>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            boot_code: data.lines().map(std::string::ToString::to_string).collect(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        run(&self.boot_code).0
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        let mut boot_code = self.boot_code.clone();
        for ip in 0..boot_code.len() {
            let instr = &boot_code[ip].clone();

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

            boot_code[ip].clone_from(instr);
        }
        0
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (i32, i32) {
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
    fn test_part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 5);
    }

    #[test]
    fn test_part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 8);
    }
}
