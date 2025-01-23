//! [Day 13: Distress Signal](https://adventofcode.com/2022/day/13)

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::cmp::Ordering;

#[derive(Parser)]
#[grammar = "day13.pest"]
struct PacketParser;

fn cmp(a: &str, b: &str) -> Ordering {
    fn inner_cmp(a: Pair<Rule>, b: Pair<Rule>) -> Ordering {
        match (a.as_rule(), b.as_rule()) {
            (Rule::integer, Rule::integer) => {
                let na = a.as_str().parse::<i32>().unwrap();
                let nb = b.as_str().parse::<i32>().unwrap();
                na.cmp(&nb)
            }
            (Rule::array, Rule::integer) => {
                let right = "[".to_string() + b.as_str() + "]";
                let b = PacketParser::parse(Rule::value, right.as_str())
                    .unwrap()
                    .next()
                    .unwrap();
                inner_cmp(a, b)
            }
            (Rule::integer, Rule::array) => {
                let left = "[".to_string() + a.as_str() + "]";
                let a = PacketParser::parse(Rule::value, left.as_str())
                    .unwrap()
                    .next()
                    .unwrap();
                inner_cmp(a, b)
            }
            (Rule::array, Rule::array) => {
                let mut iter_a = a.into_inner();
                let mut iter_b = b.into_inner();

                loop {
                    let value_a = iter_a.next();
                    let value_b = iter_b.next();

                    if value_a.is_none() && value_b.is_some() {
                        return Ordering::Less;
                    }
                    if value_a.is_some() && value_b.is_none() {
                        return Ordering::Greater;
                    }
                    if value_a.is_none() && value_b.is_none() {
                        break;
                    }
                    let c = inner_cmp(value_a.unwrap(), value_b.unwrap());
                    if c.is_ne() {
                        return c;
                    }
                }
                Ordering::Equal
            }
            _ => panic!("unknown rule pair"),
        }
    }

    let a = PacketParser::parse(Rule::value, a).unwrap().next().unwrap();
    let b = PacketParser::parse(Rule::value, b).unwrap().next().unwrap();
    inner_cmp(a, b)
}

struct Puzzle<'a> {
    packets: Vec<&'a str>,
}

impl<'a> Puzzle<'a> {
    fn new(data: &'a str) -> Self {
        Self {
            packets: data.lines().filter(|line| !line.is_empty()).collect(),
        }
    }

    // Solves part one
    fn part1(&self) -> usize {
        let mut result = 0;
        for (i, chunk) in self.packets.chunks(2).enumerate() {
            let left = &chunk[0];
            let right = &chunk[1];
            if cmp(left, right).is_lt() {
                result += i + 1;
            }
        }
        result
    }

    // Solve part two
    fn part2(&self) -> usize {
        let mut a = self.packets.clone();
        a.push("[[2]]");
        a.push("[[6]]");

        a.sort_by(|a, b| cmp(a, b));

        let mut result = 1;
        for (i, &s) in a.iter().enumerate() {
            if s == "[[2]]" || s == "[[6]]" {
                result *= i + 1;
            }
        }
        result
    }
}

#[must_use]
pub fn solve(data: &str) -> (usize, usize) {
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
        assert_eq!(puzzle.part1(), 13);
        assert_eq!(puzzle.part2(), 140);
    }
}
