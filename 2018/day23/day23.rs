//! [Day 23: Experimental Emergency Teleportation](https://adventofcode.com/2018/day/23)

use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Dichotomy3D {
    // front left bottom
    x1: i32,
    y1: i32,
    z1: i32,
    // rear right top
    x2: i32,
    y2: i32,
    z2: i32,
}

impl Dichotomy3D {
    const fn from_pow2(n: i32) -> Self {
        let n = n - 1;
        Self {
            x1: -(1 << n),
            y1: -(1 << n),
            z1: -(1 << n),
            x2: (1 << n) - 1,
            y2: (1 << n) - 1,
            z2: (1 << n) - 1,
        }
    }

    const fn size(&self) -> i32 {
        self.x2 - self.x1 + 1
    }

    fn closest(&self) -> i32 {
        let x = self.x1.abs().min(self.x2.abs());
        let y = self.y1.abs().min(self.y2.abs());
        let z = self.z1.abs().min(self.z2.abs());
        x + y + z
    }

    const fn new(x1: i32, y1: i32, z1: i32, x2: i32, y2: i32, z2: i32) -> Self {
        Self {
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
        }
    }

    /// Split the cube into eight cubes with each dimensions divided by 2
    fn split(&self) -> impl Iterator<Item = Self> + '_ {
        //
        //    z (bottom/top)
        //    |
        //    |____ y (left/right)
        //    /
        //   /
        //  x (front/rear)
        //

        let front_x1 = self.x1;
        let left_y1 = self.y1;
        let bottom_z1 = self.z1;

        let rear_x2 = self.x2;
        let right_y2 = self.y2;
        let top_z2 = self.z2;

        let front_x2 = (self.x1 + self.x2) / 2;
        let left_y2 = (self.y1 + self.y2) / 2;
        let bottom_z2 = (self.z1 + self.z2) / 2;

        let rear_x1 = front_x2 + 1;
        let right_y1 = left_y2 + 1;
        let top_z1 = bottom_z2 + 1;

        (0..8).map(move |i| match i {
            0 => Self::new(front_x1, left_y1, bottom_z1, front_x2, left_y2, bottom_z2), // front left bottom
            1 => Self::new(front_x1, right_y1, bottom_z1, front_x2, right_y2, bottom_z2), // front right bottom
            2 => Self::new(rear_x1, left_y1, bottom_z1, rear_x2, left_y2, bottom_z2), // rear left bottom
            3 => Self::new(rear_x1, right_y1, bottom_z1, rear_x2, right_y2, bottom_z2), // rear right bottom
            4 => Self::new(front_x1, left_y1, top_z1, front_x2, left_y2, top_z2), // front left top
            5 => Self::new(front_x1, right_y1, top_z1, front_x2, right_y2, top_z2), // front right top
            6 => Self::new(rear_x1, left_y1, top_z1, rear_x2, left_y2, top_z2),     // rear left top
            7 => Self::new(rear_x1, right_y1, top_z1, rear_x2, right_y2, top_z2), // rear right top
            _ => unreachable!(),
        })
    }
}

struct Nanobot {
    x: i32,
    y: i32,
    z: i32,
    r: i32,
}

impl Nanobot {
    fn in_range(&self, cube: &Dichotomy3D) -> bool {
        let x = (cube.x1 - self.x).max(0) + (self.x - cube.x2).max(0);
        let y = (cube.y1 - self.y).max(0) + (self.y - cube.y2).max(0);
        let z = (cube.z1 - self.z).max(0) + (self.z - cube.z2).max(0);
        x + y + z <= self.r
    }

    const fn manhattan(&self, other: &Self) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs() + (other.z - self.z).abs()
    }
}

struct Cost {
    range: usize,
    closest: i32,
    size: i32,
    cube: Dichotomy3D,
}

impl PartialEq for Cost {
    fn eq(&self, rhs: &Self) -> bool {
        self.range == rhs.range && self.closest == rhs.closest && self.size == rhs.size
    }
}

impl Eq for Cost {}

impl PartialOrd for Cost {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
impl Ord for Cost {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.range
            .cmp(&rhs.range)
            .then(rhs.closest.cmp(&self.closest))
            .then(rhs.size.cmp(&self.size))
    }
}

struct Puzzle {
    nanobots: Vec<Nanobot>,
}

impl Puzzle {
    const fn new() -> Self {
        Self { nanobots: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        let re = Regex::new(r"^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(-?\d+)$").unwrap();

        for line in data.lines() {
            if let Some(caps) = re.captures(line) {
                self.nanobots.push(Nanobot {
                    x: caps.get(1).unwrap().as_str().parse().unwrap(),
                    y: caps.get(2).unwrap().as_str().parse().unwrap(),
                    z: caps.get(3).unwrap().as_str().parse().unwrap(),
                    r: caps.get(4).unwrap().as_str().parse().unwrap(),
                });
            }
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let strongest = self.nanobots.iter().max_by_key(|a| a.r).unwrap();

        self.nanobots
            .iter()
            .filter(|bot| bot.manhattan(strongest) <= strongest.r)
            .count()
    }

    fn part2(&self) -> i32 {
        let mut heap = BinaryHeap::new();

        heap.push(Cost {
            range: 0,
            closest: 0,
            size: 0,
            cube: Dichotomy3D::from_pow2(30),
        });

        while let Some(cost) = heap.pop() {
            if cost.cube.size() == 1 {
                return cost.cube.closest();
            }

            for cube in cost.cube.split() {
                let range = self
                    .nanobots
                    .iter()
                    .filter(|bot| bot.in_range(&cube))
                    .count();

                let c = Cost {
                    range,
                    closest: cube.closest(),
                    size: cube.size(),
                    cube,
                };

                heap.push(c);
            }
        }

        0
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
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
        puzzle.configure(&aoc::load_input_data("sample_1.txt"));
        assert_eq!(puzzle.part1(), 7);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("sample_2.txt"));
        assert_eq!(puzzle.part2(), 36);
    }
}
