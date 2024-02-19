//! [Day 1: The Tyranny of the Rocket Equation](https://adventofcode.com/2019/day/1)

struct Puzzle {
    data: Vec<i32>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { data: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.data = data.lines().map(|line| line.parse().unwrap()).collect();
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        self.data.iter().map(|mass| mass / 3 - 2).sum()
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        let mut answer = 0;
        for mass in &self.data {
            let mut fuel = *mass;
            loop {
                fuel = fuel / 3 - 2;
                if fuel <= 0 {
                    break;
                }
                answer += fuel;
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
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();

        puzzle.data = [12].to_vec();
        assert_eq!(puzzle.part1(), 2);

        puzzle.data = [14].to_vec();
        assert_eq!(puzzle.part1(), 2);

        puzzle.data = [1969].to_vec();
        assert_eq!(puzzle.part1(), 654);

        puzzle.data = [100756].to_vec();
        assert_eq!(puzzle.part1(), 33583);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();

        puzzle.data = [14].to_vec();
        assert_eq!(puzzle.part2(), 2);

        puzzle.data = [1969].to_vec();
        assert_eq!(puzzle.part2(), 966);

        puzzle.data = [100756].to_vec();
        assert_eq!(puzzle.part2(), 50346);
    }
}
