//! [Day 18: Duet](https://adventofcode.com/2017/day/18)

use std::collections::{HashMap, VecDeque};

struct Program {
    id: i64,
    opcodes: Vec<(String, Vec<String>)>,
    regs: HashMap<String, i64>,
    ip: i64,
    terminated: bool,
    messages: VecDeque<i64>,
    mode_sound: bool,
    last_sound: i64,
}

impl Program {
    fn new(id: i64, opcodes: &[(String, Vec<String>)]) -> Self {
        let mut o = Self {
            id,
            opcodes: opcodes.to_vec(),
            regs: HashMap::new(),
            ip: 0,
            terminated: false,
            messages: VecDeque::new(),
            mode_sound: false,
            last_sound: 0,
        };
        o.regs.insert("p".to_string(), o.id);
        o
    }
    fn enqueue(&mut self, v: i64) {
        self.messages.push_back(v);
    }

    fn run(&mut self) -> Option<i64> {
        while self.ip >= 0 && self.ip < (self.opcodes.len() as i64) {
            let (instr, args) = &self.opcodes[self.ip as usize];
            self.ip += 1;

            let mut value = |i: usize| {
                let a = &args[i];
                if let Ok(v) = a.parse::<i64>() {
                    v
                } else {
                    *self.regs.entry(a.clone()).or_insert(0i64)
                }
            };

            // println!("{}:{}> {instr} {args:?}", self.id, self.ip);

            match instr.as_str() {
                "snd" => {
                    let v = value(0);
                    if self.mode_sound {
                        self.last_sound = v;
                    } else {
                        // send a value
                        return Some(v);
                    }
                }
                "set" => {
                    let v = value(1);
                    self.regs.insert(args[0].clone(), v);
                }
                "add" => {
                    *self.regs.entry(args[0].clone()).or_insert(0) += value(1);
                }
                "mul" => {
                    *self.regs.entry(args[0].clone()).or_insert(0) *= value(1);
                }
                "mod" => {
                    *self.regs.entry(args[0].clone()).or_insert(0) %= value(1);
                }
                "rcv" => {
                    if self.mode_sound {
                        if value(0) != 0 {
                            return Some(self.last_sound);
                        }
                    } else if let Some(v) = self.messages.pop_front() {
                        *self.regs.entry(args[0].clone()).or_insert(0) = v;
                    } else {
                        // waiting for a value
                        self.ip -= 1; // return to the rcv instruction
                        return None;
                    }
                }
                "jgz" => {
                    if value(0) > 0 {
                        self.ip += value(1) - 1;
                    }
                }
                _ => panic!("unknown instr {instr}"),
            }
        }

        // println!("{} terminated", self.id);
        self.terminated = true;

        None
    }
}

struct Puzzle {
    program: Vec<(String, Vec<String>)>,
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
            .filter_map(|s| s.split_once(' '))
            .map(|(a, b)| {
                (
                    a.to_string(),
                    b.split_ascii_whitespace()
                        .map(String::from)
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
    }

    /// Solve part one.
    fn part1(&self) -> i64 {
        let mut p = Program::new(0, &self.program);

        p.mode_sound = true;

        p.run().unwrap_or(0)
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let mut p0 = Program::new(0, &self.program);
        let mut p1 = Program::new(1, &self.program);

        let mut values_sent = 0;
        let mut deadlock = false;

        while !p0.terminated && !p1.terminated && !deadlock {
            deadlock = true;

            if let Some(v) = p0.run() {
                p1.enqueue(v);
                deadlock = false;
            }

            if let Some(v) = p1.run() {
                values_sent += 1;
                p0.enqueue(v);
                deadlock = false;
            }
        }

        // if (deadlock) {
        //     println!("deadlock");
        // }

        values_sent
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
        puzzle.configure("sample_1.txt");
        assert_eq!(puzzle.part1(), 4);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_2.txt");
        assert_eq!(puzzle.part2(), 3);
    }
}
