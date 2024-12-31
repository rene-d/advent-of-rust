//! [Day 11: Seating System](https://adventofcode.com/2020/day/11)

use std::collections::HashMap;

const EMPTY: char = 'L';
const OCCUPIED: char = '#';

const NEIGHBORS: &[(i32, i32)] = &[
    (1, -1),
    (1, 0),
    (1, 1),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

struct Puzzle {
    seats: HashMap<(i32, i32), char>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            seats: HashMap::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        for (y, line) in (0..).zip(data.lines()) {
            for (x, c) in (0..).zip(line.chars()) {
                if c == 'L' {
                    self.seats.insert((x, y), 'L');
                }
            }
        }
    }

    fn solve(&self, tolerance: usize, visibility: i32) -> usize {
        let mut seats = self.seats.clone();

        loop {
            let mut change = false;
            let mut new_seats = HashMap::<(i32, i32), char>::new();

            for ((x, y), seat) in &seats {
                let mut occupied = 0;
                for (dx, dy) in NEIGHBORS {
                    for v in 1..=visibility {
                        if let Some(s) = seats.get(&(x + dx * v, y + dy * v)) {
                            if s == &OCCUPIED {
                                occupied += 1;
                            }
                            break;
                        }
                    }
                }

                let new_seat = if seat == &EMPTY && occupied == 0 {
                    change = true;
                    OCCUPIED
                } else if seat == &OCCUPIED && occupied >= tolerance {
                    change = true;
                    EMPTY
                } else {
                    *seat
                };

                new_seats.insert((*x, *y), new_seat);
            }

            if !change {
                break;
            }
            seats = new_seats;
        }

        seats.values().filter(|&s| s == &OCCUPIED).count()
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.solve(4, 1)
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let mut mx = 0;
        let mut my = 0;

        self.seats.keys().for_each(|&(x, y)| {
            mx = mx.max(x);
            my = my.max(y);
        });

        self.solve(5, mx + my + 1)
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
    fn test_part1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part1(), 37);
    }

    #[test]
    fn test_part2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part2(), 26);
    }
}
