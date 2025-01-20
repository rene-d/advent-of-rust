//! [Day 21: Keypad Conundrum](https://adventofcode.com/2024/day/21)

use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

type ButtonSequences = FxHashMap<(char, char), Vec<String>>;

fn compute_sequences(keypad: &[&str]) -> ButtonSequences {
    let mut positions = FxHashMap::default();

    // size of the keypad
    let size_x = i32::try_from(keypad[0].len()).unwrap();
    let size_y = i32::try_from(keypad.len()).unwrap();

    // positions of each button
    for (y, row) in keypad.iter().enumerate() {
        for (x, button) in row.chars().enumerate() {
            if button != ' ' {
                let x = i32::try_from(x).unwrap();
                let y = i32::try_from(y).unwrap();
                positions.insert(button, (x, y));
            }
        }
    }

    // find all paths between each pair of buttons
    let mut sequences: ButtonSequences = FxHashMap::default();

    for &from_button in positions.keys() {
        for &to_button in positions.keys() {
            // same button
            if from_button == to_button {
                // nota: A to activate/push the button
                sequences.insert((from_button, to_button), vec!["A".to_string(); 1]);
                continue;
            }
            let mut possibilities = Vec::new();
            let mut queue = VecDeque::new();
            let mut shortest = usize::MAX;
            let mut visited = FxHashMap::default();

            queue.push_front((positions[&from_button], String::new()));
            visited.insert(positions[&from_button], 0);

            while let Some(((x, y), moves)) = queue.pop_back() {
                // we reach the end
                if (x, y) == positions[&to_button] {
                    if moves.len() < shortest {
                        shortest = moves.len();
                        possibilities.clear();
                    }

                    if moves.len() == shortest {
                        possibilities.push(format!("{moves}A"));
                    }
                    continue;
                }

                // try all directions
                for (nx, ny, nm) in [
                    (x - 1, y, '<'),
                    (x + 1, y, '>'),
                    (x, y - 1, '^'),
                    (x, y + 1, 'v'),
                ] {
                    // outside the keypad
                    if nx < 0 || nx >= size_x || ny < 0 || ny >= size_y {
                        continue;
                    }

                    let button = keypad[usize::try_from(ny).unwrap()]
                        .chars()
                        .nth(usize::try_from(nx).unwrap())
                        .unwrap();

                    // no button
                    if button == ' ' {
                        continue;
                    }

                    // if not yet visited of found a shorter path
                    if *visited.get(&(nx, ny)).unwrap_or(&usize::MAX) >= moves.len() {
                        queue.push_front(((nx, ny), format!("{moves}{nm}")));
                        visited.insert((nx, ny), moves.len());
                    }
                }
            }

            sequences.insert((from_button, to_button), possibilities);
        }
    }

    sequences
}

struct Solver {
    numerical_sequences: ButtonSequences,
    directional_sequences: ButtonSequences,
}

impl Solver {
    fn new() -> Self {
        // the layout of the numerical keypad
        // +---+---+---+
        // | 7 | 8 | 9 |
        // +---+---+---+
        // | 4 | 5 | 6 |
        // +---+---+---+
        // | 1 | 2 | 3 |
        // +---+---+---+
        //     | 0 | A |
        //     +---+---+
        let numerical_keypad = ["789", "456", "123", " 0A"];

        // the layout of the directional keypad
        //     +---+---+
        //     | ^ | A |
        // +---+---+---+
        // | < | v | > |
        // +---+---+---+
        let directional_keypad = [" ^A", "<v>"];

        Self {
            numerical_sequences: compute_sequences(&numerical_keypad),
            directional_sequences: compute_sequences(&directional_keypad),
        }
    }

    /// find all combinations of sequences to enter the code
    fn find_code_seqs(&self, code: &str) -> Vec<String> {
        let mut c = Vec::new();
        for i in 0..code.len() {
            let button_from = if i == 0 {
                'A' // we start at button A
            } else {
                code.chars().nth(i - 1).unwrap()
            };
            let button_to = code.chars().nth(i).unwrap();

            let seqs = self.numerical_sequences[&(button_from, button_to)].clone(); // ways to k1→k2

            c.push(seqs);
        }

        c.iter()
            .multi_cartesian_product()
            .map(|k| k.iter().join(""))
            .collect::<Vec<_>>()
    }

    /// compute recursively the length of the sequence to play the `targetted_seq`
    /// with `robots` that control directional keypads
    fn compute_seq_length(
        &self,
        targetted_seq: &str,
        robots: u32,
        cache: &mut FxHashMap<(String, u32), u64>,
    ) -> u64 {
        if let Some(found) = cache.get(&(targetted_seq.to_string(), robots)) {
            return *found;
        }

        if robots <= 1 {
            return (0..targetted_seq.len())
                .map(|i| {
                    let k1 = if i == 0 {
                        'A'
                    } else {
                        targetted_seq.chars().nth(i - 1).unwrap()
                    };
                    let k2 = targetted_seq.chars().nth(i).unwrap();

                    // all seqs have same length
                    self.directional_sequences[&(k1, k2)][0].len() as u64
                })
                .sum();
        }

        let result = (0..targetted_seq.len())
            .map(|i| {
                let button_from = if i == 0 {
                    'A'
                } else {
                    targetted_seq.chars().nth(i - 1).unwrap()
                };
                let button_to = targetted_seq.chars().nth(i).unwrap();

                self.directional_sequences[&(button_from, button_to)]
                    .iter()
                    .map(|seq| self.compute_seq_length(seq, robots - 1, cache))
                    .min()
                    .unwrap()
            })
            .sum();

        cache.insert((targetted_seq.to_string(), robots), result);

        result
    }

    /// computes the compleixity of to enter `code` with a chain of `robots` robots
    fn complexity(&self, code: &str, robots: u32) -> u64 {
        let seqs = self.find_code_seqs(code);

        let mut cache = FxHashMap::default();

        let min_length = seqs
            .iter()
            .map(|seq| self.compute_seq_length(seq, robots, &mut cache))
            .min()
            .unwrap();

        let num_code = code
            .chars()
            .map_while(|c| c.to_digit(10))
            .fold(0, |acc, d| acc * 10 + d);

        min_length * u64::from(num_code)
    }
}

struct Puzzle {
    codes: Vec<String>,
    solver: Solver,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            codes: data.lines().map(std::string::ToString::to_string).collect(),
            solver: Solver::new(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        self.codes
            .iter()
            .map(|code| self.solver.complexity(code, 2))
            .sum()
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        self.codes
            .iter()
            .map(|code| self.solver.complexity(code, 25))
            .sum()
    }
}

fn solve(data: &str) -> (u64, u64) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

fn main() {
    let mut args = aoc::parse_args();
    args.run(solve);
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 126384);
    }
}
