//! [Day 20: Race Condition](https://adventofcode.com/2024/day/20)

use std::collections::{BinaryHeap, HashMap};

use aoc24::coord::Coord;
use aoc24::grid::Grid;

struct Puzzle {
    // input
    racetrack: Grid, // the racetrack
    start: Coord,    // start position
    end: Coord,      // end position
    // precomputed
    from_start: HashMap<Coord, i32>, // distance from start
    to_end: HashMap<Coord, i32>,     // distance from end
    boring: i32,                     // length of the track
    track: Vec<Coord>,               // coords of the track (e.g. not walls)
}

impl Puzzle {
    fn new() -> Self {
        Self {
            racetrack: Grid::new(),
            start: Coord::new(0, 0),
            end: Coord::new(0, 0),
            from_start: HashMap::new(),
            to_end: HashMap::new(),
            boring: 0,
            track: Vec::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.racetrack = Grid::parse(&data);

        for (pos, &c) in self.racetrack.iter() {
            if c == 'S' {
                self.start = pos;
            } else if c == 'E' {
                self.end = pos;
            }
        }

        self.from_start = self.compute_distances(self.start);
        self.to_end = self.compute_distances(self.end);

        self.boring = self.from_start[&self.end];

        self.track = self
            .racetrack
            .iter()
            .filter(|&(pos, _)| self.racetrack[pos] != '#')
            .map(|(pos, _)| pos)
            .collect::<Vec<_>>();
    }

    fn compute_distances(&self, start: Coord) -> HashMap<Coord, i32> {
        let mut costs = HashMap::new();
        let mut heap = BinaryHeap::new();

        costs.insert(start, 0);
        heap.push((0, start));

        while let Some((cost, p)) = heap.pop() {
            for np in self.racetrack.iter_directions(p) {
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
            for cheat_end in &self.track {
                let cheat_dist = cheat_start.manhattan_distance(cheat_end);

                if cheat_dist <= max_cheats {
                    // so the distance (or the time in picoseconds) is the sum of the distance
                    // from the start to the cheat start, the cheat length and the distance from
                    // the cheat end to the end of the track

                    let time = self.from_start[cheat_start] + cheat_dist + self.to_end[cheat_end];

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

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
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
        puzzle.configure("test.txt");
        assert_eq!(
            puzzle.solve(2, 2),
            14 + 14 + 2 + 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1
        );
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(
            puzzle.solve(20, 50),
            32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
        );
    }
}
