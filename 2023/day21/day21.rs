//! [Day 21: Step Counter](https://adventofcode.com/2023/day/21)

use num::Integer;
use std::collections::HashSet;

struct Puzzle {
    garden: Vec<bool>, // twice as fast as the HashSet
    // rocks: HashSet<(i32, i32)>,
    n: i32,
    start_x: i32,
    start_y: i32,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            // rocks: HashSet::new(),
            garden: vec![],
            n: 0, // the map has to be a square n x n
            start_x: 0,
            start_y: 0,
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        let n = data.lines().count();

        assert_eq!(n, data.lines().nth(0).unwrap().len());

        self.n = i32::try_from(n).unwrap();

        self.garden.resize(n * n, true);

        for (y, line) in data.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        self.garden[n * y + x] = false;

                        // let x = i32::try_from(x).unwrap();
                        // let y = i32::try_from(y).unwrap();
                        // self.rocks.insert((x, y));
                    }
                    'S' => {
                        self.start_x = i32::try_from(x).unwrap();
                        self.start_y = i32::try_from(y).unwrap();
                    }
                    '.' => (),
                    _ => panic!(),
                };
            }
        }
    }

    fn is_garden(&self, x: i32, y: i32) -> bool {
        let x = x.rem_euclid(self.n);
        let y = y.rem_euclid(self.n);
        let i = self.n * y + x;

        self.garden[usize::try_from(i).unwrap()]
    }

    fn count(&self, n: i32) -> u64 {
        // nota: still not really optimized, could probably memoize something
        let mut p = HashSet::new();
        p.insert((self.start_x, self.start_y));

        for _ in 0..n {
            let mut np = HashSet::new();

            for (x, y) in p {
                /*
                if !self
                    .rocks
                    .contains(&((x - 1).rem_euclid(self.n), y.rem_euclid(self.n)))
                {
                    np.insert((x - 1, y));
                }
                if !self
                    .rocks
                    .contains(&((x + 1).rem_euclid(self.n), y.rem_euclid(self.n)))
                {
                    np.insert((x + 1, y));
                }
                if !self
                    .rocks
                    .contains(&((x).rem_euclid(self.n), (y - 1).rem_euclid(self.n)))
                {
                    np.insert((x, y - 1));
                }
                if !self
                    .rocks
                    .contains(&(x.rem_euclid(self.n), (y + 1).rem_euclid(self.n)))
                {
                    np.insert((x, y + 1));
                }
                */

                if self.is_garden(x - 1, y) {
                    np.insert((x - 1, y));
                }
                if self.is_garden(x + 1, y) {
                    np.insert((x + 1, y));
                }
                if self.is_garden(x, y - 1) {
                    np.insert((x, y - 1));
                }
                if self.is_garden(x, y + 1) {
                    np.insert((x, y + 1));
                }
            }

            p = np;
        }

        p.len() as u64
    }

    fn big_count(&self, n: i32) -> u64 {
        // the step count curve is parabolic
        let (t, x0) = n.div_rem(&self.n);

        let y0 = self.count(x0);
        let y1 = self.count(x0 + self.n);
        let y2 = self.count(x0 + self.n * 2);

        // println!("f(x) = a⋅x² + b⋅x + c");
        // println!("x0={x0} → y0={y0}");
        // println!("x1={} → y1={y1}", x0 + self.n);
        // println!("x2={} → y2={y2}", x0 + self.n * 2);

        // let y3 = self.count(x0 + self.n * 3);
        // println!("x3={} → y3={y3}", x0 + self.n * 3);

        let a = y2 - 2 * y1 + y0;
        let b = y1 - y0;

        let t = u64::try_from(t).unwrap();

        a * t * (t - 1) / 2 + b * t + y0
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        self.count(64)
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        self.big_count(26_501_365)
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
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
        assert_eq!(puzzle.count(6), 16);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.count(10), 50);
        assert_eq!(puzzle.count(50), 1594);
        assert_eq!(puzzle.count(100), 6536);
        // assert_eq!(puzzle.big_count(500), 167004);
        // assert_eq!(puzzle.big_count(1000), 668697);
        // assert_eq!(puzzle.big_count(5000), 16733044);
    }
}
