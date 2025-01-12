//! [Day 5: Sunny with a Chance of Asteroids](https://adventofcode.com/2019/day/5)

use intcode::{Computer, State};

fn main() {
    let args = aoc::parse_args();

    let mut program = Computer::load(&args.input);

    let mut run = |init| {
        program.reset();
        program.push(init);
        let mut result = 0;
        while let State::Output(value) = program.run() {
            result = value;
        }
        result
    };

    println!("{}", run(1)); // part 1
    println!("{}", run(5)); // part 2
}
