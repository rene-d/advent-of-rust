//! [Day 11: Radioisotope Thermoelectric Generators](https://adventofcode.com/2016/day/11)

use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

// Nota: the state could by 7*2+2=16 bits wide 2 bits for each generator and microchip and 2 for the elevator
// I don't know if it'd be really faster.
#[derive(Clone, PartialEq, Eq)]
struct State {
    items: Vec<u8>, // generators then microchips current floor
    elevator: u8,   // elevator current floor
}

impl State {
    // Same remark: the hash is actually 66 bits wide (9 bytes).
    // but it's about the same speed if I use a u128 integer as the hash key
    fn key(&self) -> [u8; 9] {
        // the hash algorithm is a critical optimization!

        let mut key = [0u8; 9];

        // count generators by floor
        for x in self.generators() {
            key[usize::from(*x)] += 1;
        }

        // count microchips by floor
        for x in self.microchips() {
            key[4 + usize::from(*x)] += 1;
        }

        key[8] = self.elevator;
        key
    }
}

impl State {
    const fn n(&self) -> usize {
        self.items.len() / 2
    }

    fn generators(&self) -> &[u8] {
        &self.items[..self.n()]
    }

    fn microchips(&self) -> &[u8] {
        &self.items[self.n()..]
    }

    fn is_valid(&self) -> bool {
        for (generator, chip) in self.generators().iter().zip(self.microchips().iter()) {
            if chip != generator && self.generators().iter().any(|generator| generator == chip) {
                return false;
            }
        }
        true
    }

    fn is_solved(&self) -> bool {
        self.items.iter().all(|floor| floor == &3)
    }

    fn up(&self, idx: usize) -> Self {
        let mut new_state = self.clone();
        new_state.items[idx] += 1;
        new_state.elevator += 1;
        new_state
    }

    fn down(&self, idx: usize) -> Self {
        let mut new_state = self.clone();
        new_state.items[idx] -= 1;
        new_state.elevator -= 1;
        new_state
    }

    fn up_two(&self, idx1: usize, idx2: usize) -> Self {
        let mut new_state = self.clone();
        new_state.items[idx1] += 1;
        new_state.items[idx2] += 1;
        new_state.elevator += 1;
        new_state
    }

    fn down_two(&self, idx1: usize, idx2: usize) -> Self {
        let mut new_state = self.clone();
        new_state.items[idx1] -= 1;
        new_state.items[idx2] -= 1;
        new_state.elevator -= 1;
        new_state
    }

    fn solve(&self) -> u32 {
        let mut seen = FxHashSet::default();
        let mut queue = VecDeque::new();

        queue.push_front((self.clone(), 0));

        let n_items = self.items.len();

        while let Some((state, steps)) = queue.pop_back() {
            let hash = state.key();

            if seen.contains(&hash) {
                continue;
            }

            // seems to be a little faster to hash/lookup than to check if a chip will be fried
            if !state.is_valid() {
                continue;
            }

            if state.is_solved() {
                return steps;
            }

            seen.insert(hash);

            let floor = state.elevator;

            for i in 0..n_items {
                if state.items[i] == floor {
                    if floor < 3 {
                        queue.push_front((state.up(i), steps + 1));
                    }

                    if floor > 0 {
                        queue.push_front((state.down(i), steps + 1));
                    }

                    for j in (i + 1)..n_items {
                        if state.items[j] == floor {
                            if floor < 3 {
                                queue.push_front((state.up_two(i, j), steps + 1));
                            }
                            if floor > 0 {
                                queue.push_front((state.down_two(i, j), steps + 1));
                            }
                        }
                    }
                }
            }
        }
        0
    }
}

struct Puzzle {
    initial: State,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        let mut generators = Vec::new();
        let mut microchips = Vec::new();
        let mut elements = FxHashMap::default();

        let re = Regex::new(r"a \b(\w+)( generator|\-compatible microchip)").unwrap();

        for (floor, line) in (0u8..).zip(data.lines()) {
            for caps in re.captures_iter(line) {
                if let Some(element) = caps.get(1) {
                    let element = element.as_str();
                    let n = elements.len();
                    let idx = *elements.entry(element).or_insert(n);

                    if let Some(kind) = caps.get(2) {
                        generators.resize(elements.len(), 0);
                        microchips.resize(elements.len(), 0);

                        if kind.as_str() == " generator" {
                            generators[idx] = floor;
                        } else {
                            microchips[idx] = floor;
                        }
                    }
                }
            }
        }

        generators.extend(&microchips);

        Self {
            initial: State {
                items: generators,
                elevator: 0,
            },
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        self.initial.solve()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let mut new_items = self.initial.clone();

        let m = new_items.n();

        // insert two new microchips
        new_items.items.insert(m, 0);
        new_items.items.insert(m, 0);

        // then insert the two new generators
        new_items.items.insert(0, 0);
        new_items.items.insert(0, 0);

        new_items.solve()
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
        assert_eq!(puzzle.part1(), 11);
    }
}
