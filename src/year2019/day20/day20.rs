//! [Day 20: Donut Maze](https://adventofcode.com/2019/day/20)

use aoc::Coord;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

type Portal = [u8; 2];

const AA: Portal = [b'A', b'A'];
const ZZ: Portal = [b'Z', b'Z'];

struct Puzzle {
    maze: FxHashSet<Coord>,
    width: i32,
    height: i32,
    portals: FxHashMap<Portal, Vec<Coord>>,
    portals_pos: FxHashMap<Coord, Portal>,
    start: Coord,
    end: Coord,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        let grid = aoc::Grid::<u8>::parse(data);
        let mut letters = FxHashMap::default();

        let mut maze = FxHashSet::default();

        for (pos, c) in &grid {
            match c {
                b'.' => {
                    maze.insert(pos);
                }
                b'A'..=b'Z' => {
                    letters.insert(pos, c);
                }
                b'#' | b' ' => {}
                _ => panic!("unexpected char: {c}"),
            };
        }

        // map of coord/name
        let mut portals_pos = FxHashMap::default();
        for (&pos, &&c) in &letters {
            let right = pos + Coord::RIGHT;
            let right2 = pos + Coord::RIGHT * 2;
            if let Some(&&right_c) = letters.get(&right) {
                if maze.contains(&right2) {
                    portals_pos.insert(right2, [c, right_c]);
                }
            }

            let left = pos + Coord::LEFT;
            let left2 = pos + Coord::LEFT * 2;
            if let Some(&&left_c) = letters.get(&left) {
                if maze.contains(&left2) {
                    portals_pos.insert(left2, [left_c, c]);
                }
            }

            let up = pos + Coord::UP;
            let up2 = pos + Coord::UP * 2;
            if let Some(&&up_c) = letters.get(&up) {
                if maze.contains(&up2) {
                    portals_pos.insert(up2, [up_c, c]);
                }
            }

            let down = pos + Coord::DOWN;
            let down2 = pos + Coord::DOWN * 2;
            if let Some(&&down_c) = letters.get(&down) {
                if maze.contains(&down2) {
                    portals_pos.insert(down2, [c, down_c]);
                }
            }
        }

        // map name/list of coords
        let mut portals: FxHashMap<Portal, Vec<Coord>> = FxHashMap::default();
        for (&k, &v) in &portals_pos {
            portals.entry(v).or_default().push(k);
        }

        let start = *portals.get(&AA).unwrap().first().unwrap();
        let end = *portals.get(&ZZ).unwrap().first().unwrap();

        Self {
            maze,
            width: grid.width(),
            height: grid.height(),
            portals,
            portals_pos,
            start,
            end,
        }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        let mut teleports = FxHashMap::default();

        for v in self.portals.values() {
            if v.len() == 2 {
                teleports.insert(v[0], v[1]);
                teleports.insert(v[1], v[0]);
            }
        }

        // bfs
        let mut seen = FxHashSet::default();
        let mut queue = VecDeque::new();
        queue.push_back((self.start, 0));

        while let Some((xy, n)) = queue.pop_front() {
            if xy == self.end {
                return n;
            }

            for dxy in [Coord::UP, Coord::DOWN, Coord::LEFT, Coord::RIGHT] {
                let nxy = xy + dxy;

                if self.maze.contains(&nxy) && seen.insert(nxy) {
                    queue.push_back((nxy, n + 1));
                }
            }

            if let Some(&t) = teleports.get(&xy) {
                if seen.insert(t) {
                    queue.push_back((t, n + 1));
                }
            }
        }

        0
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        // init inner and outer portals
        let mut inner_portals = FxHashMap::default();
        let mut inner_portals_inv = FxHashMap::default();
        let mut outer_portals = FxHashMap::default();
        let mut outer_portals_inv = FxHashMap::default();

        for (pos, v) in &self.portals_pos {
            if v == &AA || v == &ZZ {
                continue;
            }

            if pos.y == 2 || pos.x == 2 || pos.x == self.width - 3 || pos.y == self.height - 3 {
                outer_portals.insert(pos, v);
                outer_portals_inv.insert(v, pos);
            } else {
                inner_portals.insert(pos, v);
                inner_portals_inv.insert(v, pos);
            }
        }

        // bfs
        let mut seen = FxHashSet::default();
        let mut queue = VecDeque::new();
        queue.push_back((self.start, 0, 0, [0, 0]));

        while let Some((xy, n, level, portal)) = queue.pop_front() {
            if !seen.insert((xy, level)) {
                continue;
            }

            if xy == self.end && level == 0 {
                return n;
            }

            for dxy in [Coord::UP, Coord::DOWN, Coord::LEFT, Coord::RIGHT] {
                let nxy = xy + dxy;

                if level > 0 && (nxy == self.start || nxy == self.end) {
                    continue;
                }

                if self.maze.contains(&nxy) {
                    queue.push_back((nxy, n + 1, level, portal));
                }
            }

            if let Some(p) = inner_portals.get(&xy) {
                if **p != portal {
                    let t = outer_portals_inv[p];
                    queue.push_back((*t, n + 1, level + 1, **p));
                }
            }

            if level > 0 {
                if let Some(p) = outer_portals.get(&xy) {
                    if **p != portal {
                        let t = inner_portals_inv[p];
                        queue.push_back((*t, n + 1, level - 1, **p));
                    }
                }
            }
        }

        0
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u64, u64) {
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

    const SAMPLE_1: &str = include_str!("sample_1.txt");
    const SAMPLE_2: &str = include_str!("sample_2.txt");
    const SAMPLE_3: &str = include_str!("sample_3.txt");

    #[test]
    fn part1() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part1(), 23);
        assert_eq!(puzzle.part2(), 26);
    }

    #[test]
    fn part2() {
        let puzzle = Puzzle::new(SAMPLE_2);
        assert_eq!(puzzle.part1(), 58);
    }

    #[test]
    fn part3() {
        let puzzle = Puzzle::new(SAMPLE_3);
        assert_eq!(puzzle.part2(), 396);
    }
}
