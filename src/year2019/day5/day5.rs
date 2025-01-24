//! [Day 5: Sunny with a Chance of Asteroids](https://adventofcode.com/2019/day/5)

use intcode::{Computer, State};

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (i64, i64) {
    let mut program = Computer::load(data);

    let mut run = |init| {
        program.reset();
        program.push(init);
        let mut result = 0;
        while let State::Output(value) = program.run() {
            result = value;
        }
        result
    };

    (run(1), run(5))
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}
