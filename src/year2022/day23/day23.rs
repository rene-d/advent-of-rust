//! [Day 23: Unstable Diffusion](https://adventofcode.com/2022/day/23)

use rustc_hash::{FxHashMap, FxHashSet};

struct Puzzle {
    elves: FxHashSet<(i32, i32)>,
    round: usize,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut elves = FxHashSet::default();

        for (y, line) in (0..).zip(data.lines()) {
            for (x, c) in (0..).zip(line.chars()) {
                if c == '#' {
                    elves.insert((x, y));
                }
            }
        }

        Self { elves, round: 0 }
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
        let mut counter = FxHashMap::default();
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

        usize::try_from((x_max - x_min + 1) * (y_max - y_min + 1)).unwrap() - self.elves.len()
    }

    // Solve part two
    fn part2(&mut self) -> usize {
        while self.move_elves() && self.round < 2000 {}
        self.round
    }
}

#[must_use]
pub fn solve(data: &str) -> (usize, usize) {
    let mut puzzle = Puzzle::new(data);
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
        let mut puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 110);
        assert_eq!(puzzle.part2(), 20);
    }
}
