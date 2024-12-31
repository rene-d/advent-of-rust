//! [Day 2: Corruption Checksum](https://adventofcode.com/2017/day/2)

struct Puzzle {
    data: Vec<Vec<u32>>,
}

impl Puzzle {
    const fn new() -> Self {
        Self { data: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        for line in data.lines() {
            self.data.push(
                line.split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            );
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        self.data
            .iter()
            .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
            .sum()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        self.data
            .iter()
            .map(|row| {
                for a in row {
                    for b in row {
                        if a > b && a % b == 0 {
                            return a / b;
                        }
                    }
                }
                0
            })
            .sum()
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
        puzzle.configure(&aoc::load_input_data("sample_1.txt"));
        assert_eq!(puzzle.part1(), 18);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("sample_2.txt"));
        assert_eq!(puzzle.part2(), 9);
    }
}
