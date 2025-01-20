//! [Day 20: Race Condition](https://adventofcode.com/2024/day/20)

use rustc_hash::FxHashMap;
use std::collections::BinaryHeap;

use aoc::Coord;

type Grid = aoc::Grid<char>;

struct Puzzle {
    // input
    racetrack: Grid, // the racetrack
    start: Coord,    // start position
    end: Coord,      // end position
    // precomputed
    from_start: FxHashMap<Coord, i32>, // distance from start
    to_end: FxHashMap<Coord, i32>,     // distance from end
    boring: i32,                       // length of the track
    track: Vec<Coord>,                 // coords of the track (e.g. not walls)
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut puzzle = Self {
            racetrack: Grid::new(),
            start: Coord::default(),
            end: Coord::default(),
            from_start: FxHashMap::default(),
            to_end: FxHashMap::default(),
            boring: 0,
            track: Vec::new(),
        };

        puzzle.racetrack = Grid::parse(data);

        for (pos, &c) in &puzzle.racetrack {
            if c == 'S' {
                puzzle.start = pos;
            } else if c == 'E' {
                puzzle.end = pos;
            }
        }

        puzzle.from_start = puzzle.compute_distances(puzzle.start);
        puzzle.to_end = puzzle.compute_distances(puzzle.end);

        puzzle.boring = puzzle.from_start[&puzzle.end];

        puzzle.track = puzzle
            .racetrack
            .iter()
            .filter(|&(pos, _)| puzzle.racetrack[pos] != '#')
            .map(|(pos, _)| pos)
            .collect::<Vec<_>>();

        puzzle
    }

    fn compute_distances(&self, start: Coord) -> FxHashMap<Coord, i32> {
        let mut costs = FxHashMap::default();
        let mut heap = BinaryHeap::new();

        costs.insert(start, 0);
        heap.push((0, start));

        while let Some((cost, p)) = heap.pop() {
            for (_, np) in self.racetrack.iter_directions(p) {
                if self.racetrack[np] != '#' {
                    let new_cost = cost + 1;

                    if costs.get(&np).unwrap_or(&i32::MAX) > &new_cost {
                        costs.insert(np, new_cost);
                        heap.push((new_cost, np));
                    }
                }
            }
        }

        costs
    }

    fn solve(&self, max_cheats: i32, min_gain: i32) -> u32 {
        let mut nb = 0;

        // to count the cheats, we wiil iterate over all track positions that are
        // at a Manhattan distance less than the asked one (2 or 20 depending on the part)

        for cheat_start in &self.track {
            for &cheat_end in &self.track {
                let cheat_dist = cheat_start.manhattan_distance(cheat_end);

                if cheat_dist <= max_cheats {
                    // so the distance (or the time in picoseconds) is the sum of the distance
                    // from the start to the cheat start, the cheat length and the distance from
                    // the cheat end to the end of the track

                    let time = self.from_start[cheat_start] + cheat_dist + self.to_end[&cheat_end];

                    // if the gain is sufficient, we count it
                    if time + min_gain <= self.boring {
                        nb += 1;
                    }
                }
            }
        }
        nb
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        self.solve(2, 100)
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        self.solve(20, 100)
    }
}

fn solve(data: &str) -> (u32, u32) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

fn main() {
    let mut args = aoc::parse_args();
    args.run(solve);
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(
            puzzle.solve(2, 2),
            14 + 14 + 2 + 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1
        );
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(
            puzzle.solve(20, 50),
            32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
        );
    }
}
