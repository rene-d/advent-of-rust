//! [Day 12: Hot Springs](https://adventofcode.com/2023/day/12)

use std::collections::HashMap;

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

#[derive(Clone)]
struct Row {
    springs: Vec<char>,
    damaged: Vec<u64>,
}

impl Row {
    fn calc(&self) -> u64 {
        self.calc_rec(&mut HashMap::new(), 0, 0, 0)
    }

    fn calc_rec(
        &self,
        seen: &mut HashMap<(u64, usize, usize), u64>,
        damaged: u64, // current number of damaged springs
        si: usize,    // index in spring array
        di: usize,    // index in damages array
    ) -> u64 {
        let key = (damaged, si, di);

        if let Some(&v) = seen.get(&key) {
            return v;
        }

        if si == self.springs.len() {
            if (di == self.damaged.len() && damaged == 0)
                || (di == self.damaged.len() - 1 && self.damaged[di] == damaged)
            {
                // we have found an arrangement
                return 1;
            }
            // something doesn't match
            return 0;
        }

        let mut result = 0;

        let spring = self.springs[si];

        if spring == '.' || spring == '?' {
            // current spring is operational, or supposed to be
            if damaged == 0 {
                result += self.calc_rec(seen, 0, si + 1, di);
            } else if di < self.damaged.len() && self.damaged[di] == damaged {
                result += self.calc_rec(seen, 0, si + 1, di + 1);
            }
        }

        if spring == '#' || spring == '?' {
            // current spring is damaged, or supposed to be
            result += self.calc_rec(seen, damaged + 1, si + 1, di);
        }

        seen.insert(key, result);

        result
    }
}

struct Puzzle {
    field: Vec<Row>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { field: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            let mut line = line.split_ascii_whitespace();

            let row = Row {
                springs: line.next().unwrap().chars().collect(),
                damaged: line
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect(),
            };
            self.field.push(row);
        }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        self.field.iter().map(Row::calc).sum()
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        self.field
            .iter()
            .map(|row| {
                let mut row5 = row.clone();
                for _ in 0..4 {
                    row5.springs.push('?');
                    row5.springs.extend(row.springs.iter());
                    row5.damaged.extend(row.damaged.iter());
                }

                row5.calc()
            })
            .sum()
    }
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 21);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 525152);
    }
}

fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
