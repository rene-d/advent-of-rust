//! [Day 25: Code Chronicle](https://adventofcode.com/2024/day/25)

struct Puzzle<'a> {
    data: &'a str,
}

impl<'a> Puzzle<'a> {
    /// Initialize from the puzzle input.
    const fn new(data: &'a str) -> Self {
        Self { data }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
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
    let puzzle = Puzzle::new(&args.input);
    println!("{}", puzzle.part1());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let data = aoc::load_input_data("test.txt");
        let puzzle = Puzzle::new(&data);
        assert_eq!(puzzle.part1(), 3);
    }
}
