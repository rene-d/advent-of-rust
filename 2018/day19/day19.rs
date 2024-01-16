//! [Day 19: Go With The Flow](https://adventofcode.com/2018/day/19)

const OPCODES: [&str; 16] = [
    "addi", "addr", "bani", "banr", "bori", "borr", "eqir", "eqri", "eqrr", "gtir", "gtri", "gtrr",
    "muli", "mulr", "seti", "setr",
];

fn emulate(opcode: &'static str, a: u32, b: u32, c: u32, regs: &mut [u32]) {
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
        "gtir" => u32::from(a > regs[b as usize]),
        "gtri" => u32::from(regs[a as usize] > b),
        "gtrr" => u32::from(regs[a as usize] > regs[b as usize]),
        "eqir" => u32::from(a == regs[b as usize]),
        "eqri" => u32::from(regs[a as usize] == b),
        "eqrr" => u32::from(regs[a as usize] == regs[b as usize]),
        _ => panic!("bad opcode {opcode}"),
    };
}

#[derive(Debug)]
struct Instr {
    opcode: &'static str,
    a: u32,
    b: u32,
    c: u32,
}

impl Instr {
    fn run(&self, regs: &mut [u32]) {
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
                let a = line[1].parse::<u32>().unwrap();
                let b = line[2].parse::<u32>().unwrap();
                let c = line[3].parse::<u32>().unwrap();

                let instr = Instr { opcode, a, b, c };

                self.program.push(instr);
            }
        }
    }

    fn solve_optimized(&self, r0: u32) -> u32 {
        let mut regs = [r0, 0, 0, 0, 0, 0];

        loop {
            let ip = regs[self.ip_reg] as usize;

            if ip == 1 {
                break;
            }

            if ip >= self.program.len() {
                panic!();
            }

            self.program[ip].run(&mut regs);
            regs[self.ip_reg] += 1;
        }

        let n = *regs.iter().max().unwrap();
        (1..=n).filter(|k| n % k == 0).sum()
    }

    fn run(&self, r0: u32) -> u32 {
        let mut regs = [r0, 0, 0, 0, 0, 0];
        loop {
            let ip = regs[self.ip_reg] as usize;

            if ip >= self.program.len() {
                break;
            }

            let instr: &Instr = &self.program[ip];

            print!("ip={ip:2}  {regs:5?}  {instr}");

            instr.run(&mut regs);

            println!("  {regs:5?}");

            regs[self.ip_reg] += 1;
        }

        regs[0]
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        self.solve_optimized(0)
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        self.solve_optimized(1)
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    if args.verbose {
        println!("{}", puzzle.run(0));
    } else {
        println!("{}", puzzle.part1());
        println!("{}", puzzle.part2());
    }
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_1.txt");
        assert_eq!(puzzle.run(0), 7);
    }
}
