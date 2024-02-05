//! [Day 8: Haunted Wasteland](https://adventofcode.com/2023/day/8)

use num::Integer;
use std::collections::HashMap;

fn lcm(values: &Vec<u64>) -> u64 {
    let mut m = 1;
    for x in values {
        m = m.lcm(x);
    }
    m
}

struct Puzzle {
    navigation: String,
    network: HashMap<u32, (u32, u32)>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            navigation: String::new(),
            network: HashMap::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        let mut lines = data.lines();

        self.navigation = lines.next().unwrap().to_string();
        lines.next();

        for line in lines {
            let node = &line[0..3];
            let node = u32::from_str_radix(node, 36).unwrap();

            let left = u32::from_str_radix(&line[7..10], 36).unwrap();
            let right = u32::from_str_radix(&line[12..15], 36).unwrap();

            self.network.insert(node, (left, right));
        }
    }

    fn solve(&self, part1: bool) -> u64 {
        let start = u32::from_str_radix(if part1 { "AAA" } else { "A" }, 36).unwrap();
        let stop = u32::from_str_radix(if part1 { "ZZZ" } else { "Z" }, 36).unwrap();
        let mask = if part1 { 36 * 36 * 36 } else { 36 };

        let mut q: Vec<u32> = self
            .network
            .keys()
            .filter(|&&x| x % mask == start)
            .copied()
            .collect();

        let size = q.len();

        let mut z = HashMap::new();

        let mut n = 0;
        loop {
            let d = self
                .navigation
                .chars()
                .nth(n % self.navigation.len())
                .unwrap();
            n += 1;

            for i in 0..size {
                let node = q.get_mut(i).unwrap();

                let new_node = if d == 'L' {
                    self.network[node].0
                } else {
                    self.network[node].1
                };

                *node = new_node;

                if new_node % mask == stop {
                    z.insert(i, n as u64);

                    if z.len() == size {
                        let z: Vec<_> = z.values().copied().collect();
                        return lcm(&z);
                    }
                }
            }
        }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        self.solve(true)
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        self.solve(false)
    }
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test1.txt");
        assert_eq!(puzzle.part1(), 6);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test2.txt");
        assert_eq!(puzzle.part2(), 6);
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
