//! [Day 12: Subterranean Sustainability](https://adventofcode.com/2018/day/12)

use std::collections::HashSet;

struct Puzzle {
    state: String,
    rules: HashSet<Vec<char>>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            state: String::new(),
            rules: HashSet::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            if let Some(state) = line.strip_prefix("initial state: ") {
                self.state = state.to_string();
            } else if let Some(from) = line.strip_suffix(" => #") {
                self.rules.insert(Vec::from_iter(from.chars()));
            }
        }
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        let mut state = Vec::from_iter(self.state.chars());

        let mut pots = 0;

        for _ in 0..20 {
            // left
            while state[0..4] != ['.', '.', '.', '.'] {
                pots -= 1;
                state.insert(0, '.');
            }

            // right
            while state[(state.len() - 4)..] != ['.', '.', '.', '.'] {
                state.push('.');
            }

            let mut new_state = vec![];

            pots += 2;

            for c in state.windows(5) {
                if self.rules.contains(c) {
                    new_state.push('#');
                } else {
                    new_state.push('.');
                }
            }

            state = new_state;
        }

        state
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                if c == &'#' {
                    Some((i32::try_from(i).unwrap()) + pots)
                } else {
                    None
                }
            })
            .sum()
    }

    /// Solve part two.
    fn part2(&self) -> i64 {
        let mut state = Vec::from_iter(self.state.chars());

        let mut score = 0;
        let mut pots = 0;

        for seconds in 1..10000 {
            // left
            while state[0..4] != ['.', '.', '.', '.'] {
                pots -= 1;
                state.insert(0, '.');
            }

            // right
            while state[(state.len() - 4)..] != ['.', '.', '.', '.'] {
                state.push('.');
            }

            let mut new_state = vec![];

            pots += 2;

            for c in state.windows(5) {
                if self.rules.contains(c) {
                    new_state.push('#');
                } else {
                    new_state.push('.');
                }
            }

            let new_score = new_state
                .iter()
                .enumerate()
                .filter_map(|(i, c)| {
                    if c == &'#' {
                        Some((i64::try_from(i).unwrap()) + pots)
                    } else {
                        None
                    }
                })
                .sum();

            let p1 = state.iter().position(|&c| c == '#').unwrap();
            let p2 = state.iter().rposition(|&c| c == '#').unwrap();

            let n1 = new_state.iter().position(|&c| c == '#').unwrap();
            let n2 = new_state.iter().rposition(|&c| c == '#').unwrap();

            if state[p1..p2] == new_state[n1..n2] {
                // state is now stable
                // extrapolate the value at 5e9 seconds
                return new_score + (new_score - score) * (50_000_000_000 - seconds);
            }

            state = new_state;
            score = new_score;
        }

        0
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 325);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 999_999_999_374);
    }
}
