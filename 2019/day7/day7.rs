//! [Day 7: Amplification Circuit](https://adventofcode.com/2019/day/7)

use intcode::{Computer, State};
use itertools::Itertools;

fn run_amplifiers(amp: &Computer, phases: &[i64]) -> i64 {
    let mut amp = amp.clone();

    let mut input_signal = 0;
    for &phase in phases {
        amp.reset();

        amp.push(phase);
        amp.push(input_signal);

        if let State::Output(result) = amp.run() {
            input_signal = result;
        } else {
            // error
            return 0;
        }
    }

    input_signal
}

fn run_feedback(amp: &Computer, phases: &[i64]) -> i64 {
    let mut amps = vec![amp.clone(); 5];

    let mut input_signal = 0;

    for k in 0..=10 {
        for (&phase, amp) in phases.iter().zip(&mut amps) {
            if k == 0 {
                amp.push(phase);
            }
            amp.push(input_signal);

            match amp.run() {
                State::Halted => {
                    return input_signal;
                }
                State::Output(result) => input_signal = result,
                State::Input => {}
            }
        }
    }

    // error
    0
}

struct Puzzle {
    amp: Computer,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        Self {
            amp: Computer::load(data),
        }
    }

    /// Solve part one.
    fn part1(&self) -> i64 {
        (0..5)
            .permutations(5)
            .map(|phases| run_amplifiers(&self.amp, &phases))
            .max()
            .unwrap()
    }

    /// Solve part two.
    fn part2(&self) -> i64 {
        (5..10)
            .permutations(5)
            .map(|phases| run_feedback(&self.amp, &phases))
            .max()
            .unwrap()
    }
}

fn main() {
    let args = aoc::parse_args();
    let puzzle = Puzzle::new(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_amp1() {
        let amp = Computer::load("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        let r = run_amplifiers(&amp, &[4, 3, 2, 1, 0]);
        assert_eq!(r, 43210);
    }

    #[test]
    fn test_amp2() {
        let amp = Computer::load(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        );
        let r = run_amplifiers(&amp, &[0, 1, 2, 3, 4]);
        assert_eq!(r, 54321);
    }

    #[test]
    fn test_amp3() {
        let  amp = Computer::load("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
        let r = run_amplifiers(&amp, &[1, 0, 4, 3, 2]);
        assert_eq!(r, 65210);
    }

    #[test]
    fn test_feedback1() {
        let amp = Computer::load(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        );
        let r = run_feedback(&amp, &[9, 8, 7, 6, 5]);
        assert_eq!(r, 139629729);
    }

    #[test]
    fn test_feedback2() {
        let amp = Computer::load(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
        );
        let r = run_feedback(&amp, &[9, 7, 8, 5, 6]);
        assert_eq!(r, 18216);
    }
}
