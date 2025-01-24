//! [Day 15: Oxygen System](https://adventofcode.com/2019/day/15)

use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

use intcode::{Computer, State};

const NORTH: i64 = 1;
const SOUTH: i64 = 2;
const WEST: i64 = 3;
const EAST: i64 = 4;

const WALL: i64 = 0;
// const EMPTY: i64 = 1;
const OXYGEN: i64 = 2;

fn bfs(droid: &Computer) -> (Computer, i64, i64) {
    let mut q = VecDeque::new();
    let mut seen = FxHashSet::default();
    let mut droids = FxHashMap::default();

    q.push_back((0, 0, 0));
    droids.insert((0, 0), droid.clone());
    let mut max_steps = 0;

    while let Some((x, y, steps)) = q.pop_front() {
        max_steps = max_steps.max(steps);

        let droid = droids.get(&(x, y)).unwrap().clone();

        seen.insert((x, y));

        for direction in [NORTH, SOUTH, WEST, EAST] {
            let (dx, dy) = match direction {
                NORTH => (0, 1),
                SOUTH => (0, -1),
                WEST => (-1, 0),
                EAST => (1, 0),
                _ => unreachable!(),
            };

            let (mx, my) = (x + dx, y + dy);

            if seen.contains(&(mx, my)) {
                continue;
            }

            let mut new_droid = droid.clone();
            new_droid.push(direction);
            let State::Output(status) = new_droid.run() else {
                panic!("unattended state");
            };

            if status == OXYGEN {
                return (new_droid, steps + 1, 0);
            }

            if status != WALL {
                droids.insert((mx, my), new_droid);
                q.push_back((mx, my, steps + 1));
            }
        }
    }

    (Computer::new(), 0, max_steps)
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (i64, i64) {
    let droid = Computer::load(data);

    let (oxygen, steps, _) = bfs(&droid);
    let (_, _, distance) = bfs(&oxygen);

    (steps, distance)
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}
