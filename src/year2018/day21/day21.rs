//! [Day 21: Chronal Conversion](https://adventofcode.com/2018/day/21)

use rustc_hash::FxHashSet;
use std::fmt::Error;

fn us(a: u64) -> usize {
    usize::try_from(a).unwrap()
}

#[derive(Debug, PartialEq)]
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
            "addr" => Self::Addr,
            "addi" => Self::Addi,
            "mulr" => Self::Mulr,
            "muli" => Self::Muli,
            "banr" => Self::Banr,
            "bani" => Self::Bani,
            "borr" => Self::Borr,
            "bori" => Self::Bori,
            "setr" => Self::Setr,
            "seti" => Self::Seti,
            "gtir" => Self::Gtir,
            "gtri" => Self::Gtri,
            "gtrr" => Self::Gtrr,
            "eqir" => Self::Eqir,
            "eqri" => Self::Eqri,
            "eqrr" => Self::Eqrr,
            _ => panic!("unknown opcode {s}"),
        };

        Ok(opcode)
    }
}

impl OpCodes {
    fn emulate(&self, a: u64, b: u64, c: u64, regs: &mut [u64]) {
        regs[us(c)] = match &self {
            Self::Addr => regs[us(a)] + regs[us(b)],
            Self::Addi => regs[us(a)] + b,
            Self::Mulr => regs[us(a)] * regs[us(b)],
            Self::Muli => regs[us(a)] * b,
            Self::Banr => regs[us(a)] & regs[us(b)],
            Self::Bani => regs[us(a)] & b,
            Self::Borr => regs[us(a)] | regs[us(b)],
            Self::Bori => regs[us(a)] | b,
            Self::Setr => regs[us(a)],
            Self::Seti => a,
            Self::Gtir => u64::from(a > regs[us(b)]),
            Self::Gtri => u64::from(regs[us(a)] > b),
            Self::Gtrr => u64::from(regs[us(a)] > regs[us(b)]),
            Self::Eqir => u64::from(a == regs[us(b)]),
            Self::Eqri => u64::from(regs[us(a)] == b),
            Self::Eqrr => u64::from(regs[us(a)] == regs[us(b)]),
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

        Ok(Self { opcode, a, b, c })
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
    /// Get the puzzle input.
    fn new(data: &str) -> Self {
        let mut program = Vec::new();
        let mut ip_reg = 0;

        for line in data.lines() {
            if let Some(value) = line.strip_prefix("#ip ") {
                ip_reg = value.parse::<usize>().unwrap();
            } else {
                program.push(line.parse().unwrap());
            }
        }

        Self { ip_reg, program }
    }

    fn run(&self, mut iterations: u32) {
        let mut regs = [0, 0, 0, 0, 0, 0];

        loop {
            let ip = us(regs[self.ip_reg]);

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
            let ip = us(regs[self.ip_reg]);

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
        let mut seen = FxHashSet::default();
        let mut last = 0;

        loop {
            let ip = us(regs[self.ip_reg]);

            if ip == 28 {
                let m = *regs.iter().max().unwrap();
                if !seen.insert(m) {
                    return last;
                }
                last = m;
            }

            assert!(ip < self.program.len());

            self.program[ip].run(&mut regs);
            regs[self.ip_reg] += 1;
        }
    }

    fn part2_fast(&self) -> u64 {
        if self.program[7].opcode != OpCodes::Seti {
            return self.part2();
        }
        let a = self.program[7].a;

        let run = |hash: u64| -> u64 {
            let mut acc = a;
            let mut b = hash | 65536;

            for _ in 0..3 {
                acc += b & 255;
                acc &= 16_777_215;
                acc *= 65899;
                acc &= 16_777_215;
                b >>= 8;
            }

            acc
        };

        let mut seen = FxHashSet::default();
        let mut last = 0;
        let mut hash = 0;

        while seen.insert(hash) {
            last = hash;
            hash = run(hash);
        }
        last
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u64, u64) {
    let puzzle: Puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2_fast())
}

pub fn main() {
    let args = aoc::parse_args();

    if args.has_option("--emulate") {
        Puzzle::new(args.input()).run(1);
        std::process::exit(0)
    }

    args.run(solve);
}
