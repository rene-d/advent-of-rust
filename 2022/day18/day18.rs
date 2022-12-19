//! [Day 18: Boiling Boulders](https://adventofcode.com/2022/day/18)

use clap::Parser;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Puzzle {
    cubes: HashSet<(i32, i32, i32)>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            cubes: HashSet::new(),
        }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();
        let lines = data.split('\n').collect::<Vec<_>>();

        for line in lines {
            if !line.is_empty() {
                let mut xyz = line.split(',').map(|a| a.parse::<i32>().unwrap());

                let x = xyz.next().unwrap();
                let y = xyz.next().unwrap();
                let z = xyz.next().unwrap();

                self.cubes.insert((x, y, z));
            }
        }

        // println!("{:?}", self.cubes.iter().map(|x| x.2).min());
        // println!("{:?}", self.cubes.iter().map(|x| x.2).max());
    }

    // Solves part one
    fn part1(&self) -> usize {
        let mut faces = HashMap::new();

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

        let mut air = HashSet::new();
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
        let mut faces = HashMap::new();

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
        let surrounding_faces = ((sx * sy + sy * sz + sz * sx) * 2) as usize;

        faces.values().filter(|count| **count == 1).count() - surrounding_faces
    }
}

/// main function
fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.path);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test.txt");
    assert_eq!(puzzle.part1(), 64);
    assert_eq!(puzzle.part2(), 58);
}
