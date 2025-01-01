//! [Day 12: Garden Groups](https://adventofcode.com/2024/day/12)

use aoc::{Coord, Direction};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

type Grid = aoc::Grid<char>;

struct Puzzle {
    standard_price: u32,
    discount_price: u32,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            standard_price: 0,
            discount_price: 0,
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        let grid = Grid::parse(data);

        self.solve(&grid);
    }

    fn solve(&mut self, grid: &Grid) {
        self.standard_price = 0;
        self.discount_price = 0;

        let mut seen = FxHashSet::default();

        for (xy, &plant) in grid {
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
                    if let Some(neigh) = neigh {
                        if grid[neigh] == plant {
                            // bfs to compute area of current plant
                            queue.push_back(neigh);
                            continue;
                        }
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

            self.standard_price += area * perimeter;
            self.discount_price += area * sides;
        }
    }

    /// Solve part one.
    const fn part1(&self) -> u32 {
        self.standard_price
    }

    /// Solve part two.
    const fn part2(&self) -> u32 {
        self.discount_price
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        let data = aoc::load_input_data("sample_1.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part1(), 140);
        assert_eq!(puzzle.part2(), 80);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        let data = aoc::load_input_data("sample_3.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part1(), 772);
        assert_eq!(puzzle.part2(), 436);
    }

    #[test]
    fn test03() {
        let mut puzzle = Puzzle::new();
        let data = aoc::load_input_data("sample_4.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part1(), 1930);
        assert_eq!(puzzle.part2(), 1206);
    }

    #[test]
    fn test04() {
        let mut puzzle = Puzzle::new();
        let data = aoc::load_input_data("sample_6.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part2(), 236);
    }

    #[test]
    fn test05() {
        let mut puzzle = Puzzle::new();
        let data = aoc::load_input_data("sample_7.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part2(), 368);
    }
}
