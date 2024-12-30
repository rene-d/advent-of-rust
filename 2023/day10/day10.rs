//! [Day 10: Pipe Maze](https://adventofcode.com/2023/day/10)

use geo::Contains;
use geo::Polygon;
use geo_types::{Coord, LineString};

use std::collections::{HashSet, VecDeque};

use itertools::iproduct;

struct Puzzle {
    grid_data: Vec<Vec<char>>,
    sx: i32,
    sy: i32,
    start_position: (i32, i32),
    points: Vec<(i32, i32)>,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            grid_data: vec![],
            sx: 0,
            sy: 0,
            start_position: (0, 0),
            points: vec![],
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.points.clear();
        self.grid_data.clear();
        self.sx = 0;
        for line in data.lines() {
            if self.sx == 0 {
                self.sx = i32::try_from(line.len()).unwrap();
            } else {
                assert_eq!(self.sx, i32::try_from(line.len()).unwrap());
            }

            self.grid_data.push(line.chars().collect());
        }
        self.sy = i32::try_from(self.grid_data.len()).unwrap();

        for (x, y) in iproduct!(0..self.sx, 0..self.sy) {
            if self.grid(x, y) == 'S' {
                self.start_position = (x, y);
                break;
            }
        }

        self.maze();
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
        let mut visited = HashSet::new();
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
    fn test01_1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test1.txt");
        assert_eq!(puzzle.part1(), 4);
    }
    #[test]
    fn test01_2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test2.txt");
        assert_eq!(puzzle.part1(), 8);
    }

    #[test]
    fn test02_3() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test3.txt");
        assert_eq!(puzzle.part2(), 4);
    }

    #[test]
    fn test02_4() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test4.txt");
        assert_eq!(puzzle.part2(), 4);
    }
    #[test]
    fn test02_5() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test5.txt");
        assert_eq!(puzzle.part2(), 8);
    }

    #[test]
    fn test02_6() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test6.txt");
        assert_eq!(puzzle.part2(), 10);
    }
}
