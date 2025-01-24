//! [Day 19: Aplenty](https://adventofcode.com/2023/day/19)

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

fn new_range(op: &Comparison, n: u64, rating: (u64, u64)) -> (u64, u64) {
    let (mut lo, mut hi) = rating;
    match op {
        Comparison::Greater => lo = (n + 1).max(lo),
        Comparison::Lesser => hi = (n - 1).min(hi),
        Comparison::GreaterOrEqual => lo = n.max(lo),
        Comparison::LesserOrEqual => hi = n.min(hi),
    };
    (lo, hi)
}

#[derive(Clone)]
struct Ratings {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

impl Ratings {
    const fn new() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    const fn is_valid(&self) -> bool {
        self.x.0 <= self.x.1 && self.m.0 <= self.m.1 && self.a.0 <= self.a.1 && self.s.0 <= self.s.1
    }

    const fn product(&self) -> u64 {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }

    fn update_x(&mut self, op: &Comparison, value: u64) -> Self {
        let new_x = new_range(op, value, self.x);
        self.x = new_range(&op.opposite(), value, self.x);
        Self {
            x: new_x,
            m: self.m,
            a: self.a,
            s: self.s,
        }
    }

    fn update_m(&mut self, op: &Comparison, value: u64) -> Self {
        let new_m = new_range(op, value, self.m);
        self.m = new_range(&op.opposite(), value, self.m);
        Self {
            x: self.x,
            m: new_m,
            a: self.a,
            s: self.s,
        }
    }

    fn update_a(&mut self, op: &Comparison, value: u64) -> Self {
        let new_a = new_range(op, value, self.a);
        self.a = new_range(&op.opposite(), value, self.a);
        Self {
            x: self.x,
            m: self.m,
            a: new_a,
            s: self.s,
        }
    }

    fn update_s(&mut self, op: &Comparison, value: u64) -> Self {
        let new_s = new_range(op, value, self.s);
        self.s = new_range(&op.opposite(), value, self.s);
        Self {
            x: self.x,
            m: self.m,
            a: self.a,
            s: new_s,
        }
    }
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut puzzle = Self {
            workflows: FxHashMap::default(),
            parts: vec![],
        };

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

                puzzle.parts.push(caps);
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

                puzzle.workflows.insert(name, rules);
            }
        }

        puzzle
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

        q.push_back(("in".to_string(), Ratings::new()));

        while let Some((workflow, mut ratings)) = q.pop_back() {
            if !ratings.is_valid() {
                continue;
            }

            if workflow == "A" {
                accepted += ratings.product();
                continue;
            }

            if workflow == "R" {
                continue;
            }

            for rule in &self.workflows[&workflow] {
                match rule {
                    Rule::Link(name) => {
                        q.push_back((name.clone(), ratings.clone()));
                    }

                    Rule::Condition((variable, op, value, link)) => {
                        match variable.as_str() {
                            "x" => {
                                q.push_back((link.clone(), ratings.update_x(op, *value)));
                            }

                            "m" => {
                                q.push_back((link.clone(), ratings.update_m(op, *value)));
                            }

                            "a" => {
                                q.push_back((link.clone(), ratings.update_a(op, *value)));
                            }

                            "s" => {
                                q.push_back((link.clone(), ratings.update_s(op, *value)));
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

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u64, u64) {
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

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 19114);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 167_409_079_868_000);
    }
}
