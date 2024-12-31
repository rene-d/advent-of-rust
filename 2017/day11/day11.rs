//! [Day 11: Hex Ed](https://adventofcode.com/2017/day/11)

struct Puzzle {
    data: String,
    part1: u32,
    part2: u32,
}

const STEP_X: i32 = 1;
const STEP_Y: i32 = 1;

impl Puzzle {
    const fn new() -> Self {
        Self {
            data: String::new(),
            part1: 0,
            part2: 0,
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        self.data = data.trim().to_string();
    }

    /// Determine the fewest number of steps to return to (0,0).
    const fn steps(x: i32, y: i32) -> u32 {
        let mut x = x.abs();
        let mut y = y.abs();
        let mut steps = 0;

        while x != 0 {
            steps += 1;
            if x >= y {
                x -= STEP_X;
                y -= STEP_Y;
            } else {
                y -= STEP_Y * 2;
            }
        }

        steps
    }

    /// Walk throuhg the "hexes" and compute expected values.
    fn solve(&mut self) {
        let mut x = 0;
        let mut y = 0;
        let mut steps_max = 0;

        for d in self.data.split(',') {
            match d {
                "ne" => {
                    x += STEP_X;
                    y += STEP_Y;
                }
                "se" => {
                    x += STEP_X;
                    y -= STEP_Y;
                }
                "s" => y -= STEP_Y * 2,
                "n" => y += STEP_Y * 2,
                "nw" => {
                    x -= STEP_X;
                    y += STEP_Y;
                }
                "sw" => {
                    x -= STEP_X;
                    y -= STEP_Y;
                }
                _ => panic!(),
            }
            steps_max = steps_max.max(Self::steps(x, y));
        }

        self.part1 = Self::steps(x, y);
        self.part2 = steps_max;
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
    puzzle.solve();
    println!("{}", puzzle.part1);
    println!("{}", puzzle.part2);
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    fn test(data: &str, result: u32) {
        let mut puzzle = Puzzle::new();
        puzzle.data = data.to_string();
        puzzle.solve();
        assert_eq!(puzzle.part1, result);
    }

    #[test]
    fn test01() {
        test("ne,ne,ne", 3);
    }

    #[test]
    fn test02() {
        test("ne,ne,sw,sw", 0);
    }

    #[test]
    fn test03() {
        test("ne,ne,s,s", 2);
    }

    #[test]
    fn test04() {
        test("se,sw,se,sw,sw", 3);
    }
}
