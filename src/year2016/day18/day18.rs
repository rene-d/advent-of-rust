//! [Day 18: Like a Rogue](https://adventofcode.com/2016/day/18)

struct Puzzle {
    trap_bits: u128,
    width: u32,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let s = data.trim_ascii();
        let width = u32::try_from(s.len()).unwrap();
        assert!(width <= 128, "input too wide for u128 bitset");
        let trap_bits =
            s.bytes()
                .enumerate()
                .fold(0u128, |acc, (i, b)| if b == b'^' { acc | (1u128 << i) } else { acc });
        Self { trap_bits, width }
    }

    fn solve(&self, rows: usize) -> u32 {
        // A tile is a trap iff left != right  =>  next = (row << 1) ^ (row >> 1)
        // Borders are safe (0), so the natural shift gives the right padding.
        let mask = if self.width == 128 {
            u128::MAX
        } else {
            (1u128 << self.width) - 1
        };
        let mut row = self.trap_bits;
        let mut safe = self.width - row.count_ones();

        for _ in 1..rows {
            row = ((row << 1) ^ (row >> 1)) & mask;
            safe += self.width - row.count_ones();
        }
        safe
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        self.solve(40)
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        self.solve(400_000)
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

    #[test]
    fn test01() {
        let puzzle = Puzzle::new("..^^.");
        assert_eq!(puzzle.solve(3), 6);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(".^^.^.^^^^");
        assert_eq!(puzzle.solve(10), 38);
    }
}
