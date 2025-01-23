//! [Day 13: A Maze of Twisty Little Cubicles](https://adventofcode.com/2016/day/13)

use rustc_hash::FxHashSet;
use std::collections::VecDeque;

const fn is_wall(x: u32, y: u32, designer_number: u32) -> bool {
    let v = x * x + 3 * x + 2 * x * y + y + y * y + designer_number;
    let v = count_ones(v);
    v & 1 == 1
}

const fn count_ones(value: u32) -> u32 {
    let mut count = 0;
    let mut value = value;

    while value != 0 {
        count += 1;
        value &= value - 1;
    }

    count
}

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn iter(x: u32, y: u32) -> impl Iterator<Item = (u32, u32)> {
        [Self::North, Self::East, Self::South, Self::West]
            .iter()
            .filter_map(move |d| {
                if d == &Self::North && y > 0 {
                    Some((x, y - 1))
                } else if d == &Self::South {
                    Some((x, y + 1))
                } else if d == &Self::East {
                    Some((x + 1, y))
                } else if d == &Self::West && x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            })
    }
}

fn bfs(designer_number: u32, start: (u32, u32), end: (u32, u32), max_moves: usize) -> usize {
    let mut seen = FxHashSet::default();
    let mut q = VecDeque::new();

    q.push_front((start, 0));

    while let Some(((x, y), cost)) = q.pop_back() {
        // stop conditions
        if cost >= max_moves {
            // part 2
            return seen.len();
        } else if (x, y) == end {
            // part 1
            return cost;
        }

        for (x, y) in Direction::iter(x, y) {
            if !is_wall(x, y, designer_number) && !seen.contains(&(x, y)) {
                seen.insert((x, y));
                q.push_front(((x, y), cost + 1));
            }
        }
    }

    0
}

struct Puzzle {
    designer_number: u32,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            designer_number: data.trim().parse().unwrap(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        bfs(self.designer_number, (1, 1), (31, 39), usize::MAX)
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        bfs(self.designer_number, (1, 1), (u32::MAX, u32::MAX), 50)
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

    #[test]
    fn test01() {
        assert_eq!(bfs(10, (1, 1), (1, 1), usize::MAX), 0);
        assert_eq!(bfs(10, (1, 1), (7, 4), usize::MAX), 11);
    }

    #[test]
    fn test_count_ones() {
        assert_eq!(count_ones(0b0011_0011_0011), 6);
        assert_eq!(count_ones(0b1100_1100_1100), 6);
        assert_eq!(count_ones(0b111), 3);
        assert_eq!(count_ones(0b1), 1);
        assert_eq!(count_ones(0b0), 0);
    }
}
