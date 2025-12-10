//! [Day 9: Movie Theater](https://adventofcode.com/2025/day/9)

use itertools::Itertools;

struct Point {
    x: i32,
    y: i32,
}
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
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect_tuple()
                        .unwrap();

                    Point { x, y }
                })
                .collect(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        let mut max_area = 0;

        for (i, a) in self.points.iter().enumerate() {
            for b in self.points.iter().skip(i + 1) {
                let area = (u64::from(a.x.abs_diff(b.x)) + 1) * (u64::from(a.y.abs_diff(b.y)) + 1);
                if area > max_area {
                    max_area = area;
                }
            }
        }

        max_area
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        let mut max_area = 0;
        let n = self.points.len();

        // If the first edge (p0-p1) is vertical, the next edge (p1-p2) is horizontal.
        // So, we have to use points 1,3,5,... for the horizontal edges and 0,2,4... for the v ones.
        // Otherwise, it is the opposite.
        let start = usize::from(self.points[0].x == self.points[1].x);

        let horizontal_edges = self
            .points
            .iter()
            .enumerate()
            .skip(start)
            .step_by(2)
            .map(|(i, p)| {
                let x1 = p.x;
                let x2 = self.points[(i + 1) % n].x;
                (x1.min(x2), x1.max(x2), p.y)
            })
            .collect::<Vec<_>>();

        let vertical_edges = self
            .points
            .iter()
            .enumerate()
            .skip(1 - start)
            .step_by(2)
            .map(|(i, p)| {
                let y1 = p.y;
                let y2 = self.points[(i + 1) % n].y;
                (p.x, y1.min(y2), y1.max(y2))
            })
            .collect::<Vec<_>>();

        let is_ok = |a: &Point, b: &Point| -> bool {
            let x1 = a.x.min(b.x);
            let x2 = a.x.max(b.x);
            let y1 = a.y.min(b.y);
            let y2 = a.y.max(b.y);

            //
            // y1 →   Axxxxx        If there is an horizontal edge between
            //        x    x        the top and bottom of the rectangle,
            //        x    x        it should be entirely at the left or at the right
            // ey →  hhhh  x        but not overlap the horizontal side of the rectangle.
            //        x    x        If this is the case, either the top or bottom of
            // y2 →   xxxxxB        the edge is outside the polygon.
            //        ↑    ↑
            //        x1   x2       ❌ The configuration at the left is invalid.
            //
            // y1 →   Axxxxx
            //        x    x  hhhhh
            //        x    x
            //  hhh   x    x        ✅ These both configurations are ok.
            //        x    x
            // y2 →   xxxxxB
            //
            for &(edge_x1, edge_x2, edge_y) in &horizontal_edges {
                if y1 < edge_y && edge_y < y2 && x1 < edge_x2 && edge_x1 < x2 {
                    return false;
                }
            }

            // Same thing with vertical edges.
            for &(edge_x, edge_y1, edge_y2) in &vertical_edges {
                if x1 < edge_x && edge_x < x2 && y1 < edge_y2 && edge_y1 < y2 {
                    return false;
                }
            }

            true
        };

        for (i, a) in self.points.iter().enumerate() {
            for b in self.points.iter().skip(i + 1) {
                let area = (u64::from(a.x.abs_diff(b.x)) + 1) * (u64::from(a.y.abs_diff(b.y)) + 1);
                if area > max_area && is_ok(a, b) {
                    max_area = area;
                }
            }
        }

        max_area
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u64, u64) {
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
        assert_eq!(puzzle.part1(), 50);
    }

    #[test]
    fn part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 24);
    }
}
