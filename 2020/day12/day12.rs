//! [Day 12: Rain Risk](https://adventofcode.com/2020/day/12)

struct Instruction {
    action: char,
    n: i32,
}

struct Puzzle {
    instructions: Vec<Instruction>,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        for line in data.lines() {
            let action = Instruction {
                action: line.chars().next().unwrap(),
                n: line[1..].parse().unwrap(),
            };

            self.instructions.push(action);
        }
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        let mut x = 0;
        let mut y = 0;
        let mut d = 90;

        for Instruction { action, n } in &self.instructions {
            match action {
                'N' => y += n,
                'S' => y -= n,
                'E' => x += n,
                'W' => x -= n,
                'L' => d = (d - n).rem_euclid(360),
                'R' => d = (d + n).rem_euclid(360),
                'F' => match d {
                    0 => y += n,
                    90 => x += n,
                    180 => y -= n,
                    270 => x -= n,
                    _ => panic!(),
                },
                _ => panic!(),
            };
        }

        x.abs() + y.abs()
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        let (mut x, mut y) = (0, 0);
        let (mut wx, mut wy) = (10, 1);

        for Instruction { action, n } in &self.instructions {
            match action {
                'N' => wy += n,
                'S' => wy -= n,
                'E' => wx += n,
                'W' => wx -= n,
                'L' => match n {
                    90 => (wx, wy) = (-wy, wx),
                    180 => (wx, wy) = (-wx, -wy),
                    270 => (wx, wy) = (wy, -wx),
                    _ => panic!(),
                },
                'R' => match n {
                    270 => (wx, wy) = (-wy, wx),
                    180 => (wx, wy) = (-wx, -wy),
                    90 => (wx, wy) = (wy, -wx),
                    _ => panic!(),
                },
                'F' => (x, y) = (x + wx * n, y + wy * n),
                _ => panic!(),
            };
        }

        x.abs() + y.abs()
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
    fn test_part1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part1(), 25);
    }

    #[test]
    fn test_part2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part2(), 286);
    }
}
