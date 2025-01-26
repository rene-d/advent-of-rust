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
    fn new(data: &str) -> Self {
        let mut ip_reg = 0;
        let mut program = vec![];

        for line in data.lines() {
            if let Some(value) = line.strip_prefix("#ip ") {
                ip_reg = value.parse::<usize>().unwrap();
            } else {
                let line: Vec<_> = line.split_ascii_whitespace().collect();

                let opcode = OPCODES.iter().find(|&&opcode| opcode == line[0]).unwrap();
                let a = line[1].parse::<u32>().unwrap();
                let b = line[2].parse::<u32>().unwrap();
                let c = line[3].parse::<u32>().unwrap();

                let instr = Instr { opcode, a, b, c };

                program.push(instr);
            }
        }

        Self { ip_reg, program }
    }

    fn solve_optimized(&self, r0: u32) -> u32 {
        let mut regs = [r0, 0, 0, 0, 0, 0];

        loop {
            let ip = regs[self.ip_reg] as usize;

            if ip == 1 {
                break;
            }

            assert!(ip < self.program.len());

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

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();

    if args.is_verbose() {
        println!("{}", Puzzle::new(args.input()).run(0));
        return;
    }

    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_1: &str = include_str!("sample_1.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.run(0), 7);
    }
}
