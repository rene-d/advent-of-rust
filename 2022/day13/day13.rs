//! [Day 13: Distress Signal](https://adventofcode.com/2022/day/13)

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser as PestParser;
use std::cmp::Ordering;

#[derive(PestParser, Debug)]
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

struct Puzzle {
    packets: Vec<String>,
}

impl Puzzle {
    fn new() -> Self {
        Self { packets: vec![] }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();
        let lines = data.split('\n').collect::<Vec<_>>();

        for line in &lines {
            if !line.is_empty() {
                self.packets.push((*line).to_string());
            }
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
        a.push("[[2]]".to_string());
        a.push("[[6]]".to_string());

        a.sort_by(|a, b| cmp(a.as_str(), b.as_str()));

        let mut result = 1;
        for (i, s) in a.iter().enumerate() {
            if s == "[[2]]" || s == "[[6]]" {
                result *= i + 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 13);
        assert_eq!(puzzle.part2(), 140);
    }
}

/// main function
fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.path);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
