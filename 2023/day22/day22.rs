//! [Day 22: Sand Slabs](https://adventofcode.com/2023/day/22)

use std::collections::{HashMap, HashSet, VecDeque};

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    const fn new(c: &[i32]) -> Self {
        Self {
            x: c[0],
            y: c[1],
            z: c[2],
        }
    }
}

struct Brick {
    a: Point,
    b: Point,
}

impl Brick {
    fn new(coords: &[i32]) -> Self {
        Self {
            a: Point::new(&coords[0..3]),
            b: Point::new(&coords[3..6]),
        }
    }

    /// Return true if two bricks overlap in 2D
    fn overlap(&self, other: &Self) -> bool {
        self.a.x.max(other.a.x) <= self.b.x.min(other.b.x)
            && self.a.y.max(other.a.y) <= self.b.y.min(other.b.y)
    }
}

struct Puzzle {
    bricks: Vec<Brick>,                           // list of bricks sorted lowest first
    supports: HashMap<usize, HashSet<usize>>,     // set of bricks supported by another brick
    supported_by: HashMap<usize, HashSet<usize>>, // set of bricks that support another brick
}

impl Puzzle {
    fn new() -> Self {
        Self {
            bricks: vec![],
            supports: HashMap::new(),
            supported_by: HashMap::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(path)?;

        // load the bricks of sand
        for line in data.lines() {
            let coords: Vec<_> = line
                .split([',', '~'])
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            self.bricks.push(Brick::new(&coords));
        }

        // let a.z the lowest coordinate
        self.bricks.sort_unstable_by_key(|brick| brick.a.z);

        let n = self.bricks.len();

        // let the bricks fall downward until blocked
        for i in 0..n {
            let brick = &self.bricks[i];

            let max_z = self.bricks[..i]
                .iter()
                .filter(|&b| b.overlap(brick))
                .map(|brick| brick.b.z)
                .max()
                .unwrap_or(0)
                + 1;

            let brick = self.bricks.get_mut(i).unwrap();
            brick.b.z -= brick.a.z - max_z;
            brick.a.z = max_z;
        }

        // who supports whom ?
        for i in 0..n {
            self.supports.insert(i, HashSet::new());
            self.supported_by.insert(i, HashSet::new());
        }

        for (i, upper) in self.bricks.iter().enumerate() {
            for (j, lower) in self.bricks[..i].iter().enumerate() {
                if upper.overlap(lower) && upper.a.z == lower.b.z + 1 {
                    self.supported_by.get_mut(&i).unwrap().insert(j);
                    self.supports.get_mut(&j).unwrap().insert(i);
                }
            }
        }

        Ok(())
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        (0..self.bricks.len())
            .filter(|j| {
                self.supports[j]
                    .iter()
                    .all(|i| self.supported_by[i].len() >= 2)
            })
            .count()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        (0..self.bricks.len())
            .map(|j| {
                let mut q = VecDeque::new();
                let mut fall = HashSet::new();

                for &i in &self.supports[&j] {
                    if self.supported_by[&i].len() == 1 {
                        q.push_back(i);
                        fall.insert(i);
                    }
                }

                while let Some(j) = q.pop_front() {
                    let e = self.supports[&j]
                        .difference(&fall)
                        .copied()
                        .collect::<Vec<_>>();
                    for k in e {
                        if fall.is_superset(&self.supported_by[&k]) {
                            q.push_back(k);
                            fall.insert(k);
                        }
                    }
                }

                fall.len()
            })
            .sum()
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    if puzzle.configure(args.path.as_str()).is_ok() {
        println!("{}", puzzle.part1());
        println!("{}", puzzle.part2());
    }
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        assert!(puzzle.configure("test.txt").is_ok());
        assert_eq!(puzzle.part1(), 5);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        assert!(puzzle.configure("test.txt").is_ok());
        assert_eq!(puzzle.part2(), 7);
    }
}
