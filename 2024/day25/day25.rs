//! [Day 25: Code Chronicle](https://adventofcode.com/2024/day/25)

struct Puzzle {
    data: String,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        self.data = std::fs::read_to_string(path).unwrap_or_else(|_| {
            eprintln!("cannot read input file {path}");
            std::process::exit(1);
        });
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut locks = Vec::new();
        let mut keys = Vec::new();

        for schematics in self.data.split("\n\n") {
            let heights: Vec<_> = (0..5)
                .map(|x| {
                    schematics
                        .lines()
                        .filter(|row| row.chars().nth(x).unwrap() == '#')
                        .count()
                        - 1
                })
                .collect();

            if schematics.starts_with("#####") {
                locks.push(heights);
            } else {
                keys.push(heights);
            }
        }

        let mut answer = 0;

        for key in &keys {
            for lock in &locks {
                if key.iter().zip(lock.iter()).all(|(a, b)| a + b <= 5) {
                    answer += 1;
                }
            }
        }

        answer
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 3);
    }
}
