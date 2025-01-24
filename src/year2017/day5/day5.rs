//! [Day 5: A Maze of Twisty Trampolines, All Alike](https://adventofcode.com/2017/day/5)

struct Puzzle {
    jumps: Vec<i32>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            jumps: data.lines().map(|s| s.parse().unwrap()).collect(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut jumps = self.jumps.clone();
        let length = i32::try_from(jumps.len()).unwrap();
        let mut offset = 0;
        let mut n = 0;

        while 0 <= offset && offset < length {
            let index = usize::try_from(offset).unwrap();

            let jump = jumps[index];
            jumps[index] += 1;

            offset += jump;
            n += 1;
        }

        n
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let mut jumps = self.jumps.clone();
        let length = i32::try_from(jumps.len()).unwrap();
        let mut offset = 0;
        let mut n = 0;

        while 0 <= offset && offset < length {
            let index = usize::try_from(offset).unwrap();

            let jump = jumps[index];
            if jump >= 3 {
                jumps[index] -= 1;
            } else {
                jumps[index] += 1;
            }

            offset += jump;
            n += 1;
        }

        n
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
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 5);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 10);
    }
}
