//! [Day 24: Planet of Discord](https://adventofcode.com/2019/day/24)

use aoc::Coord;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Clone)]
struct Bugs(aoc::Grid<u8>);

impl Bugs {
    const BUG: u8 = b'#';
    const EMPTY: u8 = b'.';

    fn evolve(&self) -> Self {
        let mut new_bugs = self.clone();
        for (pos, &c) in &self.0 {
            let adj = self
                .0
                .iter_directions(pos)
                .filter(|(_, neigh)| self.0[*neigh] == b'#')
                .count();

            if c == Self::BUG {
                if adj != 1 {
                    new_bugs.0[pos] = Self::EMPTY;
                }
            } else if adj == 1 || adj == 2 {
                new_bugs.0[pos] = Self::BUG;
            }
        }
        new_bugs
    }

    fn biodiversity_rating(&self) -> u32 {
        let mut rating = 0;
        for (pos, &c) in &self.0 {
            if c == b'#' {
                rating |= 1 << (pos.y * 5 + pos.x);
            }
        }
        rating
    }
}

#[derive(PartialEq, Eq, Hash)]
struct FoldedBug {
    pos: Coord,
    level: i32,
}

impl FoldedBug {
    const fn new(x: i32, y: i32, level: i32) -> Self {
        Self {
            pos: Coord::new(x, y),
            level,
        }
    }

    fn neighbors(&self) -> Vec<Self> {
        let mut n = Vec::new();

        // same level
        for delta in [Coord::UP, Coord::DOWN, Coord::LEFT, Coord::RIGHT] {
            let np = self.pos + delta;
            if (0..5).contains(&np.x) && (0..5).contains(&np.y) && !(np.x == 2 && np.y == 2) {
                n.push(Self::new(np.x, np.y, self.level));
            }
        }

        let (x, y) = (self.pos.x, self.pos.y);

        match (x, y) {
            // inner
            (2, 1) => n.extend((0..5).map(|x| Self::new(x, 0, self.level + 1))),
            (2, 3) => n.extend((0..5).map(|x| Self::new(x, 4, self.level + 1))),
            (1, 2) => n.extend((0..5).map(|y| Self::new(0, y, self.level + 1))),
            (3, 2) => n.extend((0..5).map(|y| Self::new(4, y, self.level + 1))),
            _ => (),
        }
        match x {
            // outer
            0 => n.push(Self::new(1, 2, self.level - 1)),
            4 => n.push(Self::new(3, 2, self.level - 1)),
            _ => {}
        };

        match y {
            0 => n.push(Self::new(2, 1, self.level - 1)),
            4 => n.push(Self::new(2, 3, self.level - 1)),
            _ => {}
        };

        n
    }
}

struct FoldedBugs(FxHashSet<FoldedBug>);

impl FoldedBugs {
    fn new(bugs: &Bugs) -> Self {
        Self(
            bugs.0
                .iter()
                .filter_map(|(pos, &c)| (c == Bugs::BUG).then_some(FoldedBug::new(pos.x, pos.y, 0)))
                .collect(),
        )
    }

    fn evolve(&mut self) {
        let mut bugs2: FxHashSet<FoldedBug> = FxHashSet::default();

        let mut adjs = FxHashMap::<FoldedBug, u32>::default();
        for bug in &self.0 {
            for adj in bug.neighbors() {
                *adjs.entry(adj).or_default() += 1;
            }
        }

        for (a, n) in adjs {
            if self.0.contains(&a) {
                if n == 1 {
                    bugs2.insert(a);
                }
            } else if n == 1 || n == 2 {
                bugs2.insert(a);
            }
        }
        self.0 = bugs2;
    }
}

struct Puzzle {
    bugs: Bugs,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        Self {
            bugs: Bugs(aoc::Grid::<u8>::parse(data)),
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut seen = FxHashSet::default();
        let mut bugs = self.bugs.clone();
        loop {
            let rating = bugs.biodiversity_rating();
            if !seen.insert(rating) {
                break rating;
            }
            bugs = bugs.evolve();
        }
    }

    fn p2(&self, minutes: u32) -> usize {
        let mut bugs = FoldedBugs::new(&self.bugs);
        for _ in 0..minutes {
            bugs.evolve();
        }
        bugs.0.len()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        self.p2(200)
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, usize) {
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
        assert_eq!(puzzle.part1(), 2129920);
    }

    #[test]
    fn part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.p2(10), 99);
    }
}
