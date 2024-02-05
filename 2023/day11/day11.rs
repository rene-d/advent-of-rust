//! [Day 11: Cosmic Expansion](https://adventofcode.com/2023/day/11)

struct Puzzle {
    galaxies: Vec<(u64, u64)>,
    empty_rows: Vec<u64>,
    empty_cols: Vec<u64>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            empty_rows: vec![],
            empty_cols: vec![],
            galaxies: vec![],
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        let mut grid = vec![];

        for (line, y) in data.lines().zip(0..) {
            let row: Vec<_> = line.chars().collect();
            for (&c, x) in row.iter().zip(0..) {
                if c == '#' {
                    self.galaxies.push((x, y));
                }
            }

            if row.iter().all(|&c| c == '.') {
                self.empty_rows.push(y);
            }

            grid.push(row);
        }

        for x in 0..grid[0].len() {
            if (0..grid.len()).all(|y| grid[y][x] == '.') {
                self.empty_cols.push(x as u64);
            }
        }
    }

    fn solve(&self, expansion_factor: u64) -> u64 {
        let expansion_factor = expansion_factor - 1;
        let mut result = 0;
        for (i, &(x1, y1)) in self.galaxies.iter().enumerate() {
            for &(x2, y2) in self.galaxies.iter().skip(i + 1) {
                let mut distance = x2.abs_diff(x1) + y2.abs_diff(y1);

                // expand empty spaces horizontally
                distance += self
                    .empty_cols
                    .iter()
                    .filter(|&&col| x1.min(x2) <= col && col <= x1.max(x2))
                    .count() as u64
                    * expansion_factor;

                // expand empty spaces vertically
                distance += self
                    .empty_rows
                    .iter()
                    .filter(|&&row| y1.min(y2) <= row && row <= y1.max(y2))
                    .count() as u64
                    * expansion_factor;

                result += distance;
            }
        }

        result
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        self.solve(2)
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        self.solve(1_000_000)
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
        assert_eq!(puzzle.part1(), 374);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.solve(10), 1030);
        assert_eq!(puzzle.solve(100), 8410);
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
