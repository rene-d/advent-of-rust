//! [Day 22: Sporifica Virus](https://adventofcode.com/2017/day/22)

use rustc_hash::{FxHashMap, FxHashSet};

const MOVES: &[(i32, i32)] = &[
    (0, -1), // up
    (1, 0),  // right
    (0, 1),  // down
    (-1, 0), // left
];

/// Node states for part 2
#[derive(Clone, Copy)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

struct Puzzle {
    infected: FxHashSet<(i32, i32)>,
    nx: i32, // size of the map
    ny: i32,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        let mut infected = FxHashSet::default();
        let mut nx = 0;
        let mut ny = 0;

        for (y, line) in (0..).zip(data.lines()) {
            for (x, c) in (0..).zip(line.chars()) {
                if c == '#' {
                    infected.insert((x, y));
                }
                nx = nx.max(x + 1);
            }
            ny = y + 1;
        }

        Self { infected, nx, ny }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut infected = self.infected.clone();

        let mut x = self.nx / 2; // middle of the map
        let mut y = self.ny / 2;
        let mut dir: usize = 0; // start facing up

        let mut infections = 0;

        for _ in 0..10_000 {
            if infected.remove(&(x, y)) {
                dir = (dir + 1) % 4; // turn right and clean the node
            } else {
                dir = (dir + 3) % 4; // turn left
                infected.insert((x, y)); // infect the node
                infections += 1;
            }

            // move
            let (dx, dy) = MOVES[dir];
            x += dx;
            y += dy;
        }

        infections
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let mut nodes = self
            .infected
            .iter()
            .copied()
            .map(|(x, y)| ((x, y), State::Infected))
            .collect::<FxHashMap<(i32, i32), State>>();

        let mut x = self.nx / 2; // middle of the map
        let mut y = self.ny / 2;
        let mut dir: usize = 0; // start facing up

        let mut infections = 0;

        for _ in 0..10_000_000 {
            let node = nodes.entry((x, y)).or_insert(State::Clean);

            (dir, *node) = match node {
                State::Clean => ((dir + 3) % 4, State::Weakened), // turn left
                State::Weakened => {
                    infections += 1;
                    (dir, State::Infected) // do not turn
                }
                State::Infected => ((dir + 1) % 4, State::Flagged), // turn right
                State::Flagged => ((dir + 2) % 4, State::Clean),    // go back
            };

            // move
            let (dx, dy) = MOVES[dir];
            x += dx;
            y += dy;
        }

        infections
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
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
        assert_eq!(puzzle.part1(), 5587);
    }

    #[test]
    fn part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 2511944);
    }
}
