//! [Day 12: Garden Groups](https://adventofcode.com/2024/day/12)

use aoc::{Coord, Direction};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

type Grid = aoc::Grid<u8>;

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let grid = Grid::parse(data);

    let mut standard_price = 0;
    let mut discount_price = 0;

    let mut seen = FxHashSet::default();

    for (xy, &plant) in &grid {
        let mut area: u32 = 0;
        let mut perimeter: u32 = 0;
        let mut sides = 0;
        let mut queue = VecDeque::new();
        let mut side_fences: FxHashMap<Direction, FxHashSet<Coord>> = FxHashMap::default();

        queue.push_back(xy);

        while let Some(c) = queue.pop_front() {
            if seen.contains(&c) {
                continue;
            }
            seen.insert(c);

            area += 1;

            for (d, neigh) in grid.iter_directions_all(c) {
                if let Some(neigh) = neigh
                    && grid[neigh] == plant {
                        // bfs to compute area of current plant
                        queue.push_back(neigh);
                        continue;
                    }

                // fence: increase perimter
                perimeter += 1;

                // (part 2)
                side_fences.entry(d).or_default().insert(c);
            }
        }

        // println!("{xy:?} {plant} {side_fences:?}");

        for vs in side_fences.values() {
            let mut seen_sides = FxHashSet::default();

            for &p in vs {
                if seen_sides.contains(&p) {
                    continue;
                }

                sides += 1;

                let mut queue_sides = VecDeque::new();
                queue_sides.push_back(p);

                while let Some(c) = queue_sides.pop_front() {
                    if seen_sides.contains(&c) {
                        continue;
                    }
                    seen_sides.insert(c);

                    grid.iter_directions(c)
                        .filter(|(_, a)| vs.contains(a))
                        .for_each(|(_, a)| queue_sides.push_back(a));
                }
            }
        }

        standard_price += area * perimeter;
        discount_price += area * sides;
    }

    (standard_price, discount_price)
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// Test from answers input
#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_1: &str = include_str!("sample_1.txt");
    const SAMPLE_3: &str = include_str!("sample_3.txt");
    const SAMPLE_4: &str = include_str!("sample_4.txt");
    const SAMPLE_6: &str = include_str!("sample_6.txt");
    const SAMPLE_7: &str = include_str!("sample_7.txt");

    #[test]
    fn test01() {
        let answers = solve(SAMPLE_1);
        assert_eq!(answers.0, 140);
        assert_eq!(answers.1, 80);
    }

    #[test]
    fn test02() {
        let answers = solve(SAMPLE_3);
        assert_eq!(answers.0, 772);
        assert_eq!(answers.1, 436);
    }

    #[test]
    fn test03() {
        let answers = solve(SAMPLE_4);
        assert_eq!(answers.0, 1930);
        assert_eq!(answers.1, 1206);
    }

    #[test]
    fn test04() {
        let answers = solve(SAMPLE_6);
        assert_eq!(answers.1, 236);
    }

    #[test]
    fn test05() {
        let answers = solve(SAMPLE_7);
        assert_eq!(answers.1, 368);
    }
}
