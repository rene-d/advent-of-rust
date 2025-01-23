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
    fn new(data: &str) -> Self {
        let mut guide = Vec::new();

        for strategy in data.lines() {
            let shapes = strategy.split_ascii_whitespace().collect::<Vec<_>>();
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
            guide.push((opponent, you));
        }

        Self { guide }
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

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
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
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 15);
        assert_eq!(puzzle.part2(), 12);
    }
}
