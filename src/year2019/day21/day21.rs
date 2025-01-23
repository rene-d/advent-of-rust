//! [Day 21: Springdroid Adventure](https://adventofcode.com/2019/day/21)

use intcode::{Computer, State};

struct Puzzle {
    springdroid: Computer,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        Self {
            springdroid: Computer::load(data),
        }
    }

    fn run_script(&self, script: &str) -> i64 {
        let mut springdroid = self.springdroid.clone();

        springdroid.push_ascii(script);

        let mut robot_report = 0;
        loop {
            match springdroid.run() {
                State::Output(num) => {
                    // print!("{}", (num as u8) as char);
                    robot_report = num;
                }
                State::Input => panic!("missing input ?!"),
                State::Halted => break robot_report,
            };
        }
    }

    /// Solve part one.
    fn part1(&self) -> i64 {
        self.run_script(concat!(
            "NOT A J\n",
            "NOT C T\n",
            "AND D T\n",
            "OR T J\n",
            "WALK\n"
        ))
    }

    /// Solve part two.
    fn part2(&self) -> i64 {
        self.run_script(concat!(
            "NOT B J\n",
            "NOT C T\n",
            "OR T J\n",
            "AND D J\n",
            "AND H J\n",
            "NOT A T\n",
            "OR T J\n",
            "RUN\n"
        ))
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (i64, i64) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}
