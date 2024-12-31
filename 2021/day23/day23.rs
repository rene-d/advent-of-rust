//! [Day 23: Amphipod](https://adventofcode.com/2021/day/23)

#![allow(dead_code)]

use std::{collections::HashMap, ops::Index};

const HALLS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
const ROOMS: [usize; 4] = [2, 4, 6, 8];
const ENERY: [usize; 4] = [1, 10, 100, 1000];

trait BurrowPlace {
    fn is_room(&self) -> bool;
}

impl BurrowPlace for usize {
    fn is_room(&self) -> bool {
        ROOMS.contains(self)
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Burrow {
    p: [Vec<u8>; 11],
}

impl Burrow {
    /// Returns the amphipod if a hall or the first if a room.
    /// Should exist!
    fn bug(&self, index: usize) -> u8 {
        *self[index].last().unwrap()
    }

    fn move_bug(&self, from: usize, to: usize) -> Self {
        let mut new = self.clone();

        let bug = new.p[from].pop().unwrap();
        new.p[to].push(bug);

        new
    }

    fn new() -> Self {
        Self {
            p: [
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
        }
    }

    fn from(rooms: &[Vec<u8>]) -> Self {
        Self {
            p: [
                vec![],
                vec![],
                rooms[0].clone(),
                vec![],
                rooms[1].clone(),
                vec![],
                rooms[2].clone(),
                vec![],
                rooms[3].clone(),
                vec![],
                vec![],
            ],
        }
    }
}

impl Index<usize> for Burrow {
    type Output = Vec<u8>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.p[index]
    }
}

impl std::fmt::Display for Burrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;

        let wb = |f: &mut std::fmt::Formatter<'_>, x: usize, y: usize| -> std::fmt::Result {
            if let Some(bug) = self.p[x].get(y) {
                write!(f, "{}", (bug + b'A') as char)
            } else {
                write!(f, ".")
            }
        };

        write!(f, "#")?;
        for i in 0..11 {
            if HALLS.contains(&i) {
                wb(f, i, 0)?;
            } else {
                write!(f, ".")?;
            }
        }
        writeln!(f, "#")?;

        let room_size = if self.p.iter().map(Vec::len).sum::<usize>() == 8 {
            2
        } else {
            4
        };

        for i in (0..room_size).rev() {
            if i == room_size - 1 {
                write!(f, "###")?;
            } else {
                write!(f, "  #")?;
            }
            for j in 0..4 {
                wb(f, 2 * j + 2, i)?;
                write!(f, "#")?;
            }

            if i == room_size - 1 {
                writeln!(f, "##")?;
            } else {
                writeln!(f)?;
            }
        }

        writeln!(f, "  #########")
    }
}

struct Puzzle {
    rooms: Vec<Vec<u8>>,
    target: Burrow,
    seen: HashMap<Burrow, usize>,
}

fn movements(hi: usize, ri: usize) -> std::ops::Range<usize> {
    if hi <= ri {
        (hi + 1)..(ri + 1)
    } else {
        ri..hi
    }
}
impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            rooms: vec![],
            target: Burrow::new(),
            seen: HashMap::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        let mut rooms = vec![vec![]; 4];

        for line in data.lines() {
            line.as_bytes()
                .iter()
                .filter_map(|c| {
                    if c.is_ascii_uppercase() {
                        Some(c - b'A')
                    } else {
                        None
                    }
                })
                .enumerate()
                .for_each(|(room, amphipod)| rooms[room].insert(0, amphipod));
        }

        self.rooms = rooms;
    }

    fn search(&mut self, burrow: &Burrow, room_size: usize) -> usize {
        if burrow == &self.target {
            return 0;
        }

        if let Some(&energy) = self.seen.get(burrow) {
            return energy;
        }

        let mut least_energy = usize::MAX;

        // move from hall to room
        for hi in HALLS {
            if let Some(&bug) = burrow[hi].first() {
                let ri = ROOMS[bug as usize];
                let room = &burrow[ri];

                if movements(hi, ri).all(|i| i.is_room() || burrow[i].is_empty())
                    && room.len() < room_size
                    && room.iter().all(|&i| i == bug)
                {
                    let new = burrow.move_bug(hi, ri);
                    let energy = (ri.abs_diff(hi) + room_size - room.len()) * ENERY[bug as usize];
                    least_energy =
                        least_energy.min(self.search(&new, room_size).saturating_add(energy));
                }
            }
        }

        // move from room to hall
        for (i, &ri) in ROOMS.iter().enumerate() {
            if !burrow[ri].iter().all(|&x| x == (i as u8)) {
                let bug = burrow.bug(ri);
                let room = &burrow[ri];

                for hi in HALLS {
                    if movements(ri, hi).all(|i| i.is_room() || burrow[i].is_empty()) {
                        let new = burrow.move_bug(ri, hi);
                        let energy =
                            (ri.abs_diff(hi) + room_size - room.len() + 1) * ENERY[bug as usize];
                        least_energy =
                            least_energy.min(self.search(&new, room_size).saturating_add(energy));
                    }
                }
            }
        }

        self.seen.insert(burrow.clone(), least_energy);

        least_energy
    }

    /// Solve part one.
    fn part1(&mut self) -> usize {
        let burrow: Burrow = Burrow::from(&self.rooms);

        let target: Vec<_> = (0u8..4u8).map(|i| vec![i; 2]).collect();
        self.target = Burrow::from(&target);

        self.seen.clear();
        self.search(&burrow, 2)
    }

    /// Solve part two.
    fn part2(&mut self) -> usize {
        let mut rooms = self.rooms.clone();

        rooms[0].insert(1, 3); // add Desert
        rooms[0].insert(2, 3); // add Desert

        rooms[1].insert(1, 1); // add Bronze
        rooms[1].insert(2, 2); // add Copper

        rooms[2].insert(1, 0); // add Amber
        rooms[2].insert(2, 1); // add Bronze

        rooms[3].insert(1, 2); // add Copper
        rooms[3].insert(2, 0); // add Amber

        let burrow: Burrow = Burrow::from(&rooms);

        let target: Vec<_> = (0u8..4u8).map(|i| vec![i; 4]).collect();
        self.target = Burrow::from(&target);

        self.seen.clear();
        self.search(&burrow, 4)
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
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part1(), 12521);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part2(), 44169);
    }
}
