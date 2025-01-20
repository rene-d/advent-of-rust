//! [Day 25: The Halting Problem](https://adventofcode.com/2017/day/25)

use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Default)]
struct Blueprint<'a> {
    write_if: [isize; 2],
    advance: [isize; 2],
    next_state: [&'a str; 2],
}

struct Puzzle<'a> {
    data: &'a str,
}

impl<'a> Puzzle<'a> {
    /// Initialize from the puzzle input.
    const fn new(data: &'a str) -> Self {
        Self { data }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let mut blueprints: FxHashMap<&str, Blueprint> = FxHashMap::default();
        let mut state: &str = "";
        let mut steps: u32 = 0;

        let mut current: &mut Blueprint = &mut Blueprint::default();

        let mut condition: usize = 0;

        for line in self.data.lines().map(str::trim_ascii) {
            if line.is_empty() {
                condition = 0;
            } else if let Some(s) = line.strip_prefix("Begin in state ") {
                state = s.trim_end_matches('.');
            } else if let Some(s) = line.strip_prefix("Perform a diagnostic checksum after ") {
                steps = s.trim_end_matches(" steps.").parse().unwrap();
            } else if let Some(s) = line.strip_prefix("In state ") {
                current = blueprints.entry(s.trim_end_matches(':')).or_default();
            } else if line == "If the current value is 0:" {
                condition = 0;
            } else if line == "If the current value is 1:" {
                condition = 1;
            } else if line == "- Move one slot to the left." {
                current.advance[condition] = -1;
            } else if line == "- Move one slot to the right." {
                current.advance[condition] = 1;
            } else if line == "- Write the value 1." {
                current.write_if[condition] = 1;
            } else if line == "- Write the value 0." {
                current.write_if[condition] = 0;
            } else if let Some(s) = line.strip_prefix("- Continue with state ") {
                current.next_state[condition] = s.trim_end_matches('.');
            } else {
                panic!("unexpected line: {line}");
            }
        }

        let mut tape = FxHashSet::default();
        let mut cursor = 0;

        for _ in 0..steps {
            let b = &blueprints[state];

            let value = usize::from(tape.contains(&cursor));
            if b.write_if[value] == 1 {
                tape.insert(cursor);
            } else {
                tape.remove(&cursor);
            }

            cursor += b.advance[value];

            state = b.next_state[value];
        }

        tape.len()
    }
}

fn main() {
    let mut args = aoc::parse_args();
    args.run(|data| {
        let puzzle: Puzzle<'_> = Puzzle::new(data);
        (puzzle.part1(), aoc::CHRISTMAS)
    });
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 3);
    }
}
