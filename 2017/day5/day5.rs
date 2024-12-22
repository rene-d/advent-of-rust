//! [Day 5: A Maze of Twisty Trampolines, All Alike](https://adventofcode.com/2017/day/5)

struct Puzzle {
    jumps: Vec<i32>,
}

impl Puzzle {
    const fn new() -> Self {
        Self { jumps: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.jumps = data.lines().map(|s| s.parse().unwrap()).collect();
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
        assert_eq!(puzzle.part1(), 5);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 10);
    }
}
