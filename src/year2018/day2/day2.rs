//! [Day 2: Inventory Management System](https://adventofcode.com/2018/day/2)

struct Puzzle<'a> {
    data: &'a str,
}

impl<'a> Puzzle<'a> {
    const fn new(data: &'a str) -> Self {
        Self { data }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut two = 0;
        let mut three = 0;
        for line in self.data.lines() {
            let mut has_two = 0;
            let mut has_three = 0;
            for letter in 'a'..='z' {
                let n = line.chars().filter(|x| x == &letter).count();
                match n {
                    2 => has_two = 1,
                    3 => has_three = 1,
                    _ => (),
                }
            }
            two += has_two;
            three += has_three;
        }
        two * three
    }

    /// Solve part two.
    fn part2(&self) -> String {
        for l in self.data.lines() {
            for r in self.data.lines() {
                let same: String = l
                    .chars()
                    .zip(r.chars())
                    .filter_map(|x| if x.0 == x.1 { Some(x.0) } else { None })
                    .collect();

                if same.len() == l.len() - 1 {
                    return same;
                }
            }
        }

        "?".to_string()
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, String) {
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

    const TEST_INPUT_1: &str = include_str!("test1.txt");
    const TEST_INPUT_2: &str = include_str!("test2.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT_1);
        assert_eq!(puzzle.part1(), 12);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT_2);
        assert_eq!(puzzle.part2(), "fgij".to_string());
    }
}
