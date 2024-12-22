//! [Day 8: I Heard You Like Registers](https://adventofcode.com/2017/day/8)

use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser as PestParser;

#[derive(PestParser, Debug)]
#[grammar = "day8.pest"]
struct MyParser;

struct Puzzle {
    registers: HashMap<String, i32>,

    last_max: i32,  // i.e. part 1
    value_max: i32, // i.e. part 2
}

impl Puzzle {
    fn new() -> Self {
        Self {
            registers: HashMap::new(),
            last_max: 0,
            value_max: 0,
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.parse(&data);
    }

    /// Parse the input.
    fn parse_line(&mut self, line: &str) {
        let mut a = MyParser::parse(Rule::instr, line)
            .unwrap()
            .next()
            .unwrap()
            .into_inner();

        let target = a.next().unwrap().as_str();
        let oper = a.next().unwrap().as_rule();
        let value = a.next().unwrap().as_str().parse::<i32>().unwrap();
        let lhs = a.next().unwrap().as_str();
        let cmp = a.next().unwrap().as_rule();
        let rhs = a.next().unwrap().as_str().parse::<i32>().unwrap();

        let lhs = self.registers.get(lhs).unwrap_or(&0);

        let ok = match cmp {
            Rule::equal => lhs == &rhs,
            Rule::different => lhs != &rhs,
            Rule::greater => lhs > &rhs,
            Rule::greater_or_equal => lhs >= &rhs,
            Rule::less => lhs < &rhs,
            Rule::less_or_equal => lhs <= &rhs,
            _ => panic!(),
        };

        if ok {
            let r = self.registers.entry(target.to_string()).or_insert(0);

            match oper {
                Rule::inc => *r += value,
                Rule::dec => *r -= value,
                _ => panic!(),
            }

            let max = *self.registers.values().max().unwrap();
            self.last_max = max;
            self.value_max = self.value_max.max(self.last_max);
        }
    }

    fn parse(&mut self, data: &str) {
        for line in data.lines() {
            self.parse_line(line);
        }
    }

    /// Solve part one.
    const fn part1(&self) -> i32 {
        self.last_max
    }

    /// Solve part two.
    const fn part2(&self) -> i32 {
        self.value_max
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.path);
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
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 1);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 10);
    }
}
