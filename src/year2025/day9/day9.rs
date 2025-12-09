//! [Day 9: Movie Theater](https://adventofcode.com/2025/day/9)

// Nota: not a good solution 😡 But it solves the puzzle...

use geo::algorithm::contains::Contains;
use geo::{Point, Polygon, Rect};
use itertools::Itertools;

struct Puzzle {
    points: Vec<Point>,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        Self {
            points: data
                .lines()
                .map(|line| {
                    let (x, y) = line
                        .split(',')
                        .map(|x| x.parse::<f64>().unwrap())
                        .collect_tuple()
                        .unwrap();

                    Point::new(x, y)
                })
                .collect(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> f64 {
        let mut max_area = 0.;

        for (i, p1) in self.points.iter().enumerate() {
            for (j, p2) in self.points.iter().enumerate() {
                if i > j {
                    let xmin = p1.x().min(p2.x());
                    let xmax = p1.x().max(p2.x());
                    let ymin = p1.y().min(p2.y());
                    let ymax = p1.y().max(p2.y());

                    let area = (xmax + 1. - xmin) * (ymax + 1. - ymin);
                    if area > max_area {
                        max_area = area;
                    }
                }
            }
        }

        max_area
    }

    /// Solve part two.
    fn part2(&self) -> f64 {
        let poly = Polygon::new(self.points.clone().into(), vec![]);

        let mut max_area = 0.;

        for (i, p1) in self.points.iter().enumerate() {
            for (j, p2) in self.points.iter().enumerate() {
                if i > j {
                    let xmin = p1.x().min(p2.x());
                    let xmax = p1.x().max(p2.x());
                    let ymin = p1.y().min(p2.y());
                    let ymax = p1.y().max(p2.y());

                    let rect = Rect::new((xmin, ymin), (xmax, ymax));

                    if poly.contains(&rect) {
                        let area = (xmax + 1. - xmin) * (ymax + 1. - ymin);
                        if area > max_area {
                            max_area = area;
                        }
                    }
                }
            }
        }

        max_area
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (f64, f64) {
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

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 50.);
    }

    #[test]
    fn part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 24.);
    }
}
