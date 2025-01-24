//! [Day 13: Point of Incidence](https://adventofcode.com/2023/day/13)

struct Pattern {
    p: Vec<Vec<u8>>,
}

impl Pattern {
    fn new(data: &str) -> Self {
        let mut pattern = Self { p: vec![] };

        for row in data.lines() {
            pattern.p.push(
                row.chars()
                    .map(|c| match c {
                        '.' => 0,
                        '#' => 1,
                        _ => panic!("unknown {c}"),
                    })
                    .collect(),
            );
        }

        pattern
    }

    fn find_v(&self, smudge: bool) -> usize {
        let cols = self.p[0].len();

        for c in 0..(cols - 1) {
            // check symmetry at columns c/c+1
            let mut errors = 0;
            for i in 0..=c.min(cols - c - 2) {
                // count differences between column c-i and column c+1+i
                errors += self
                    .p
                    .iter()
                    .filter(|row| row[c - i] != row[c + 1 + i])
                    .count();
                if errors > 1 {
                    // too many differences, symmetry is not between cols c and c+1
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
            // check symmetry at rows r/r+1
            let mut errors = 0;
            for i in 0..=r.min(rows - r - 2) {
                // count differences between row r-i and row r+1+i
                errors += self.p[r - i]
                    .iter()
                    .zip(self.p[r + 1 + i].iter())
                    .filter(|&(&a, &b)| a != b)
                    .count();
                if errors > 1 {
                    // too many differences, symmetry is not between rows r and r+1
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
    patterns: Vec<Pattern>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            patterns: data.split("\n\n").map(Pattern::new).collect(),
        }
    }

    /// Compute the sum of notes.
    fn solve(&self, smudge: bool) -> usize {
        self.patterns
            .iter()
            .map(|grid| grid.find_h(smudge) * 100 + grid.find_v(smudge))
            .sum()
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.solve(false)
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        self.solve(true)
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, usize) {
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
        assert_eq!(puzzle.part1(), 405);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 400);
    }
}
