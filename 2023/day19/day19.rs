//! [Day 19: Aplenty](https://adventofcode.com/2023/day/19)

#![allow(clippy::too_many_lines)]
// #![allow(clippy::option_if_let_else)]

use core::panic;
use regex::Regex;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

#[derive(Debug)]
enum Comparison {
    Lesser,
    Greater,
    LesserOrEqual,
    GreaterOrEqual,
}

impl Comparison {
    fn from(s: &str) -> Self {
        match s {
            ">" => Self::Greater,
            "<" => Self::Lesser,
            _ => panic!(),
        }
    }

    fn opposite(&self) -> Self {
        match &self {
            Self::Lesser => Self::GreaterOrEqual,
            Self::Greater => Self::LesserOrEqual,
            _ => panic!("unused"),
        }
    }
}

#[derive(Debug)]
enum Rule {
    Link(String),
    Condition((String, Comparison, u64, String)),
}

struct Puzzle {
    workflows: FxHashMap<String, Vec<Rule>>,
    parts: Vec<[u64; 4]>,
}

fn new_range(op: &Comparison, n: u64, mut lo: u64, mut hi: u64) -> (u64, u64) {
    match op {
        Comparison::Greater => lo = (n + 1).max(lo),
        Comparison::Lesser => hi = (n - 1).min(hi),
        Comparison::GreaterOrEqual => lo = n.max(lo),
        Comparison::LesserOrEqual => hi = n.min(hi),
    };
    (lo, hi)
}

