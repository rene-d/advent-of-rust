//! [Day 11: Hex Ed](https://adventofcode.com/2017/day/11)

struct Puzzle<'a> {
    data: &'a str,
    part1: u32,
    part2: u32,
}

const STEP_X: i32 = 1;
const STEP_Y: i32 = 1;

impl<'a> Puzzle<'a> {
    const fn new(data: &'a str) -> Self {
        Self {
            data: data.trim_ascii(),
            part1: 0,
            part2: 0,
        }
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

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let mut puzzle = Puzzle::new(data);
    puzzle.solve();
    (puzzle.part1, puzzle.part2)
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(data: &str, result: u32) {
        let mut puzzle = Puzzle::new(data);
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
