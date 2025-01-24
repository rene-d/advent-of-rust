//! [Day 18: Duet](https://adventofcode.com/2017/day/18)

use rustc_hash::FxHashMap;
use std::collections::VecDeque;

struct Program {
    id: i64,
    opcodes: Vec<(String, Vec<String>)>,
    regs: FxHashMap<String, i64>,
    ip: usize,
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
            regs: FxHashMap::default(),
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
                        let offset: isize = (value(1) - 1).try_into().unwrap();
                        self.ip = self.ip.wrapping_add_signed(offset);
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

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (i64, u32) {
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

    const SAMPLE_1: &str = include_str!("sample_1.txt");
    const SAMPLE_2: &str = include_str!("sample_2.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part1(), 4);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(SAMPLE_2);
        assert_eq!(puzzle.part2(), 3);
    }
}
