//! [Day 5: Binary Boarding](https://adventofcode.com/2020/day/5)

fn parse_seat(seat: &str) -> u32 {
    let (mut a, mut b) = (0, 127);
    for letter in seat.chars().take(7) {
        match letter {
            'F' => b = (b - a + 1) / 2 - 1 + a,
            'B' => a = b + 1 - (b - a + 1) / 2,
            _ => (),
        }
    }
    let row = a;

    let (mut a, mut b) = (0, 7);
    for letter in seat.chars().skip(7) {
        match letter {
            'L' => b = (b - a + 1) / 2 - 1 + a,
            'R' => a = b + 1 - (b - a + 1) / 2,
            _ => (),
        }
    }
    let column = a;

    row * 8 + column
}

struct Puzzle {
    seats: Vec<u32>,
}

impl Puzzle {
    const fn new() -> Self {
        Self { seats: Vec::new() }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        self.seats.extend(data.lines().map(parse_seat));

        self.seats.sort_unstable();
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        *self.seats.iter().max().unwrap()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        for i in self.seats.windows(2) {
            if i[1] - i[0] == 2 {
                return i[0] + 1;
            }
        }
        0
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
    fn test_parse_seat() {
        assert_eq!(parse_seat("BFFFBBFRRR"), 567);
        assert_eq!(parse_seat("FFFBBBFRRR"), 119);
        assert_eq!(parse_seat("BBFFBBFRLL"), 820);
    }
}
