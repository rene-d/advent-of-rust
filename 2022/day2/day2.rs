//! [Day 2: Rock Paper Scissors](https://adventofcode.com/2022/day/2)

const VALUE_ROCK: u32 = 1;
const VALUE_PAPER: u32 = 2;
const VALUE_SCISSORS: u32 = 3;

const SHOULD_LOSE: u32 = 1;
const SHOULD_DRAW: u32 = 2;

const ROUND_OUTCOME_DRAW: u32 = 3;
const ROUND_OUTCOME_WIN: u32 = 6;

struct Puzzle {
    guide: Vec<(u32, u32)>,
}

impl Puzzle {
    const fn new() -> Self {
        Self { guide: Vec::new() }
    }

    fn configure(&mut self, data: &str) {
        let lines = data.trim().lines().collect::<Vec<_>>();

        for strategy in lines {
            let shapes = strategy.split(' ').collect::<Vec<_>>();
            let opponent = match shapes[0].parse::<char>().unwrap() {
                'A' => VALUE_ROCK,
                'B' => VALUE_PAPER,
                'C' => VALUE_SCISSORS,
                _ => panic!("bad input"),
            };
            let you = match shapes[1].parse::<char>().unwrap() {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => panic!("bad input"),
            };
            self.guide.push((opponent, you));
        }
    }

    fn part1(&self) -> u32 {
        let mut result = 0;
        for strategy in &self.guide {
            let (opponent, you) = *strategy;

            // Always win what has been played
            result += you;

            // Take advantage of shapes sorting to check win conditions
            if opponent + 1 == you || you + 2 == opponent {
                result += ROUND_OUTCOME_WIN;
            } else if opponent == you {
                result += ROUND_OUTCOME_DRAW;
            }
        }
        result
    }

    fn part2(&self) -> u32 {
        let mut result = 0;
        for strategy in &self.guide {
            let (opponent, you) = *strategy;

            if you == SHOULD_LOSE {
                if opponent == VALUE_ROCK {
                    result += VALUE_SCISSORS;
                } else {
                    // Take advantage of shapes sorting to know what to play
                    result += opponent - 1;
                }
            } else if you == SHOULD_DRAW {
                result += ROUND_OUTCOME_DRAW + opponent;
            } else {
                result += ROUND_OUTCOME_WIN;
                if opponent == VALUE_SCISSORS {
                    result += VALUE_ROCK;
                } else {
                    // Take advantage of shapes sorting to know what to play
                    result += opponent + 1;
                }
            }
        }
        result
    }
}

/// Solve the puzzle with the user input
fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure(&aoc::load_input_data("test.txt"));
    assert_eq!(puzzle.part1(), 15);
    assert_eq!(puzzle.part2(), 12);
}
