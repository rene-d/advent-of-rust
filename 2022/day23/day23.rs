//! [Day 23: Unstable Diffusion](https://adventofcode.com/2022/day/23)

use clap::Parser;
use std::collections::{HashMap, HashSet};

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Puzzle {
    elves: HashSet<(i32, i32)>,
    round: usize,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            elves: HashSet::new(),
            round: 0,
        }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for (y, line) in data.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let x = x as i32;
                let y = y as i32;
                if c == '#' {
                    self.elves.insert((x, y));
                }
            }
        }
    }

    fn propose_move(&self, x: i32, y: i32) -> (i32, i32) {
        // if elf at (x,y) has no adjacent elf, do not move
        let no_neighbor = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .all(|(dx, dy)| !self.elves.contains(&(x + dx, y + dy)));
        if no_neighbor {
            return (x, y);
        }

        // at each round, the first considered direction rolls
        for direction in self.round..(self.round + 4) {
            match direction % 4 {
                0 => {
                    // north
                    let no_neighbor = (-1..=1).all(|dx| !self.elves.contains(&(x + dx, y - 1)));
                    if no_neighbor {
                        // north is free: propose to move north
                        return (x, y - 1);
                    }
                }
                1 => {
                    // south
                    let no_neighbor = (-1..=1).all(|dx| !self.elves.contains(&(x + dx, y + 1)));
                    if no_neighbor {
                        return (x, y + 1);
                    }
                }
                2 => {
                    // west
                    let no_neighbor = (-1..=1).all(|dy| !self.elves.contains(&(x - 1, y + dy)));
                    if no_neighbor {
                        return (x - 1, y);
                    }
                }
                3 => {
                    // east
                    let no_neighbor = (-1..=1).all(|dy| !self.elves.contains(&(x + 1, y + dy)));
                    if no_neighbor {
                        return (x + 1, y);
                    }
                }
                _ => (),
            }
        }

        (x, y)
    }

    fn move_elves(&mut self) -> bool {
        let mut counter = HashMap::new();
        let mut proposed_moves = Vec::new();

        for (x, y) in &self.elves {
            let (new_x, new_y) = self.propose_move(*x, *y);

            // save move: old, new
            proposed_moves.push((*x, *y, new_x, new_y));

            // count destination cells
            *counter.entry((new_x, new_y)).or_insert(0) += 1;
        }

        let mut moved = false;
        self.elves.clear();
        for (x, y, new_x, new_y) in proposed_moves {
            // if two or more elves propose to move on the same cell, they won't move
            if counter[&(new_x, new_y)] <= 1 {
                self.elves.insert((new_x, new_y));
                if x != new_x || y != new_y {
                    moved = true;
                }
            } else {
                self.elves.insert((x, y));
            }
        }

        self.round += 1;
        moved
    }

    // Solves part one
    fn part1(&mut self) -> usize {
        for _ in 0..10 {
            self.move_elves();
        }

        let x_min = self.elves.iter().map(|(x, _)| x).min().unwrap();
        let x_max = self.elves.iter().map(|(x, _)| x).max().unwrap();
        let y_min = self.elves.iter().map(|(_, y)| y).min().unwrap();
        let y_max = self.elves.iter().map(|(_, y)| y).max().unwrap();

        ((x_max - x_min + 1) * (y_max - y_min + 1)) as usize - self.elves.len()
    }

    // Solve part two
    fn part2(&mut self) -> usize {
        while self.move_elves() && self.round < 2000 {}
        self.round
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
    assert_eq!(puzzle.part1(), 110);
    assert_eq!(puzzle.part2(), 20);
}
