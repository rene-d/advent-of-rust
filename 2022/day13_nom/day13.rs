//! [Day 13: Distress Signal](https://adventofcode.com/2022/day/13)

use nom::{
    branch::alt, bytes::complete::tag, character::complete::i32, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};
use std::cmp::Ordering;

/// A signal packet, as described in the puzzle
#[derive(Debug, Clone)]
enum Packet {
    Integer(i32),
    Array(Vec<Packet>),
}

impl Packet {
    fn new(input: &str) -> IResult<&str, Self> {
        alt((
            map(i32, Self::Integer),
            map(
                delimited(
                    tag("["),                             // start of array
                    separated_list0(tag(","), Self::new), // array element (array or integer)
                    tag("]"),                             // end of array
                ),
                Self::Array,
            ),
        ))(input)
    }

    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => a.cmp(b),
            (Self::Integer(_), Self::Array(_)) => Self::Array(vec![self.clone()]).cmp(other),
            (Self::Array(_), Self::Integer(_)) => self.cmp(&Self::Array(vec![other.clone()])),
            (Self::Array(a), Self::Array(b)) => {
                let mut iter_a = a.iter();
                let mut iter_b = b.iter();
                loop {
                    let value_a = iter_a.next();
                    let value_b = iter_b.next();
                    if value_a.is_none() || value_b.is_none() {
                        break;
                    }
                    let c = value_a.unwrap().cmp(value_b.unwrap());
                    if c != Ordering::Equal {
                        return c;
                    }
                }
                a.len().cmp(&b.len())
            }
        }
    }
}

struct Puzzle {
    packets: Vec<Packet>,
}

impl Puzzle {
    const fn new() -> Self {
        Self { packets: vec![] }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, data: &str) {
        let lines = data.split('\n').collect::<Vec<_>>();

        for line in lines {
            if !line.is_empty() {
                let packet = Packet::new(line).unwrap().1;
                self.packets.push(packet);
            }
        }
    }

    // Solve part one
    fn part1(&self) -> usize {
        let mut result = 0;
        for (i, p) in self.packets.chunks(2).enumerate() {
            if p[0].cmp(&p[1]) == Ordering::Less {
                result += i + 1;
            }
        }
        result
    }

    // Solve part two
    fn part2(&self) -> usize {
        let mut packets = self.packets.clone();
        let divider1 = Packet::new("[[2]]").unwrap().1;
        let divider2 = Packet::new("[[6]]").unwrap().1;

        packets.push(divider1.clone());
        packets.push(divider2.clone());

        packets.sort_by(Packet::cmp);

        let mut result = 1;
        for (i, p) in packets.iter().enumerate() {
            if p.cmp(&divider1) == Ordering::Equal || p.cmp(&divider2) == Ordering::Equal {
                result *= i + 1;
            }
        }
        result
    }
}

/// main function
fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure(&aoc::load_input_data("test.txt"));
    assert_eq!(puzzle.part1(), 13);
    assert_eq!(puzzle.part2(), 140);
}
