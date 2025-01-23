//! [Day 11: Seating System](https://adventofcode.com/2020/day/11)

use rustc_hash::FxHashMap;

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
    seats: FxHashMap<(i32, i32), char>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut seats = FxHashMap::default();

        for (y, line) in (0..).zip(data.lines()) {
            for (x, c) in (0..).zip(line.chars()) {
                if c == 'L' {
                    seats.insert((x, y), 'L');
                }
            }
        }
        Self { seats }
    }

    fn solve(&self, tolerance: usize, visibility: i32) -> usize {
        let mut seats = self.seats.clone();

        loop {
            let mut change = false;
            let mut new_seats = FxHashMap::<(i32, i32), char>::default();

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

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (usize, usize) {
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
    fn test_part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 37);
    }

    #[test]
    fn test_part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 26);
    }
}
