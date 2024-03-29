//! [Day 10: Cathode-Ray Tube](https://adventofcode.com/2022/day/10)

use aoc::ocr::scan_5x6;

struct Puzzle {
    /// Value of X during the `index+1` cycle
    cycles: Vec<i32>,
}

impl Puzzle {
    fn new() -> Self {
        Self { cycles: vec![] }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();
        let lines = data.split('\n').collect::<Vec<_>>();

        #[allow(non_snake_case)]
        let mut X = 1;

        self.cycles.push(X); // value of X during the first cycle

        for line in lines {
            if line == "noop" {
                self.cycles.push(X);
            } else if let Some(v) = line.strip_prefix("addx ") {
                self.cycles.push(X);
                X += v.parse::<i32>().unwrap();
                self.cycles.push(X);
            }
        }
    }

    // Solves part one
    fn part1(&self) -> i32 {
        let mut signal_strength = 0;
        for (i, x) in self.cycles.iter().enumerate() {
            let cycle = (i + 1) as i32;
            if (cycle + 20) % 40 == 0 {
                signal_strength += cycle * (*x);
            }
        }
        signal_strength
    }

    // Solve part two
    fn part2(&self) -> String {
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
}

/// main function
fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.path);
    println!("{}", puzzle.part1());
    // println!("{}", puzzle.part2());
    println!("{}", scan_5x6(&puzzle.part2()));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 13140);
    }

    #[test]
    fn test_part2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(
            puzzle.part2(),
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
