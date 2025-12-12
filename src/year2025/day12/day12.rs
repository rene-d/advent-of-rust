//! [Day 12: Christmas Tree Farm](https://adventofcode.com/2025/day/12)

use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};

/// A 3x3 present shape.
#[derive(Eq, PartialEq, Hash, Clone)]
struct Shape(Vec<(usize, usize)>);

impl Shape {
    const fn new() -> Self {
        Self(vec![])
    }

    fn rotate_clockwise(&self) -> Self {
        Self(self.0.iter().map(|&(x, y)| (2 - y, x)).collect())
    }

    fn flip_horizontal(&self) -> Self {
        Self(self.0.iter().map(|&(x, y)| (x, 2 - y)).collect())
    }

    fn get_all_orientations(&self) -> Vec<Self> {
        let mut orientations = FxHashSet::default();

        let mut current = self.clone();
        for _ in 0..4 {
            orientations.insert(current.clone());
            orientations.insert(current.flip_horizontal());
            current = current.rotate_clockwise();
        }

        orientations.into_iter().collect()
    }
}

/// The region modelized as a grid.
struct Grid {
    g: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            g: vec![vec![0usize; width]; height],
            width,
            height,
        }
    }
    fn can_place(&self, cells: &Shape, start_x: usize, start_y: usize) -> bool {
        for &(dx, dy) in &cells.0 {
            let x = start_x + dx;
            let y = start_y + dy;

            if y >= self.height || x >= self.width {
                return false;
            }
            if self.g[y][x] != 0 {
                return false;
            }
        }
        true
    }

    fn place(&mut self, cells: &Shape, start_x: usize, start_y: usize, value: usize) {
        for &(dy, dx) in &cells.0 {
            let x = start_x + dx;
            let y = start_y + dy;
            self.g[y][x] = value;
        }
    }

    fn remove(&mut self, cells: &Shape, start_x: usize, start_y: usize) {
        for &(dy, dx) in &cells.0 {
            let x = start_x + dx;
            let y = start_y + dy;
            self.g[y][x] = 0;
        }
    }

    fn backtrack(&mut self, idx: usize, pieces: &Vec<Vec<Shape>>) -> bool {
        if idx == pieces.len() {
            return true;
        }

        for cells in &pieces[idx] {
            let max_x = cells.0.iter().map(|&(x, _)| x).max().unwrap_or(0);
            let max_y = cells.0.iter().map(|&(_, y)| y).max().unwrap_or(0);

            if max_x >= self.width {
                continue;
            }
            if max_y >= self.height {
                continue;
            }

            let max_start_x = self.width - max_y - 1;
            let max_start_y = self.height - max_x - 1;

            for start_y in 0..=max_start_y {
                for start_x in 0..=max_start_x {
                    if self.can_place(cells, start_x, start_y) {
                        self.place(cells, start_x, start_y, idx + 1);
                        if self.backtrack(idx + 1, pieces) {
                            return true;
                        }
                        self.remove(cells, start_x, start_y);
                    }
                }
            }
        }

        false
    }
}

struct Puzzle {
    shapes: FxHashMap<usize, Shape>,
    regions: Vec<(usize, usize, Vec<usize>)>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut shapes = FxHashMap::default();
        let mut regions = vec![];

        for s in data.split("\n\n") {
            // a present shape
            if s.contains('#') {
                let lines: Vec<_> = s.lines().collect();

                let idx = lines[0].strip_suffix(':').unwrap().parse().unwrap();

                let mut shape = Shape::new();
                for y in 0..3usize {
                    for x in 0..3usize {
                        if lines[y + 1].chars().nth(x).unwrap() == '#' {
                            shape.0.push((x, y));
                        }
                    }
                }

                shapes.insert(idx, shape);
            }

            // regions under the tree
            if s.contains('x') {
                for line in s.lines() {
                    let (size, counts) = line.split_once(": ").unwrap();
                    let (width, height) = size.split_once('x').unwrap();

                    regions.push((
                        width.parse().unwrap(),
                        height.parse().unwrap(),
                        counts
                            .split_ascii_whitespace()
                            .map(|s| s.parse().unwrap())
                            .collect(),
                    ));
                }
            }
        }

        Self { shapes, regions }
    }

    /// Try to fit pieces into a region (width x height) using backtracking.
    fn solve_region(&self, width: usize, height: usize, counts: &[usize]) -> bool {
        let mut pieces = vec![];

        for (shape_idx, &count) in counts.iter().enumerate() {
            if count == 0 {
                continue;
            }
            if let Some(shape) = self.shapes.get(&shape_idx) {
                let orientations = shape.get_all_orientations();
                let size = shape.0.len();
                for _ in 0..count {
                    pieces.push((size, orientations.clone()));
                }
            }
        }

        if pieces.is_empty() {
            return true;
        }

        let total_cells: usize = pieces.iter().map(|(s, _)| *s).sum();
        if total_cells > width * height {
            return false;
        }

        pieces.sort_by(|a, b| b.0.cmp(&a.0));
        let pieces_orientations: Vec<Vec<Shape>> = pieces.into_iter().map(|(_, o)| o).collect();

        let mut grid = Grid::new(width, height);
        grid.backtrack(0, &pieces_orientations)
    }

    fn part1(&self) -> usize {
        self.regions
            .par_iter()
            .map(|(width, height, counts)| usize::from(self.solve_region(*width, *height, counts)))
            .sum()
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, aoc::Christmas) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), aoc::CHRISTMAS)
}

/// # Panics
#[must_use]
pub fn solve_dummy(data: &str) -> (usize, aoc::Christmas) {
    let part1 = data
        .split("\n\n")
        .map(|s| {
            if s.contains('x') {
                s.lines()
                    .map(|line| {
                        let (size, counts) = line.split_once(": ").unwrap();
                        let (width, height) = size.split_once('x').unwrap();

                        let size: usize =
                            width.parse::<usize>().unwrap() * height.parse::<usize>().unwrap();

                        usize::from(
                            size >= 9 * counts
                                .split_ascii_whitespace()
                                .map(|s| s.parse::<usize>().unwrap())
                                .sum::<usize>(),
                        )
                    })
                    .sum()
            } else {
                0
            }
        })
        .sum();

    (part1, aoc::CHRISTMAS)
}

pub fn main() {
    let args = aoc::parse_args();
    if args.has_option("--dummy") {
        args.run(solve_dummy);
    } else {
        args.run(solve);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 3);
    }
}
