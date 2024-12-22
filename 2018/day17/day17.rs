//! [Day 17: Reservoir Research](https://adventofcode.com/2018/day/17)

use nom::{bytes::complete::tag, character::complete::digit1, combinator::map_res, IResult};
use std::collections::HashMap;

const CLAY: u8 = 1;
const SETTLE: u8 = 2;
const FLOW: u8 = 4;

#[derive(Eq, PartialEq)]
enum Dir {
    Down,
    Left,
    Right,
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    const fn go(&self, dir: &Dir) -> Self {
        match dir {
            Dir::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Dir::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Dir::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
        }
    }
}

fn x_yy(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let (input, _) = tag("x=")(input)?;
    let (input, x) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y1) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = tag("..")(input)?;
    let (input, y2) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    Ok((input, (x, y1, y2)))
}

fn y_xx(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let (input, _) = tag("y=")(input)?;
    let (input, x) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = tag(", x=")(input)?;
    let (input, y1) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = tag("..")(input)?;
    let (input, y2) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    Ok((input, (x, y1, y2)))
}

struct Puzzle {
    grid: HashMap<Point, u8>,
    ymin: u32,
    ymax: u32,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            grid: HashMap::new(),
            ymin: 0,
            ymax: 0,
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            if let Ok((_, (x, y1, y2))) = x_yy(line) {
                assert!(y1 < y2);

                for y in y1..=y2 {
                    self.grid.insert(Point { x, y }, CLAY);
                }
            } else if let Ok((_, (y, x1, x2))) = y_xx(line) {
                assert!(x1 < x2);

                for x in x1..=x2 {
                    self.grid.insert(Point { x, y }, CLAY);
                }
            } else {
                panic!("bad input line: {line}");
            }
        }

        self.ymin = self.grid.keys().map(|p| p.y).min().unwrap();
        self.ymax = self.grid.keys().map(|p| p.y).max().unwrap();

        // self.grid.insert(Point { x: 500, y: 0 }, SPRING);
    }

    fn show(&self) {
        let xmin = self.grid.keys().map(|p| p.x).min().unwrap();
        let xmax = self.grid.keys().map(|p| p.x).max().unwrap();

        for y in 0..=self.ymax {
            for x in xmin..=xmax {
                let c =
                    self.grid
                        .get(&Point { x, y })
                        .map_or("\x1b[38;5;231m.\x1b[0m", |t| match *t {
                            CLAY => "\x1b[38;5;166m#\x1b[0m",
                            SETTLE => "?",
                            FLOW => "\x1b[94m|\x1b[0m",
                            _ => "\x1b[96m~\x1b[0m",
                        });
                print!("{c}");
            }
            println!();
        }

        println!();
    }

    fn is(&self, p: &Point, t: u8) -> bool {
        self.grid.get(p).is_some_and(|&x| (x & t) == t)
    }

    fn is_not(&self, p: &Point, t: u8) -> bool {
        !self.is(p, t)
    }

    fn set(&mut self, p: &Point, t: u8) {
        *self.grid.entry(p.clone()).or_insert(t) |= t;
    }

    fn fill(&mut self, start: &Point, dir: &Dir) -> bool {
        self.set(start, FLOW);

        let below = start.go(&Dir::Down);
        let mut left = start.go(&Dir::Left);
        let mut right = start.go(&Dir::Right);

        if self.is_not(&below, CLAY) {
            if self.is_not(&below, FLOW) && 1 <= below.y && below.y <= self.ymax {
                self.fill(&below, &Dir::Down);
            }
            if self.is_not(&below, SETTLE) {
                return false;
            }
        }

        let fill_left =
            self.is(&left, CLAY) || (self.is_not(&left, FLOW) && self.fill(&left, &Dir::Left));

        let fill_right =
            self.is(&right, CLAY) || (self.is_not(&right, FLOW) && self.fill(&right, &Dir::Right));

        if dir == &Dir::Down && fill_left && fill_right {
            self.set(start, SETTLE);

            while self.is(&left, FLOW) {
                self.set(&left, SETTLE);
                left = left.go(&Dir::Left);
            }

            while self.is(&right, FLOW) {
                self.set(&right, SETTLE);
                right = right.go(&Dir::Right);
            }
        }

        (dir == &Dir::Left && (fill_left || self.is(&left, CLAY)))
            || (dir == &Dir::Right && (fill_right || self.is(&right, CLAY)))
    }

    fn solve(&mut self) {
        self.fill(&Point { x: 500, y: 0 }, &Dir::Down);
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.grid
            .iter()
            .filter(|(p, t)| p.y >= self.ymin && p.y <= self.ymax && (**t & (FLOW + SETTLE)) != 0)
            .count()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        self.grid
            .iter()
            .filter(|(p, t)| p.y >= self.ymin && p.y <= self.ymax && (**t & SETTLE) != 0)
            .count()
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.path);

    puzzle.solve();

    if args.verbose {
        puzzle.show();
    }

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
        puzzle.solve();
        assert_eq!(puzzle.part1(), 57);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        puzzle.solve();
        assert_eq!(puzzle.part2(), 29);
    }
}
