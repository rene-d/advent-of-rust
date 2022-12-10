//! [Day 10: Cathode-Ray Tube](https://adventofcode.com/2022/day/10)

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Puzzle {
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
            if (i + 2 + 20) % 40 == 0 {
                signal_strength += ((i + 2) as i32) * (*x);
            }
        }
        signal_strength
    }

    // Solve part two
    fn part2(&self) -> String {
        let mut sprite = 1;
        let mut iter_x = self.cycles.iter();
        let mut crt = String::new();
        for _ in 1..=6 {
            for pixel in 1..=40 {
                if sprite <= pixel && pixel < sprite + 3 {
                    crt.push('#');
                } else {
                    crt.push('.');
                }
                sprite = *iter_x.next().unwrap();
            }
            crt.push('\n');
        }
        crt
    }
}

/// main function
fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.path);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test.txt");
    assert_eq!(puzzle.part1(), 13140);
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
