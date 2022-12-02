//! [Day 1: Day 2: Rock Paper Scissors](https://adventofcode.com/2022/day/2)

// do not warn about if/else constructions
#![allow(clippy::collapsible_else_if)]

pub const ROCK: u32 = 1;
pub const PAPER: u32 = 2;
pub const SCISSORS: u32 = 3;

pub const SCORE_WIN: u32 = 6;
pub const SCORE_DRAW: u32 = 3;
pub const SCORE_LOOSE: u32 = 0;

pub const NEED_TO_LOOSE: u32 = 1;
pub const NEED_TO_DRAW: u32 = 2;
pub const NEED_TO_WIN: u32 = 3;

struct Puzzle {
    part1: u32,
    part2: u32,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { part1: 0, part2: 0 }
    }

    fn solve(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();
        let lines = data.split('\n').collect::<Vec<_>>();

        for line in lines {
            if line.is_empty() {
                continue;
            }
            let turn = line.split(' ').collect::<Vec<_>>();

            let opponent = match *turn.first().unwrap() {
                "A" => ROCK,
                "B" => PAPER,
                "C" => SCISSORS,
                _ => panic!("bad input"),
            };

            // part 1
            let me = match *turn.get(1).unwrap() {
                "X" => ROCK,
                "Y" => PAPER,
                "Z" => SCISSORS,
                _ => panic!("bad input"),
            };

            if opponent == me {
                self.part1 += opponent + SCORE_DRAW;
            } else if (opponent == ROCK && me == SCISSORS)
                || (opponent == SCISSORS && me == PAPER)
                || (opponent == PAPER && me == ROCK)
            {
                self.part1 += me + SCORE_LOOSE;
            } else {
                self.part1 += me + SCORE_WIN;
            }

            // part 2
            let need_to = match *turn.get(1).unwrap() {
                "X" => NEED_TO_LOOSE,
                "Y" => NEED_TO_DRAW,
                "Z" => NEED_TO_WIN,
                _ => panic!("bad input"),
            };

            if need_to == NEED_TO_DRAW {
                self.part2 += opponent + SCORE_DRAW;
            } else if need_to == NEED_TO_LOOSE {
                if opponent == ROCK {
                    self.part2 += SCISSORS + SCORE_LOOSE;
                } else if opponent == PAPER {
                    self.part2 += ROCK + SCORE_LOOSE;
                } else if opponent == SCISSORS {
                    self.part2 += PAPER + SCORE_LOOSE;
                }
            } else {
                if opponent == ROCK {
                    self.part2 += PAPER + SCORE_WIN;
                } else if opponent == PAPER {
                    self.part2 += SCISSORS + SCORE_WIN;
                } else if opponent == SCISSORS {
                    self.part2 += ROCK + SCORE_WIN;
                }
            }
        }
    }
}

/// Solve the puzzle with the user input
fn main() {
    let mut puzzle = Puzzle::new();
    puzzle.solve("input.txt");
    println!("{}", puzzle.part1);
    println!("{}", puzzle.part2);
}

/// Test from puzzle input
#[test]
fn test_puzzle() {
    let mut puzzle = Puzzle::new();
    puzzle.solve("test.txt");
    assert_eq!(puzzle.part1, 15);
    assert_eq!(puzzle.part2, 12);
}
