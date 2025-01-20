//! [Day 14: Restroom Redoubt](https://adventofcode.com/2024/day/14)

use regex::Regex;
use rustc_hash::FxHashMap;

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
    fn new(data: &str) -> Self {
        let mut robots = Vec::new();

        let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

        for line in data.lines() {
            let caps = re.captures(line).unwrap();

            let robot = Robot {
                px: caps.get(1).unwrap().as_str().parse().unwrap(),
                py: caps.get(2).unwrap().as_str().parse().unwrap(),
                vx: caps.get(3).unwrap().as_str().parse().unwrap(),
                vy: caps.get(4).unwrap().as_str().parse().unwrap(),
            };

            robots.push(robot);
        }

        // #[cfg(not(test))]
        Self {
            robots,
            width: 101,
            height: 103,
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut quadrants = FxHashMap::default();

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
            let mut grid: FxHashMap<(i32, i32), u32> = FxHashMap::default();

            for robot in &self.robots {
                let px = (robot.px + robot.vx * seconds).rem_euclid(self.width);
                let py = (robot.py + robot.vy * seconds).rem_euclid(self.height);

                if grid.contains_key(&(px, py)) {
                    continue 'outer;
                }

                *grid.entry((px, py)).or_default() += 1;
            }

            // check if we have an horizontal line
            let mut horizontal_lines = 0;
            for y in 0..self.height {
                for x in 0..(self.width - 10) {
                    horizontal_lines += u32::from((x..(x + 5)).all(|i| grid.contains_key(&(i, y))));
                }
            }

            if horizontal_lines > 5 {
                return seconds;
            }
        }

        0
    }
}

fn solve(data: &str) -> (u32, i32) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

fn main() {
    let mut args = aoc::parse_args();
    args.run(solve);
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new(TEST_INPUT);
        puzzle.width = 11;
        puzzle.height = 7;
        assert_eq!(puzzle.part1(), 12);
    }
}
