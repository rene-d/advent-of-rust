//! [Day 14: Restroom Redoubt](https://adventofcode.com/2024/day/14)

use regex::Regex;
use std::collections::HashMap;

struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

struct Puzzle {
    robots: Vec<Robot>,
    width: i32,
    height: i32,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            robots: Vec::new(),
            width: 101,
            height: 103,
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

        for line in data.lines() {
            let caps = re.captures(line).unwrap();

            let robot = Robot {
                px: caps.get(1).unwrap().as_str().parse().unwrap(),
                py: caps.get(2).unwrap().as_str().parse().unwrap(),
                vx: caps.get(3).unwrap().as_str().parse().unwrap(),
                vy: caps.get(4).unwrap().as_str().parse().unwrap(),
            };

            self.robots.push(robot);
        }

        if path == "test.txt" {
            self.width = 11;
            self.height = 7;
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut quadrants = HashMap::new();

        for robot in &self.robots {
            let px = (robot.px + robot.vx * 100).rem_euclid(self.width);
            let py = (robot.py + robot.vy * 100).rem_euclid(self.height);

            if px == self.width / 2 || py == self.height / 2 {
                continue;
            }

            let q = ((px * 2) / self.width, (py * 2) / self.height);
            *quadrants.entry(q).or_default() += 1_u32;
        }

        quadrants.values().product::<u32>()
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        // assume there is a christmas tree in the middle of picture
        // when no robot is in the same place

        'outer: for seconds in 0..100_000 {
            let mut grid: HashMap<(i32, i32), u32> = HashMap::new();

            for robot in &self.robots {
                let px = (robot.px + robot.vx * seconds).rem_euclid(self.width);
                let py = (robot.py + robot.vy * seconds).rem_euclid(self.height);

                if grid.contains_key(&(px, py)) {
                    continue 'outer;
                }

                *grid.entry((px, py)).or_default() += 1;
            }

            return seconds;
        }

        0
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
        assert_eq!(puzzle.part1(), 12);
    }
}
