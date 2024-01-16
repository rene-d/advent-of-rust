//! [Day 21: Chronal Conversion](https://adventofcode.com/2018/day/21)

use std::collections::HashSet;

const OPCODES: [&str; 16] = [
    "addi", "addr", "bani", "banr", "bori", "borr", "eqir", "eqri", "eqrr", "gtir", "gtri", "gtrr",
    "muli", "mulr", "seti", "setr",
];

fn emulate(opcode: &'static str, a: u64, b: u64, c: u64, regs: &mut [u64]) {
    regs[c as usize] = match opcode {
        "addr" => regs[a as usize] + regs[b as usize],
        "addi" => regs[a as usize] + b,
        "mulr" => regs[a as usize] * regs[b as usize],
        "muli" => regs[a as usize] * b,
        "banr" => regs[a as usize] & regs[b as usize],
        "bani" => regs[a as usize] & b,
        "borr" => regs[a as usize] | regs[b as usize],
        "bori" => regs[a as usize] | b,
        "setr" => regs[a as usize],
        "seti" => a,
        "gtir" => u64::from(a > regs[b as usize]),
        "gtri" => u64::from(regs[a as usize] > b),
        "gtrr" => u64::from(regs[a as usize] > regs[b as usize]),
        "eqir" => u64::from(a == regs[b as usize]),
        "eqri" => u64::from(regs[a as usize] == b),
        "eqrr" => u64::from(regs[a as usize] == regs[b as usize]),
        _ => panic!("bad opcode {opcode}"),
    };
}

#[derive(Debug)]
struct Instr {
    opcode: &'static str,
    a: u64,
    b: u64,
    c: u64,
}

impl Instr {
    fn run(&self, regs: &mut [u64]) {
        emulate(self.opcode, self.a, self.b, self.c, regs);
    }
}

impl std::fmt::Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", self.opcode, self.a, self.b, self.c)
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
                let line: Vec<_> = line.split_ascii_whitespace().collect();

                let opcode = OPCODES.iter().find(|&&opcode| opcode == line[0]).unwrap();
                let a = line[1].parse::<u64>().unwrap();
                let b = line[2].parse::<u64>().unwrap();
                let c = line[3].parse::<u64>().unwrap();

                let instr = Instr { opcode, a, b, c };

                self.program.push(instr);
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
