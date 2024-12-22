//! [Day 11: Radioisotope Thermoelectric Generators](https://adventofcode.com/2016/day/11)

// Nota: very slow implementation

use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

const RTG: u8 = 1;
const CHIP: u8 = 2;

// u32 is a combination of the first 3 characters of the element
type Element = (u32, u8);

#[derive(Default, Debug, Clone)]
struct Floor {
    generator: HashSet<Element>,
    microchip: HashSet<Element>,
}

impl Floor {
    fn is_valid(&self) -> bool {
        // if a chip is ever left in the same area
        for m in &self.microchip {
            let m = (m.0, RTG);
            // as another RTG, and it's not connected to its own RTG,
            if !self.generator.is_empty() && !self.generator.contains(&m) {
                // the chip will be fried
                return false;
            }
        }
        true
    }
}
#[derive(Default, Debug, Clone)]
struct State {
    floors: Vec<Floor>,
    elevator: usize,
}

impl State {
    fn key(&self) -> Vec<u32> {
        let mut s = vec![];

        s.push(u32::try_from(self.elevator).unwrap());

        for i in &self.floors {
            s.push(u32::MAX);
            s.extend(i.generator.iter().map(|(e, _)| e).sorted());
            s.push(u32::MAX);
            s.extend(i.microchip.iter().map(|(e, _)| e).sorted());
        }

        // let mut s = format!("{}", self.elevator);
        // for i in &self.floors {
        //     s += "|";
        //     i.generator
        //         .iter()
        //         .map(|(e, _)| e)
        //         .sorted()
        //         .copied()
        //         .for_each(|e| s.push_str(format!("{e}").as_str()));
        //     s += " ";
        //     i.microchip
        //         .iter()
        //         .map(|(e, _)| e)
        //         .sorted()
        //         .copied()
        //         .for_each(|e| s.push_str(format!("{e}").as_str()));
        // }

        s
    }

    /*

    fn show(&self) {
        let mut all = HashSet::new();
        for i in &self.floors {
            let a: HashSet<_> = i.generator.iter().map(|e| e.0).collect();
            let b: HashSet<_> = i.microchip.iter().map(|e| e.0).collect();

            all.extend(a);
            all.extend(b);
        }

        for i in (0..4).rev() {
            print!("F{} ", i + 1);
            if i == self.elevator {
                print!(" E ");
            } else {
                print!(" . ");
            }

            let f = &self.floors[i];

            for e in &all {
                if f.generator.contains(&(*e, RTG)) {
                    print!(" {}G ", e.to_uppercase());
                } else {
                    print!(" .  ");
                }
                if f.microchip.contains(&(*e, CHIP)) {
                    print!(" {}M ", e.to_uppercase());
                } else {
                    print!(" .  ");
                }
            }

            println!();
        }
        println!();
    }

    */

    fn next_state(&self, steps: u32, q: &mut VecDeque<(Self, u32)>) {
        let current_floor = &self.floors[self.elevator];

        let mut floor_items: Vec<_> = current_floor.generator.union(&current_floor.microchip).collect();
        floor_items.push(&(0, 0)); // empty placeholder to move just one equipment

        let n = floor_items.len();
        for indices in (0..n).combinations(2) {
            let a = floor_items[indices[0]];
            let b = floor_items[indices[1]];

            let mut new_floor = current_floor.clone();
            new_floor.generator.retain(|e| e != a && e != b);
            new_floor.microchip.retain(|e| e != a && e != b);

            if !new_floor.is_valid() {
                continue;
            }

            let down = self.elevator.saturating_sub(1);
            let up = (self.elevator + 1).min(self.floors.len() - 1);

            for next_elevator in down..=up {
                if next_elevator == self.elevator {
                    continue;
                }

                //
                let mut new_state = Self {
                    floors: self.floors.clone(),
                    elevator: next_elevator,
                };

                new_state.floors[self.elevator].clone_from(&new_floor);

                match a.1 {
                    RTG => new_state.floors[next_elevator].generator.insert(*a),
                    CHIP => new_state.floors[next_elevator].microchip.insert(*a),
                    _ => false,
                };
                match b.1 {
                    RTG => new_state.floors[next_elevator].generator.insert(*b),
                    CHIP => new_state.floors[next_elevator].microchip.insert(*b),
                    _ => false,
                };

                if !new_state.floors[next_elevator].is_valid() {
                    continue;
                }

                q.push_back((new_state, steps + 1));
            }
        }
    }
}

fn parse_floor(line: &str) -> Option<(usize, Floor)> {
    // The [first|second|...] floor contains
    let line = line.strip_prefix("The ")?;
    let (nth, line) = line.split_once(" floor contains ")?;
    let nth = match nth {
        "first" => 0,
        "second" => 1,
        "third" => 2,
        "fourth" => 3,
        _ => return None,
    };

    let line = line.strip_suffix('.')?;

    if line == "nothing relevant" {
        return Some((nth, Floor::default()));
    }

    let line = line.replace(", and ", " and ");
    let line = line.replace(" and ", ", ");

    let mut floor = Floor::default();

    for e in line.split(", ") {
        let e = e.strip_prefix("a ")?;

        let (element, eqpt) = e.split_once([' ', '-'])?;
        let element = element.chars().take(3).fold(0, |acc, c| acc * 256 + c as u32);

        match eqpt {
            "generator" => floor.generator.insert((element, RTG)),
            "compatible microchip" => floor.microchip.insert((element, CHIP)),
            _ => panic!("eqpt={eqpt}"),
        };
    }

    Some((nth, floor))
}

fn solve(floors: &[Floor]) -> u32 {
    let total = floors
        .iter()
        .map(|floor| floor.generator.len() + floor.microchip.len())
        .sum();

    let mut seen = HashSet::new();

    let mut q: VecDeque<(State, u32)> = VecDeque::new();

    let initial = State {
        floors: floors.to_vec(),
        elevator: 0,
    };

    // bfs
    q.push_back((initial, 0));
    while let Some((state, steps)) = q.pop_front() {
        if seen.contains(&state.key()) {
            continue;
        }
        seen.insert(state.key());

        // state.show();

        if state.floors[3].generator.len() + state.floors[3].microchip.len() == total {
            return steps;
        }

        state.next_state(steps, &mut q);
    }
    0
}

struct Puzzle {
    floors: Vec<Floor>,
}

impl Puzzle {
    const fn new() -> Self {
        Self { floors: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            if let Some((n, floor)) = parse_floor(line) {
                self.floors.push(floor);

                assert_eq!(n + 1, self.floors.len());
            }
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        solve(&self.floors)
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let mut floors = self.floors.clone();

        floors[0].generator.insert((0xffff_e1e4, RTG)); // elerium
        floors[0].microchip.insert((0xffff_e1e4, CHIP));

        floors[0].generator.insert((0xffff_d111, RTG)); // dilithium
        floors[0].microchip.insert((0xffff_d111, CHIP));

        solve(&floors)
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
        assert_eq!(puzzle.part1(), 11);
    }
}
