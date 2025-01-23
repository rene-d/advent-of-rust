//! [Day 10: Cathode-Ray Tube](https://adventofcode.com/2022/day/10)

use aoc::ocr::scan_5x6;

struct Puzzle {
    /// Value of X during the `index+1` cycle
    cycles: Vec<i32>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut cycles = vec![];

        let lines = data.split('\n').collect::<Vec<_>>();

        #[allow(non_snake_case)]
        let mut X = 1;

        cycles.push(X); // value of X during the first cycle

        for line in lines {
            if line == "noop" {
                cycles.push(X);
            } else if let Some(v) = line.strip_prefix("addx ") {
                cycles.push(X);
                X += v.parse::<i32>().unwrap();
                cycles.push(X);
            }
        }

        Self { cycles }
    }

    // Solves part one
    fn part1(&self) -> i32 {
        let mut signal_strength = 0;
        for (i, &x) in (0..).zip(&self.cycles) {
            let cycle = i + 1;
            if (cycle + 20) % 40 == 0 {
                signal_strength += cycle * x;
            }
        }
        signal_strength
    }

    // Solve part two
    fn part2_raw(&self) -> String {
        let mut iter_x = self.cycles.iter();
        let mut crt = String::new();
        for _ in 1..=6 {
            for pixel in 1..=40 {
                let sprite = *iter_x.next().unwrap();
                if sprite <= pixel && pixel < sprite + 3 {
                    crt.push('#');
                } else {
                    crt.push('.');
                }
            }
            crt.push('\n');
        }
        crt
    }

    fn part2(&self) -> String {
        scan_5x6(&self.part2_raw())
    }
}

#[must_use]
pub fn solve(data: &str) -> (i32, String) {
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
        assert_eq!(puzzle.part1(), 13140);
    }

    #[test]
    fn test_part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(
            puzzle.part2_raw(),
            "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
    }
}