impl Puzzle {
    fn new() -> Self {
        Self {
            workflows: FxHashMap::default(),
            parts: vec![],
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        //
        // _   _ _____  _   __   __  _
        // | | | |  __ \| |  \ \ / / | |
        // | | | | |  \/| |   \ V /  | |
        // | | | | | __ | |    \ /   | |
        // | |_| | |_\ \| |____| |   |_|
        //  \___/ \____/\_____/\_/   (_)
        //

        let re = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();

        let re2 = Regex::new(r"(\w+)\{(.+)\}$").unwrap();

        let re3: Regex = Regex::new(r"^([xmas])([<>])(\d+):(\w+)$").unwrap();

        let rule_new = |s: &str| match s {
            s if s.chars().all(char::is_alphabetic) => Rule::Link(s.to_string()),

            _ => re3.captures(s).map_or_else(
                || {
                    panic!();
                },
                |caps| {
                    let variable = caps.get(1).unwrap().as_str().to_owned();
                    let op = Comparison::from(caps.get(2).unwrap().as_str());
                    let value: u64 = caps.get(3).unwrap().as_str().parse().unwrap();
                    let next = caps.get(4).unwrap().as_str().to_owned();

                    Rule::Condition((variable, op, value, next))
                },
            ),
        };

        for line in data.lines() {
            if let Some(caps) = re.captures(line) {
                let caps: [u64; 4] = caps
                    .iter()
                    .skip(1)
                    .map(|cap| cap.unwrap().as_str().parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
                    .as_slice()
                    .try_into()
                    .unwrap();

                self.parts.push(caps);
            } else if let Some(caps) = re2.captures(line) {
                let name = caps.get(1).unwrap().as_str().to_string();
                let rules: Vec<Rule> = caps
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(rule_new)
                    .collect();

                if let Some(Rule::Link(_)) = rules.last() {
                } else {
                    panic!("rule '{line}' must end with a link");
                }

                self.workflows.insert(name, rules);
            }
        }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        let mut accepted: u64 = 0;

        for xmas in &self.parts {
            let mut workflow = "in";

            loop {
                let rules = &self.workflows[workflow];

                workflow = "";

                for rule in rules {
                    match rule {
                        Rule::Link(link) => {
                            workflow = link;
                            break;
                        }

                        Rule::Condition((part, op, value, next)) => {
                            let part = match part.as_str() {
                                "x" => xmas[0],
                                "m" => xmas[1],
                                "a" => xmas[2],
                                "s" => xmas[3],
                                _ => unreachable!(),
                            };

                            let done = match op {
                                Comparison::Lesser => part < *value,
                                Comparison::Greater => part > *value,
                                _ => unreachable!(),
                            };

                            if done {
                                workflow = next;
                                break;
                            }
                        }
                    };
                }

                match workflow {
                    "" => panic!("no match"),
                    "A" => {
                        accepted += xmas.iter().sum::<u64>();
                        break;
                    }
                    "R" => break,
                    _ => (),
                }
            }
        }
        accepted
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        let mut accepted = 0;
        let mut q = VecDeque::new();

        q.push_back(("in".to_string(), 1, 4000, 1, 4000, 1, 4000, 1, 4000));

        while let Some((
            workflow,
            mut x_lo,
            mut x_hi,
            mut m_lo,
            mut m_hi,
            mut a_lo,
            mut a_hi,
            mut s_lo,
            mut s_hi,
        )) = q.pop_back()
        {
            if x_lo > x_hi || m_lo > m_hi || a_lo > a_hi || s_lo > s_hi {
                continue;
            }

            if workflow == "A" {
                accepted +=
                    (x_hi - x_lo + 1) * (m_hi - m_lo + 1) * (a_hi - a_lo + 1) * (s_hi - s_lo + 1);
                continue;
            }

            if workflow == "R" {
                continue;
            }

            for rule in &self.workflows[&workflow] {
                match rule {
                    Rule::Link(name) => {
                        q.push_back((name.clone(), x_lo, x_hi, m_lo, m_hi, a_lo, a_hi, s_lo, s_hi));
                    }

                    Rule::Condition((variable, op, value, link)) => {
                        match variable.as_str() {
                            "x" => {
                                let (new_x1, new_x2) = new_range(op, *value, x_lo, x_hi);
                                q.push_back((
                                    link.clone(),
                                    new_x1,
                                    new_x2,
                                    m_lo,
                                    m_hi,
                                    a_lo,
                                    a_hi,
                                    s_lo,
                                    s_hi,
                                ));
                                (x_lo, x_hi) = new_range(&op.opposite(), *value, x_lo, x_hi);
                            }

                            "m" => {
                                let (new_m_lo, new_m_hi) = new_range(op, *value, m_lo, m_hi);
                                q.push_back((
                                    link.clone(),
                                    x_lo,
                                    x_hi,
                                    new_m_lo,
                                    new_m_hi,
                                    a_lo,
                                    a_hi,
                                    s_lo,
                                    s_hi,
                                ));
                                (m_lo, m_hi) = new_range(&op.opposite(), *value, m_lo, m_hi);
                            }

                            "a" => {
                                let (new_a_lo, new_a_hi) = new_range(op, *value, a_lo, a_hi);
                                q.push_back((
                                    link.clone(),
                                    x_lo,
                                    x_hi,
                                    m_lo,
                                    m_hi,
                                    new_a_lo,
                                    new_a_hi,
                                    s_lo,
                                    s_hi,
                                ));
                                (a_lo, a_hi) = new_range(&op.opposite(), *value, a_lo, a_hi);
                            }

                            "s" => {
                                let (new_s_lo, new_s_hi) = new_range(op, *value, s_lo, s_hi);
                                q.push_back((
                                    link.clone(),
                                    x_lo,
                                    x_hi,
                                    m_lo,
                                    m_hi,
                                    a_lo,
                                    a_hi,
                                    new_s_lo,
                                    new_s_hi,
                                ));
                                (s_lo, s_hi) = new_range(&op.opposite(), *value, s_lo, s_hi);
                            }

                            _ => panic!(),
                        };
                    }
                }
            }

            continue;
        }
        accepted
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
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
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part1(), 19114);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part2(), 167_409_079_868_000);
    }
}
