//! [Day 24: Lobby Layout](https://adventofcode.com/2020/day/24)

use aoc::Coord;
use rustc_hash::{FxHashMap, FxHashSet};

struct Walk(FxHashMap<Coord, bool>);

impl Walk {
    fn new() -> Self {
        Self(FxHashMap::default())
    }

    fn tiles(&self) -> FxHashSet<Coord> {
        self.0
            .iter()
            .filter(|(_, v)| **v)
            .map(|(k, _)| k)
            .copied()
            .collect()
    }

    fn walk(&mut self, line: &str) {
        let mut pos = Coord::ZERO;

        let mut it = line.chars();
        while let Some(ch) = it.next() {
            pos += match ch {
                'w' => Coord::WEST * 2,
                'e' => Coord::EAST * 2,
                'n' => match it.next().unwrap() {
                    'w' => Coord::NORTH_WEST,
                    'e' => Coord::NORTH_EAST,
                    _ => panic!(),
                },
                's' => match it.next().unwrap() {
                    'w' => Coord::SOUTH_WEST,
                    'e' => Coord::SOUTH_EAST,
                    _ => panic!(),
                },
                _ => break,
            };
        }

        *self.0.entry(pos).or_default() ^= true;
    }
}

#[derive(Debug)]
struct Puzzle {
    tiles: FxHashSet<Coord>,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        let mut walk = Walk::new();

        for line in data.lines() {
            walk.walk(line);
        }

        Self {
            tiles: walk.tiles(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.tiles.len()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let mut tiles = self.tiles.clone();

        let neighbors = [
            Coord::ZERO,
            Coord::NORTH_EAST,
            Coord::EAST * 2,
            Coord::SOUTH_EAST,
            Coord::NORTH_WEST,
            Coord::WEST * 2,
            Coord::SOUTH_WEST,
        ];

        // Hexagonal Conway's game of life

        for _ in 0..100 {
            let mut new_tiles = FxHashSet::default();
            let mut seen = FxHashSet::default();

            // for every visited tiles
            for &black in &tiles {
                for dxy in neighbors {
                    let xy = black + dxy;

                    // but only once
                    if !seen.insert(xy) {
                        continue;
                    }

                    let is_black = tiles.contains(&xy);

                    let black_neighbors = neighbors[1..]
                        .iter()
                        .filter(|&&nxy| tiles.contains(&(xy + nxy)))
                        .count();

                    // evolve rule
                    let new_is_black = if is_black && (black_neighbors == 0 || black_neighbors > 2)
                    {
                        false
                    } else if !is_black && black_neighbors == 2 {
                        true
                    } else {
                        is_black
                    };

                    if new_is_black {
                        new_tiles.insert(xy);
                    }
                }
            }

            tiles = new_tiles;
        }

        tiles.len()
    }
}

/// # Panics
/// over malformed input
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
    fn part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 10);
    }

    #[test]
    fn part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 2208);
    }
}
