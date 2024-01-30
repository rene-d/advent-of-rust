//! [Day 21: Chronal Conversion](https://adventofcode.com/2018/day/21)

#![allow(clippy::cast_possible_truncation)]

use std::collections::HashSet;
use std::fmt::Error;

#[derive(Debug)]
enum OpCodes {
    Addi,
    Addr,
    Bani,
    Banr,
    Bori,
    Borr,
    Eqir,
    Eqri,
    Eqrr,
    Gtir,
    Gtri,
    Gtrr,
    Muli,
    Mulr,
    Seti,
    Setr,
}

impl std::str::FromStr for OpCodes {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let opcode = match s {
            "addr" => OpCodes::Addr,
            "addi" => OpCodes::Addi,
            "mulr" => OpCodes::Mulr,
            "muli" => OpCodes::Muli,
            "banr" => OpCodes::Banr,
            "bani" => OpCodes::Bani,
            "borr" => OpCodes::Borr,
            "bori" => OpCodes::Bori,
            "setr" => OpCodes::Setr,
            "seti" => OpCodes::Seti,
            "gtir" => OpCodes::Gtir,
            "gtri" => OpCodes::Gtri,
            "gtrr" => OpCodes::Gtrr,
            "eqir" => OpCodes::Eqir,
            "eqri" => OpCodes::Eqri,
            "eqrr" => OpCodes::Eqrr,
            _ => panic!("unknown opcode {s}"),
        };

        Ok(opcode)
    }
}

impl OpCodes {
    fn emulate(&self, a: u64, b: u64, c: u64, regs: &mut [u64]) {
        regs[c as usize] = match &self {
            OpCodes::Addr => regs[a as usize] + regs[b as usize],
            OpCodes::Addi => regs[a as usize] + b,
            OpCodes::Mulr => regs[a as usize] * regs[b as usize],
            OpCodes::Muli => regs[a as usize] * b,
            OpCodes::Banr => regs[a as usize] & regs[b as usize],
            OpCodes::Bani => regs[a as usize] & b,
            OpCodes::Borr => regs[a as usize] | regs[b as usize],
            OpCodes::Bori => regs[a as usize] | b,
            OpCodes::Setr => regs[a as usize],
            OpCodes::Seti => a,
            OpCodes::Gtir => u64::from(a > regs[b as usize]),
            OpCodes::Gtri => u64::from(regs[a as usize] > b),
            OpCodes::Gtrr => u64::from(regs[a as usize] > regs[b as usize]),
            OpCodes::Eqir => u64::from(a == regs[b as usize]),
            OpCodes::Eqri => u64::from(regs[a as usize] == b),
            OpCodes::Eqrr => u64::from(regs[a as usize] == regs[b as usize]),
        };
    }
}

struct Instr {
    opcode: OpCodes,
    a: u64,
    b: u64,
    c: u64,
}

impl Instr {
    fn run(&self, regs: &mut [u64]) {
        self.opcode.emulate(self.a, self.b, self.c, regs);
    }
}

impl std::str::FromStr for Instr {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<_> = s.split_ascii_whitespace().collect();

        let opcode = s[0].parse().unwrap();
        let a = s[1].parse().unwrap();
        let b = s[2].parse().unwrap();
        let c = s[3].parse().unwrap();

        Ok(Instr { opcode, a, b, c })
    }
}

impl std::fmt::Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {} {}", self.opcode, self.a, self.b, self.c)
    }
}

type Program = Vec<Instr>;

struct Puzzle {
    ip_reg: usize,
    program: Program,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            ip_reg: 0,
            program: vec![],
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            if let Some(value) = line.strip_prefix("#ip ") {
                self.ip_reg = value.parse::<usize>().unwrap();
            } else {
                self.program.push(line.parse().unwrap());
            }
        }
    }

    fn run(&self, mut iterations: u32) {
        let mut regs = [0, 0, 0, 0, 0, 0];

        loop {
            let ip = regs[self.ip_reg] as usize;

            if ip >= self.program.len() {
                break;
            }

            let instr: &Instr = &self.program[ip];

            println!("ip={ip:2}  {regs:5?}  {instr}");

            instr.run(&mut regs);

            if ip == 28 {
                if iterations <= 1 {
                    return;
                }
                iterations -= 1;
            }

            regs[self.ip_reg] += 1;
        }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        let mut regs = [0, 0, 0, 0, 0, 0];

        loop {
            let ip = regs[self.ip_reg] as usize;

            if ip == 28 {
                return *regs.iter().max().unwrap();
            }

            assert!(ip < self.program.len());

            self.program[ip].run(&mut regs);
            regs[self.ip_reg] += 1;
        }
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        let mut regs = [0, 0, 0, 0, 0, 0];
        let mut seen = HashSet::new();
        let mut last = 0;

        loop {
            let ip = regs[self.ip_reg] as usize;

            if ip == 28 {
                let m = *regs.iter().max().unwrap();
                if seen.contains(&m) {
                    return last;
                }
                last = m;
                seen.insert(last);
            }

            assert!(ip < self.program.len());

            self.program[ip].run(&mut regs);
            regs[self.ip_reg] += 1;
        }
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    if args.verbose {
        puzzle.run(1);
    } else {
        println!("{}", puzzle.part1());
        println!("{}", puzzle.part2());
    }
}
