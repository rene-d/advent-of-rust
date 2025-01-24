//! [Day 17: Conway Cubes](https://adventofcode.com/2020/day/17)

use rustc_hash::FxHashSet;

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

    fn cycle(cubes: &FxHashSet<Self>) -> FxHashSet<Self> {
        let mut next_cubes = FxHashSet::default();
        let mut tested = FxHashSet::default();

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
    cubes: FxHashSet<Cube>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut cubes = FxHashSet::default();

        for (y, line) in (0..).zip(data.lines()) {
            for (x, c) in (0..).zip(line.chars()) {
                if c == '#' {
                    cubes.insert(Cube::new_3d(x, y, 0));
                }
            }
        }

        Self { cubes }
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

/// # Panics
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
    fn test_part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 112);
    }

    #[test]
    fn test_part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 848);
    }
}
