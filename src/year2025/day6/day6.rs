//! [Day 6: Trash Compactor](https://adventofcode.com/2025/day/6)

struct Puzzle<'a> {
    //
    rows: Vec<&'a str>,
    operators: &'a str,
}

impl<'a> Puzzle<'a> {
    /// Initialize from the puzzle input.
    fn new(data: &'a str) -> Self {
        let mut rows = vec![];
        let mut operators = "";

        for line in data.lines() {
            if line.contains('+') || line.contains('*') {
                operators = line;
            } else {
                rows.push(line);
            }
        }

        Self { rows, operators }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        let mut result = 0;
        let rows = self
            .rows
            .iter()
            .map(|row| row.split_ascii_whitespace().collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        for (i, op) in self.operators.split_ascii_whitespace().enumerate() {
            result += match op {
                "+" => rows.iter().map(|row| row[i].parse::<u64>().unwrap()).sum(),
                "*" => rows
                    .iter()
                    .map(|row| row[i].parse::<u64>().unwrap())
                    .product::<u64>(),
                _ => panic!("unknown op {op}"),
            };
        }
        result
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        let mut result = 0;

        for (p, op) in self.operators.char_indices().filter(|(_, op)| op != &' ') {
            let mut end = p + 1;
            while end < self.operators.len() && self.operators.chars().nth(end) == Some(' ') {
                end += 1;
            }
            if end != self.operators.len() {
                end -= 1;
            }

            let mut nums = vec![];

            for i in p..end {
                let mut vnum = 0;
                for row in &self.rows {
                    if let Some(ch) = row.chars().nth(i)
                        && ch != ' '
                    {
                        vnum = vnum * 10 + u64::from(ch.to_digit(10).unwrap());
                    }
                }
                nums.push(vnum);
            }

            result += match op {
                '+' => nums.iter().sum(),
                '*' => nums.iter().product::<u64>(),
                _ => panic!("unknown op {op}"),
            };
        }

        result
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u64, u64) {
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
    fn part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 4277556);
    }

    #[test]
    fn part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 3263827);
    }
}
