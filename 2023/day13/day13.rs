//! [Day 13: Point of Incidence](https://adventofcode.com/2023/day/13)

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Grid {
    p: Vec<Vec<char>>,
}

impl Grid {
    fn new() -> Self {
        Self { p: vec![] }
    }

    fn find_v(&self, smudge: bool) -> usize {
        let cols = self.p[0].len();

        for c in 0..(cols - 1) {
            // check symmetry as columns c/c+1
            let mut errors = 0;
            for i in 0..=c.min(cols - c - 2) {
                // count differences between column c-i and column c+1+i
                errors += self
                    .p
                    .iter()
                    .filter(|row| row[c - i] != row[c + 1 + i])
                    .count();
                if errors > 1 {
                    break;
                }
            }
            if (smudge && errors == 1) || (!smudge && errors == 0) {
                return c + 1;
            }
        }

        0
    }

    fn find_h(&self, smudge: bool) -> usize {
        let rows = self.p.len();

        for r in 0..(rows - 1) {
            // check symmetry as rows r/r+1
            let mut errors = 0;
            for i in 0..=r.min(rows - r - 2) {
                // count differences between row r-i and row r+1+i
                errors += self.p[r - i]
                    .iter()
                    .zip(self.p[r + 1 + i].iter())
                    .filter(|&(&a, &b)| a != b)
                    .count();
                if errors > 1 {
                    break;
                }
            }
            if (smudge && errors == 1) || (!smudge && errors == 0) {
                return r + 1;
            }
        }

        0
    }
}

struct Puzzle {
    grids: Vec<Grid>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { grids: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for grid in data.split("\n\n") {
            let mut z = Grid::new();

            for row in grid.lines() {
                z.p.push(row.chars().collect());
            }
            self.grids.push(z);
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.grids
            .iter()
            .map(|grid| grid.find_h(false) * 100 + grid.find_v(false))
            .sum()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        self.grids
            .iter()
            .map(|grid| grid.find_h(true) * 100 + grid.find_v(true))
            .sum()
    }
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 405);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 400);
    }
}

fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
