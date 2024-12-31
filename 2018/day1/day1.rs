//! [Day 1: Chronal Calibration](https://adventofcode.com/2018/day/1)

use std::collections::HashSet;

struct Puzzle<'a> {
    data: &'a str,
}

impl<'a> Puzzle<'a> {
    const fn new(data: &'a str) -> Self {
        Self { data }
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        self.data.lines().map(|x| x.parse::<i32>().unwrap()).sum()
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        let mut frequencies = HashSet::new();
        let mut sum = 0;
        loop {
            for i in self.data.lines() {
                sum += i.parse::<i32>().unwrap();
                if frequencies.contains(&sum) {
                    return sum;
                }
                frequencies.insert(sum);
            }
        }
    }
}

fn main() {
    let args = aoc::parse_args();
    let puzzle = Puzzle::new(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let data = aoc::load_input_data("test1.txt");
        let puzzle = Puzzle::new(&data);
        assert_eq!(puzzle.part1(), -6);
    }

    #[test]
    fn test02() {
        let data = aoc::load_input_data("test2.txt");
        let puzzle = Puzzle::new(&data);
        assert_eq!(puzzle.part2(), 14);
    }
}
