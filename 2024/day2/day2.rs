//! [Day 2: Red-Nosed Reports](https://adventofcode.com/2024/day/2)

struct Puzzle {
    data: Vec<Vec<i32>>,
}

impl Puzzle {
    const fn new() -> Self {
        Self { data: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        for line in data.lines() {
            self.data.push(
                line.split_whitespace()
                    .filter_map(|s: &str| s.parse().ok())
                    .collect(),
            );
        }
    }

    fn is_safe(v: &[i32]) -> bool {
        let safe_rule = |a: i32, b: i32| -> bool { (1 <= a - b) && (a - b <= 3) };

        let increasing = v.windows(2).all(|pair| safe_rule(pair[0], pair[1]));
        let decreasing = v.windows(2).all(|pair| safe_rule(pair[1], pair[0]));

        increasing || decreasing
    }

    fn is_safe_except_one(v: &[i32]) -> bool {
        for i in 0..v.len() {
            let mut w = v.to_vec();
            w.remove(i);
            if Self::is_safe(&w) {
                return true;
            }
        }
        false
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.data
            .iter()
            .filter(|v: &&Vec<i32>| Self::is_safe(v))
            .count()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        self.data
            .iter()
            .filter(|v: &&Vec<i32>| Self::is_safe(v) || Self::is_safe_except_one(v))
            .count()
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
        let data = aoc::load_input_data("test.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part1(), 2);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        let data = aoc::load_input_data("test.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part2(), 4);
    }
}
