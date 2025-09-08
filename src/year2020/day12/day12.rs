//! [Day 12: Rain Risk](https://adventofcode.com/2020/day/12)

struct Instruction {
    action: char,
    n: i32,
}

struct Puzzle {
    instructions: Vec<Instruction>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            instructions: data
                .lines()
                .map(|line| Instruction {
                    action: line.chars().next().unwrap(),
                    n: line[1..].parse().unwrap(),
                })
                .collect(),
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
            }
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
            }
        }

        x.abs() + y.abs()
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (i32, i32) {
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
    fn test_part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 25);
    }

    #[test]
    fn test_part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 286);
    }
}
