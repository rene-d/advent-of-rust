//! [Day 20: Grove Positioning System](https://adventofcode.com/2022/day/20)

use clap::Parser;
// use regex::Regex;
use std::collections::VecDeque;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Puzzle {
    numbers: Vec<i64>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            numbers: Vec::new(),
        }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.numbers = data
            .split('\n')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<_>().unwrap())
            .collect::<Vec<_>>();
    }

    // Solves part one
    fn part1(&self) -> i64 {
        self.decrypt(1, 1)
    }

    // Solve part two
    fn part2(&self) -> i64 {
        self.decrypt(811_589_153, 10)
    }

    fn decrypt(&self, key: i64, rounds: usize) -> i64 {
        let mut q = VecDeque::new();

        q.extend(self.numbers.iter().map(|x| (*x) * key).enumerate());

        let nb = self.numbers.len();

        for _ in 0..rounds {
            for i in 0..nb {
                let mut shift = (0usize, 0i64);

                while let Some(e) = q.pop_front() {
                    if e.0 == i {
                        shift = e;
                        break;
                    }
                    q.push_back(e);
                }

                match shift.1 {
                    o if o > 0 => q.rotate_left((o as usize) % (nb - 1)),
                    o if o < 0 => q.rotate_right((-o as usize) % (nb - 1)),
                    _ => (),
                }

                q.push_back(shift);
            }
        }

        for (i, v) in q.iter().enumerate() {
            if v.1 == 0 {
                return q.get((i + 1000) % nb).unwrap().1
                    + q.get((i + 2000) % nb).unwrap().1
                    + q.get((i + 3000) % nb).unwrap().1;
            }
        }
        0
    }
}

/// main function
fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.path);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test.txt");
    assert_eq!(puzzle.part1(), 3);
    assert_eq!(puzzle.part2(), 1623178306);
}
