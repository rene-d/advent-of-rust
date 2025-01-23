//! [Day 18: Boiling Boulders](https://adventofcode.com/2022/day/18)

use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

struct Puzzle {
    cubes: FxHashSet<(i32, i32, i32)>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut cubes = FxHashSet::default();

        for line in data.lines() {
            let mut xyz = line.split(',').map(|a| a.parse::<i32>().unwrap());

            let x = xyz.next().unwrap();
            let y = xyz.next().unwrap();
            let z = xyz.next().unwrap();

            cubes.insert((x, y, z));
        }

        Self { cubes }
    }

    // Solves part one
    fn part1(&self) -> usize {
        let mut faces = FxHashMap::default();

        // each 1x1x1 cube has - obviously - six faces: each face is identified by its center
        // to simplify computation, we use pair coordinates for the cubes and odd for the center of faces
        // (in other words, cubes become 2x2x2 ones)

        for (x, y, z) in &self.cubes {
            for (dx, dy, dz) in [
                (1, 1, 0),
                (1, 0, 1),
                (0, 1, 1),
                (1, 1, 2),
                (1, 2, 1),
                (2, 1, 1),
            ] {
                let center = (2 * x + dx, 2 * y + dy, 2 * z + dz);

                *faces.entry(center).or_insert(0) += 1;
            }
        }

        faces.values().filter(|count| **count == 1).count()
    }

    // Solve part two
    fn part2(&self) -> usize {
        // main idea: do a DFS between a known empty cell of the englobing parallelepiped to find
        // all accessible (non trapped) empty cells

        let x_min = self.cubes.iter().map(|a| a.0).min().unwrap() - 1;
        let x_max = self.cubes.iter().map(|a| a.0).max().unwrap() + 1;

        let y_min = self.cubes.iter().map(|a| a.1).min().unwrap() - 1;
        let y_max = self.cubes.iter().map(|a| a.1).max().unwrap() + 1;

        let z_min = self.cubes.iter().map(|a| a.2).min().unwrap() - 1;
        let z_max = self.cubes.iter().map(|a| a.2).max().unwrap() + 1;

        let mut air = FxHashSet::default();
        let mut q = VecDeque::new();

        // first empty cell (but we could start with any of the surrounding cells)
        q.push_front((x_min, y_min, x_min));

        while let Some(cell) = q.pop_back() {
            for d in [
                (0, 0, 1),
                (0, 1, 0),
                (1, 0, 0),
                (0, 0, -1),
                (0, -1, 0),
                (-1, 0, 0),
            ] {
                let adj_cell = (cell.0 + d.0, cell.1 + d.1, cell.2 + d.2);

                // inbound?
                if !(x_min <= adj_cell.0
                    && adj_cell.0 <= x_max
                    && y_min <= adj_cell.1
                    && adj_cell.1 <= y_max
                    && z_min <= adj_cell.2
                    && adj_cell.2 <= z_max)
                {
                    continue;
                }

                // a cube (lava droplet)
                if self.cubes.contains(&adj_cell) {
                    continue;
                }

                // empty cell (air) - acts as a 'visited' guard for the DFS
                if air.contains(&adj_cell) {
                    continue;
                }

                air.insert(adj_cell);
                q.push_front(adj_cell);
            }
        }

        // same algo as part1 but applied on empty cells
        let mut faces = FxHashMap::default();

        for (x, y, z) in &air {
            for (dx, dy, dz) in [
                (1, 1, 0),
                (1, 0, 1),
                (0, 1, 1),
                (1, 1, 2),
                (1, 2, 1),
                (2, 1, 1),
            ] {
                let center = (2 * x + dx, 2 * y + dy, 2 * z + dz);

                *faces.entry(center).or_insert(0) += 1;
            }
        }

        // we have to substract the faces of the big surrounding parallelepiped

        let sx = x_max - x_min + 1;
        let sy = y_max - y_min + 1;
        let sz = z_max - z_min + 1;
        let surrounding_faces = usize::try_from((sx * sy + sy * sz + sz * sx) * 2).unwrap();

        faces.values().filter(|count| **count == 1).count() - surrounding_faces
    }
}

#[must_use]
pub fn solve(data: &str) -> (usize, usize) {
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
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 64);
        assert_eq!(puzzle.part2(), 58);
    }
}
