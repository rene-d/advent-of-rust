//! [Day 23: Coprocessor Conflagration](https://adventofcode.com/2017/day/23)

use num::integer::Roots;
use rustc_hash::FxHashMap;

struct Program {
    opcodes: Vec<(String, Vec<String>)>,
    regs: FxHashMap<String, i64>,
    ip: usize,
    mul_called: u32,
}

impl Program {
    fn new(opcodes: &[(String, Vec<String>)]) -> Self {
        Self {
            opcodes: opcodes.to_vec(),
            regs: FxHashMap::default(),
            ip: 0,
            mul_called: 0,
        }
    }
    fn run(&mut self) {
        while self.ip < self.opcodes.len() {
            let (instr, args) = &self.opcodes[self.ip];
            self.ip += 1;

            let mut value = |i: usize| {
                let a = &args[i];
                if let Ok(v) = a.parse::<i64>() {
                    v
                } else {
                    *self.regs.entry(a.clone()).or_insert(0i64)
                }
            };

            // println!("{}> {instr} {args:?}", self.ip);

            match instr.as_str() {
                "set" => {
                    let v = value(1);
                    self.regs.insert(args[0].clone(), v);
                }
                "sub" => {
                    *self.regs.entry(args[0].clone()).or_insert(0) -= value(1);
                }
                "mul" => {
                    self.mul_called += 1;
                    *self.regs.entry(args[0].clone()).or_insert(0) *= value(1);
                }

                "jnz" => {
                    if value(0) != 0 {
                        let offset: isize = (value(1) - 1).try_into().unwrap();
                        self.ip = self.ip.wrapping_add_signed(offset);
                    }
                }
                "hcf" => {
                    return;
                }
                _ => panic!("unknown instr {instr}"),
            }
        }
    }
}

struct Puzzle {
    program: Vec<(String, Vec<String>)>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            program: data
                .lines()
                .filter_map(|s| s.split_once(' '))
                .map(|(a, b)| {
                    (
                        a.to_string(),
                        b.split_ascii_whitespace()
                            .map(String::from)
                            .collect::<Vec<_>>(),
                    )
                })
                .collect(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut p = Program::new(&self.program);

        p.run();

        p.mul_called
    }

    /// Solve part two.
    fn part2(&self) -> i64 {
        let mut p = Program::new(&self.program);

        p.regs.insert("a".to_string(), 1);
        p.opcodes[11] = ("hcf".to_string(), vec![]);

        p.run();

        let b = p.regs["b"];
        let c = p.regs["c"];

        // non prime numbers between b and c, 17 by 17
        let mut h = 0;
        for n in (b..=c).step_by(17) {
            if n % 2 == 0 {
                h += 1;
            } else {
                for i in 3..=n.sqrt() {
                    if n % i == 0 {
                        h += 1;
                        break;
                    }
                }
            }
        }

        h
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, i64) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}
