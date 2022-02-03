/*!
[Day 13: A Maze of Twisty Little Cubicles](https://adventofcode.com/2016/day/13)

You arrive at the first floor of this new building to discover a much
less welcoming environment than the shiny atrium of the last one.
Instead, you are in a maze of twisty little cubicles, all alike.

Every location in this area is addressed by a pair of non-negative
integers (`x,y`). Each such coordinate is either a wall or an open space.
You can't move diagonally. The cube maze starts at `0,0` and seems to
extend infinitely toward **positive** `x` and `y`; negative values are **invalid**,
as they represent a location outside the building. You are in a small
waiting area at `1,1`.

While it seems chaotic, a nearby morale-boosting poster explains, the
layout is actually quite logical. You can determine whether a given `x,y`
coordinate will be a wall or an open space using a simple system:

- Find `x*x + 3*x + 2*x*y + y + y*y`.
- Add the office designer's favorite number (your puzzle input).
- Find the [binary representation](https://en.wikipedia.org/wiki/Binary_number)
  of that sum; count the **number** of [bits](https://en.wikipedia.org/wiki/Bit)
  that are `1`.
  - If the number of bits that are `1` is **even**, it's an **open space**.
  - If the number of bits that are `1` is **odd**, it's a **wall**.

What is the **fewest number of steps required** for you to reach `31,39`?

--- Part Two ---

**How many locations** (distinct `x,y` coordinates, including your starting
location) can you reach in at most `50` steps?
*/

use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    x: i32,
    y: i32,
    cost: i32,
    heuristic: i32,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> std::cmp::Ordering {
        other.heuristic.cmp(&self.heuristic)
    }
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let designer_number = data.parse::<i32>().unwrap();

    println!("{}", shortest_path((31, 39), designer_number));

    let mut n = 0;
    for y in 0..52 {
        for x in 0..52 {
            if shortest_path((x, y), designer_number) <= 50 {
                n += 1;
            }
        }
    }
    println!("{}", n);
}

/// Find the shortest path from (1, 1) to (x, y) with A* search algorithm.
fn shortest_path(destination: (i32, i32), designer_number: i32) -> i32 {
    if is_wall(destination.0, destination.1, designer_number) {
        return i32::MAX;
    }

    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::new();

    queue.push(State {
        x: 1,
        y: 1,
        cost: 0,
        heuristic: 0,
    });

    while let Some(state) = queue.pop() {
        if state.x == destination.0 && state.y == destination.1 {
            return state.cost;
        }

        for (dx, dy) in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let new_x = state.x + dx;
            let new_y = state.y + dy;
            let new_cost = state.cost + 1;

            if is_wall(new_x, new_y, designer_number) {
                continue;
            }

            if seen.contains(&(new_x, new_y)) {
                continue;
            }

            if queue.iter().any(|s| s.x == new_x && s.y == new_y && s.cost <= new_cost) {
                continue;
            }

            queue.push(State {
                x: new_x,
                y: new_y,
                cost: new_cost,
                heuristic: (new_x - destination.0).abs() + (new_y - destination.1).abs() + new_cost,
            });
        }

        seen.insert((state.x, state.y));
    }

    i32::MAX
}

fn is_wall(x: i32, y: i32, designer_number: i32) -> bool {
    let v = x * x + 3 * x + 2 * x * y + y + y * y + designer_number;
    let v = count_ones(v);
    v & 1 == 1
}

fn count_ones(value: i32) -> i32 {
    let mut count = 0;
    let mut value = value;

    while value != 0 {
        count += 1;
        value &= value - 1;
    }

    count
}

#[test]
fn test_count_ones() {
    assert_eq!(count_ones(0b001100110011), 6);
    assert_eq!(count_ones(0b110011001100), 6);
    assert_eq!(count_ones(0b111), 3);
    assert_eq!(count_ones(0b1), 1);
    assert_eq!(count_ones(0b0), 0);
}

#[test]

fn test_a_star() {
    assert_eq!(shortest_path((1, 1), 10), 0);
    assert_eq!(shortest_path((7, 4), 10), 11);
}
