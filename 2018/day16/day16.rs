//! [Day 16: Chronal Classification](https://adventofcode.com/2018/day/16)

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const OPCODES: [&str; 16] = [
    "addi", "addr", "bani", "banr", "bori", "borr", "eqir", "eqri", "eqrr", "gtir", "gtri", "gtrr",
    "muli", "mulr", "seti", "setr",
];

fn emulate(opcode: &str, a: u32, b: u32, c: u32, regs: &[u32]) -> Vec<u32> {
    let mut output = regs.to_vec();

    output[c as usize] = match opcode {
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

    output
}

struct Puzzle {
    result1: u32,
    result2: u32,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            result1: 0,
            result2: 0,
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        let (samples, program) = data.split_once("\n\n\n\n").unwrap();

        // the mapping between opcode numebrs and labels
        let mut mapping: HashMap<u32, HashSet<&str>> = HashMap::new();

        // parse the CPU monitoring
        let parse_line = |line: &str, prefix: &str| -> Vec<u32> {
            line.strip_prefix(prefix)
                .unwrap()
                .strip_suffix(']')
                .unwrap()
                .split(',')
                .map(str::trim)
                .map(|x| x.parse().unwrap())
                .collect()
        };

        // parse the samples
        self.result1 = 0;

        for sample in samples.split("\n\n") {
            let sample: Vec<_> = sample.lines().collect();

            let before = parse_line(sample[0], "Before: [");
            let after = parse_line(sample[2], "After:  [");

            let instruction: Vec<_> = sample[1]
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            let (o, a, b, c) = instruction.iter().copied().collect_tuple().unwrap();

            // find opcodes that transform 'before' to 'after'
            let matches = OPCODES
                .iter()
                .filter(|opcode| emulate(opcode, a, b, c, &before) == after)
                .map(|opcode| {
                    mapping.entry(o).or_default().insert(opcode);
                })
                .count();

            // compute part 1
            if matches >= 3 {
                self.result1 += 1;
            }
        }

        // resolve the mapping
        let mut opcodes = HashMap::new();

        while !mapping.is_empty() {
            // find the opcode with one label exactly (or panic, but puzzle inputs are built to prevent this)
            let (opcode, labels) = mapping
                .iter()
                .find(|(_, labels)| labels.len() == 1)
                .unwrap();

            // dereference the found opcode/label
            let opcode = *opcode;
            let label = *labels.iter().nth(0).unwrap();

            // remove it from the mapping
            mapping.remove(&opcode);
            for i in mapping.values_mut() {
                i.remove(label);
            }

            // and insert it into the opcode resolver
            opcodes.insert(opcode, label);
        }

        // run the test program
        let mut regs = vec![0; 4];

        for instruction in program.lines() {
            let instruction: Vec<_> = instruction
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            let (o, a, b, c) = instruction.iter().copied().collect_tuple().unwrap();

            regs = emulate(opcodes.get(&o).unwrap(), a, b, c, &regs);
        }

        // part 2 is the content of register 0
        self.result2 = regs[0];
    }

    /// Solve part one.
    const fn part1(&self) -> u32 {
        self.result1
    }

    /// Solve part two.
    const fn part2(&self) -> u32 {
        self.result2
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
