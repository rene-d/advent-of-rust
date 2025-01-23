//! [Day 8: I Heard You Like Registers](https://adventofcode.com/2017/day/8)

use rustc_hash::FxHashMap;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "day8.pest"]
struct MyParser;

struct Cpu {
    registers: FxHashMap<String, i32>,
    last_max: i32,  // i.e. part 1
    value_max: i32, // i.e. part 2
}

impl Cpu {
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
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (i32, i32) {
    let mut cpu = Cpu {
        registers: FxHashMap::default(),
        last_max: 0,
        value_max: 0,
    };

    for line in data.lines() {
        cpu.parse_line(line);
    }

    (cpu.last_max, cpu.value_max)
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
    fn test_solve() {
        let answer = solve(TEST_INPUT);
        assert_eq!(answer.0, 1);
        assert_eq!(answer.1, 10);
    }
}
