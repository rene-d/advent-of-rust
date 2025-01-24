//! [Day 10: Pipe Maze](https://adventofcode.com/2023/day/10)

use geo::Contains;
use geo::Polygon;
use geo_types::{Coord, LineString};

use rustc_hash::FxHashSet;
use std::collections::VecDeque;

use itertools::iproduct;

struct Puzzle {
    grid_data: Vec<Vec<char>>,
    sx: i32,
    sy: i32,
    start_position: (i32, i32),
    points: Vec<(i32, i32)>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut puzzle = Self {
            grid_data: vec![],
            sx: 0,
            sy: 0,
            start_position: (0, 0),
            points: vec![],
        };

        puzzle.points.clear();
        puzzle.grid_data.clear();
        puzzle.sx = 0;
        for line in data.lines() {
            if puzzle.sx == 0 {
                puzzle.sx = i32::try_from(line.len()).unwrap();
            } else {
                assert_eq!(puzzle.sx, i32::try_from(line.len()).unwrap());
            }

            puzzle.grid_data.push(line.chars().collect());
        }
        puzzle.sy = i32::try_from(puzzle.grid_data.len()).unwrap();

        for (x, y) in iproduct!(0..puzzle.sx, 0..puzzle.sy) {
            if puzzle.grid(x, y) == 'S' {
                puzzle.start_position = (x, y);
                break;
            }
        }

        puzzle.maze();

        puzzle
    }

    fn grid(&self, x: i32, y: i32) -> char {
        if 0 <= x && x < self.sx && 0 <= y && y < self.sy {
            let x = usize::try_from(x).unwrap();
            let y = usize::try_from(y).unwrap();
            self.grid_data[y][x]
        } else {
            '.'
        }
    }

    fn maze(&mut self) {
        let mut visited = FxHashSet::default();
        let mut queue = VecDeque::new();

        queue.push_back(self.start_position);

        while !queue.is_empty() {
            let p = queue.pop_back().unwrap();

            if visited.contains(&p) {
                continue;
            }
            visited.insert(p);

            self.points.push(p);

            let (x, y) = p;
            let c = self.grid(x, y);

            if "|LJ".contains(self.grid(x, y + 1)) && "|7FS".contains(c) {
                queue.push_back((x, y + 1));
            }

            if "|7F".contains(self.grid(x, y - 1)) && "|LJS".contains(c) {
                queue.push_back((x, y - 1));
            }

            if "-FL".contains(self.grid(x - 1, y)) && "-J7S".contains(c) {
                queue.push_back((x - 1, y));
            }

            if "-7J".contains(self.grid(x + 1, y)) && "-FLS".contains(c) {
                queue.push_back((x + 1, y));
            }
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.points.len() / 2
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let line_string = self
            .points
            .iter()
            .map(|&(x, y)| Coord { x, y })
            .collect::<LineString<i32>>();

        let polygon = Polygon::new(line_string, vec![]);

        let mut n = 0;
        for (x, y) in iproduct!(0..self.sx, 0..self.sy) {
            let p = Coord { x, y };
            if polygon.contains(&p) {
                n += 1;
            }
        }

        n
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, u32) {
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

    const TEST_INPUT_1: &str = include_str!("test1.txt");
    const TEST_INPUT_2: &str = include_str!("test2.txt");
    const TEST_INPUT_3: &str = include_str!("test3.txt");
    const TEST_INPUT_4: &str = include_str!("test4.txt");
    const TEST_INPUT_5: &str = include_str!("test5.txt");
    const TEST_INPUT_6: &str = include_str!("test6.txt");

    #[test]
    fn test01_1() {
        let puzzle = Puzzle::new(TEST_INPUT_1);
        assert_eq!(puzzle.part1(), 4);
    }
    #[test]
    fn test01_2() {
        let puzzle = Puzzle::new(TEST_INPUT_2);
        assert_eq!(puzzle.part1(), 8);
    }

    #[test]
    fn test02_3() {
        let puzzle = Puzzle::new(TEST_INPUT_3);
        assert_eq!(puzzle.part2(), 4);
    }

    #[test]
    fn test02_4() {
        let puzzle = Puzzle::new(TEST_INPUT_4);
        assert_eq!(puzzle.part2(), 4);
    }
    #[test]
    fn test02_5() {
        let puzzle = Puzzle::new(TEST_INPUT_5);
        assert_eq!(puzzle.part2(), 8);
    }

    #[test]
    fn test02_6() {
        let puzzle = Puzzle::new(TEST_INPUT_6);
        assert_eq!(puzzle.part2(), 10);
    }
}
