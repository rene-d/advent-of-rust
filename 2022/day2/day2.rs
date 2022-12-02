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

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let lines = data.split('\n').collect::<Vec<_>>();

    let mut part1 = 0;
    let mut part2 = 0;

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let ab = line.split(' ').collect::<Vec<_>>();

        let opponent = match *ab.first().unwrap() {
            "A" => ROCK,
            "B" => PAPER,
            "C" => SCISSORS,
            _ => panic!("bad input"),
        };

        // part 1
        let me = match *ab.get(1).unwrap() {
            "X" => ROCK,
            "Y" => PAPER,
            "Z" => SCISSORS,
            _ => panic!("bad input"),
        };

        if opponent == me {
            part1 += opponent + SCORE_DRAW;
        } else if (opponent == ROCK && me == SCISSORS)
            || (opponent == SCISSORS && me == PAPER)
            || (opponent == PAPER && me == ROCK)
        {
            part1 += me + SCORE_LOOSE;
        } else {
            part1 += me + SCORE_WIN;
        }

        // part 2
        let need_to = match *ab.get(1).unwrap() {
            "X" => NEED_TO_LOOSE,
            "Y" => NEED_TO_DRAW,
            "Z" => NEED_TO_WIN,
            _ => panic!("bad input"),
        };

        if need_to == NEED_TO_DRAW {
            part2 += opponent + SCORE_DRAW;
        } else if need_to == NEED_TO_LOOSE {
            if opponent == ROCK {
                part2 += SCISSORS + SCORE_LOOSE;
            } else if opponent == PAPER {
                part2 += ROCK + SCORE_LOOSE;
            } else if opponent == SCISSORS {
                part2 += PAPER + SCORE_LOOSE;
            }
        } else {
            if opponent == ROCK {
                part2 += PAPER + SCORE_WIN;
            } else if opponent == PAPER {
                part2 += SCISSORS + SCORE_WIN;
            } else if opponent == SCISSORS {
                part2 += ROCK + SCORE_WIN;
            }
        }
    }

    println!("{}", part1);
    println!("{}", part2);
}
