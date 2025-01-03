//! [Day 16: Reindeer Maze](https://adventofcode.com/2024/day/16)

use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::BinaryHeap;

use aoc::Coord;

const ZERO: Coord = Coord { x: 0, y: 0 };
const EAST: Coord = Coord { x: 1, y: 0 }; // starting direction

struct Cost1 {
    cost: u32,
    pos: Coord,
    dir: Coord,
}

impl Cost1 {
    const fn new(cost: u32, pos: Coord, dir: Coord) -> Self {
        Self { cost, pos, dir }
    }
}

impl Ord for Cost1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Cost1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cost1 {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Cost1 {}

struct Cost2 {
    cost: u32,
    pos: Coord,
    dir: Coord,
    path: Vec<Coord>,
}

impl Cost2 {
    const fn new(cost: u32, pos: Coord, dir: Coord, path: Vec<Coord>) -> Self {
        Self {
            cost,
            pos,
            dir,
            path,
        }
    }
}

impl Ord for Cost2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Cost2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cost2 {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Cost2 {}

struct Puzzle {
    start: Coord,
    end: Coord,
    maze: FxHashSet<Coord>,
    size: Coord,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            start: ZERO,
            end: ZERO,
            maze: FxHashSet::default(),
            size: ZERO,
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        for (y, line) in data.lines().enumerate() {
            let y = i32::try_from(y).unwrap();

            for (x, c) in line.chars().enumerate() {
                let x = i32::try_from(x).unwrap();
                self.size.x = x;

                if c == '#' {
                    continue;
                }

                if c == 'S' {
                    self.start = Coord::new(x, y);
                } else if c == 'E' {
                    self.end = Coord::new(x, y);
                }
                self.maze.insert(Coord { x, y });
            }

            self.size.y = y;
        }
    }

    #[cfg(feature = "anim")]
    fn show_maze(&self, path: &[Coord], n: u32) {
        const SCALE: u32 = 2;

        let width = self.size.x as u32 + 1;
        let height = self.size.y as u32 + 1;

        let mut imgbuf = image::ImageBuffer::new(width * SCALE, height * SCALE);

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let r = (0.3 * x as f32) as u8;
            let b = (0.3 * y as f32) as u8;
            *pixel = image::Rgb([r, 0, b]);
        }
        for y in 0..=self.size.y {
            for x in 0..=self.size.x {
                let pos = Coord { x, y };

                let c = image::Rgb(if pos == self.start {
                    [0, 255, 0]
                } else if pos == self.end {
                    [255, 255, 0]
                } else if path.contains(&pos) {
                    [0, 0, 255]
                } else if self.maze.contains(&pos) {
                    continue;
                } else {
                    [0, 0, 0]
                });

                let x = x as u32 * SCALE;
                let y = y as u32 * SCALE;

                for k in 0..(SCALE * SCALE) {
                    let pixel = imgbuf.get_pixel_mut(x + k % SCALE, y + k / SCALE);
                    *pixel = c
                }
            }

            imgbuf.save(format!("frame{n:05}.png")).unwrap();
        }
    }

    #[allow(dead_code)]
    fn show_maze_ascii(&self, path: &[Coord]) {
        for y in 0..=self.size.y {
            for x in 0..=self.size.x {
                let pos = Coord { x, y };
                let c = if pos == self.start {
                    'S'
                } else if pos == self.end {
                    'E'
                } else if path.contains(&pos) {
                    'O'
                } else if self.maze.contains(&pos) {
                    '.'
                } else {
                    '#'
                };

                print!("{c}");
            }
            println!();
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut seen = FxHashSet::default();
        let mut heap = BinaryHeap::new();

        heap.push(Cost1::new(0, self.start, EAST));

        while let Some(Cost1 { cost, pos, dir }) = heap.pop() {
            seen.insert((pos, dir));

            let counterclockwise = Coord::new(dir.y, -dir.x);
            let clockwise = Coord::new(-dir.y, dir.x);

            for (new_cost, new_pos, new_dir) in [
                (cost + 1, pos + dir, dir), // advance in same direction
                (cost + 1001, pos + counterclockwise, counterclockwise), // turn then move
                (cost + 1001, pos + clockwise, clockwise), // turn then move
            ] {
                if new_pos == self.end {
                    return new_cost;
                }
                if self.maze.contains(&new_pos) && !seen.contains(&(new_pos, dir)) {
                    heap.push(Cost1::new(new_cost, new_pos, new_dir));
                }
            }
        }

        0
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let mut heap = BinaryHeap::new();
        let mut costs = FxHashMap::default();
        let mut best_path_tiles: FxHashSet<Coord> = FxHashSet::default();

        let mut best_cost = u32::MAX;

        #[cfg(feature = "anim")]
        let mut frames = 0;

        heap.push(Cost2::new(0, self.start, EAST, [self.start].to_vec()));
        while let Some(Cost2 {
            cost,
            pos,
            dir,
            path: tiles,
        }) = heap.pop()
        {
            if pos == self.end {
                best_cost = best_cost.min(cost);
                if best_cost == cost {
                    best_path_tiles.extend(&tiles);

                    #[cfg(feature = "anim")]
                    {
                        self.show_maze(&tiles, frames);
                        frames += 1;
                    }
                }
            }

            let counterclockwise = Coord::new(dir.y, -dir.x);
            let clockwise = Coord::new(-dir.y, dir.x);

            for (new_cost, new_pos, new_dir) in [
                (cost + 1, pos + dir, dir),
                (cost + 1001, pos + counterclockwise, counterclockwise),
                (cost + 1001, pos + clockwise, clockwise),
            ] {
                if self.maze.contains(&new_pos)
                    && costs.get(&(new_pos, new_dir)).copied().unwrap_or(u32::MAX) >= new_cost
                {
                    costs.insert((new_pos, new_dir), new_cost);
                    let mut tiles = tiles.clone();
                    tiles.push(new_pos);
                    heap.push(Cost2::new(new_cost, new_pos, new_dir, tiles));
                }
            }
        }

        best_path_tiles.len()
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
        let data = aoc::load_input_data("sample_1.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part1(), 7036);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        let data = aoc::load_input_data("sample_3.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part1(), 11048);
    }

    #[test]
    fn test03() {
        let mut puzzle = Puzzle::new();
        let data = aoc::load_input_data("sample_1.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part2(), 45);
    }

    #[test]
    fn test04() {
        let mut puzzle = Puzzle::new();
        let data = aoc::load_input_data("sample_3.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part2(), 64);
    }
}
