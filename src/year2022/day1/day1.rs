//! [Day 1: Calorie Counting](https://adventofcode.com/2022/day/1)

struct Puzzle {
    calories: Vec<usize>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut calories = data
            .trim_ascii()
            .split("\n\n")
            .map(|x| {
                x.split('\n')
                    .map(|y| y.parse::<usize>().unwrap())
                    .sum::<usize>()
            })
            .collect::<Vec<_>>();

        // Reverse sort to have to most significant values first
        calories.sort_by(|a, b| b.cmp(a));

        Self { calories }
    }

    fn part1(&self) -> usize {
        self.calories[0]
    }

    fn part2(&self) -> usize {
        self.calories[0..3].iter().sum::<usize>()
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
        assert_eq!(puzzle.part1(), 24000);
        assert_eq!(puzzle.part2(), 45000);
    }
}
