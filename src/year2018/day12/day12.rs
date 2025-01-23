//! [Day 12: Subterranean Sustainability](https://adventofcode.com/2018/day/12)

use rustc_hash::FxHashSet;

struct Puzzle {
    state: String,
    rules: FxHashSet<Vec<char>>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut state = String::new();
        let mut rules = FxHashSet::default();

        for line in data.lines() {
            if let Some(s) = line.strip_prefix("initial state: ") {
                state = s.to_string();
            } else if let Some(from) = line.strip_suffix(" => #") {
                rules.insert(from.chars().collect::<Vec<_>>());
            }
        }

        Self { state, rules }
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        let mut state = self.state.chars().collect::<Vec<_>>();

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
        let mut state = self.state.chars().collect::<Vec<_>>();

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

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (i32, i64) {
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
        assert_eq!(puzzle.part1(), 325);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 999_999_999_374);
    }
}
