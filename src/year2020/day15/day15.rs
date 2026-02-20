//! [Day 15: Rambunctious Recitation](https://adventofcode.com/2020/day/15)

#[cfg(target_pointer_width = "16")]
fn usize32(_n: u32) -> usize {
    compile_error!("16-bit architecture not supported");
}

struct Puzzle {
    nums: Vec<u32>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            nums: data
                .trim()
                .split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect(),
        }
    }

    fn solve(&self, limit: u32) -> u32 {
        let max_start = *self.nums.iter().max().unwrap_or(&0);
        let length = self.nums.iter().map(|_| 1u32).sum();
        let cap = limit.max(max_start + 1);
        let mut last_seen = vec![0u32; cap as usize];

        for (&n, i) in self.nums.iter().zip(1u32..).take(self.nums.len() - 1) {
            last_seen[n as usize] = i;
        }

        let mut last_val = *self.nums.last().unwrap();
        let mut turn = length;

        let mut last_zero = last_seen[0];

        while turn < limit {
            // The assertion below helps LLVM prove that last_val < cap,
            // which removes unnecessary bounds checking and makes the code safe
            assert!(last_val < cap);

            if last_val == 0 {
                // reduce cache pressure by avoiding reading last_seen[0]
                last_val = if last_zero != 0 { turn - last_zero } else { 0 };
                last_zero = turn;
            } else {
                let seen_at = last_seen[last_val as usize];
                last_seen[last_val as usize] = turn;
                last_val = if seen_at != 0 { turn - seen_at } else { 0 };
            }
            turn += 1;
        }

        last_val
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        self.solve(2020)
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        self.solve(30_000_000)
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
    fn test_part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 436);
    }
}
