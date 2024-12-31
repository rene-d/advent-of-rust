//! [Day 17: Conway Cubes](https://adventofcode.com/2020/day/17)

use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Cube {
    is_hyper: bool,
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Cube {
    const fn new_3d(x: i32, y: i32, z: i32) -> Self {
        Self {
            is_hyper: false,
            x,
            y,
            z,
            w: 0,
        }
    }

    const fn make_4(&self) -> Self {
        Self {
            is_hyper: true,
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }

    fn neighbors(&self) -> impl Iterator<Item = Self> + '_ {
        (-1..=1).flat_map(move |dx| {
            (-1..=1).flat_map(move |dy| {
                (-1..=1).flat_map(move |dz| {
                    let fourth_dim = if self.is_hyper { -1..=1 } else { 0..=0 };
                    fourth_dim.filter_map(move |dw| {
                        if dx != 0 || dy != 0 || dz != 0 || (self.is_hyper && dw != 0) {
                            Some(Self {
                                is_hyper: self.is_hyper,
                                x: self.x + dx,
                                y: self.y + dy,
                                z: self.z + dz,
                                w: self.w + dw,
                            })
                        } else {
                            None
                        }
                    })
                })
            })
        })
    }

    fn cycle(cubes: &HashSet<Self>) -> HashSet<Self> {
        let mut next_cubes = HashSet::new();
        let mut tested = HashSet::new();

        for cube in cubes {
            let mut actives = 0;

            for c in cube.neighbors() {
                if cubes.contains(&c) {
                    actives += 1;
                } else if tested.insert(c.clone()) {
                    // c is inactive (and never tested)

                    if c.neighbors().filter(|cc| cubes.contains(cc)).count() == 3 {
                        //  becomes active since it has exactly 3 active neighbors
                        next_cubes.insert(c);
                    }
                }
            }

            if actives == 2 || actives == 3 {
                // active with exactly 2 or 3 active neighbors, the cube remains active
                next_cubes.insert(cube.clone());
            }
        }

        next_cubes
    }
}

struct Puzzle {
    cubes: HashSet<Cube>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            cubes: HashSet::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap_or_else(|_| {
            eprintln!("cannot read input file {path}");
            std::process::exit(1);
        });

        for (y, line) in (0..).zip(data.lines()) {
            for (x, c) in (0..).zip(line.chars()) {
                if c == '#' {
                    self.cubes.insert(Cube::new_3d(x, y, 0));
                }
            }
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let mut cubes = self.cubes.clone();
        for _ in 0..6 {
            cubes = Cube::cycle(&cubes);
        }
        cubes.len()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let mut hypercubes = self.cubes.iter().map(Cube::make_4).collect();

        for _ in 0..6 {
            hypercubes = Cube::cycle(&hypercubes);
        }
        hypercubes.len()
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
    fn test_part1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 112);
    }

    #[test]
    fn test_part2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 848);
    }
}