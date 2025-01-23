//! [Day 12: Hot Springs](https://adventofcode.com/2023/day/12)

use rustc_hash::FxHashMap;

#[derive(Clone)]
struct Row {
    springs: Vec<char>,
    damaged: Vec<u64>,
}

impl Row {
    fn calc(&self) -> u64 {
        self.calc_rec(&mut FxHashMap::default(), 0, 0, 0)
    }

    fn calc_rec(
        &self,
        seen: &mut FxHashMap<(u64, usize, usize), u64>,
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
    fn new(data: &str) -> Self {
        Self {
            field: data
                .lines()
                .map(|line| {
                    let mut line = line.split_ascii_whitespace();

                    Row {
                        springs: line.next().unwrap().chars().collect(),
                        damaged: line
                            .next()
                            .unwrap()
                            .split(',')
                            .map(|x| x.parse::<u64>().unwrap())
                            .collect(),
                    }
                })
                .collect(),
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

/// # Panics
/// over malformed input
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
        assert_eq!(puzzle.part1(), 21);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 525152);
    }
}
