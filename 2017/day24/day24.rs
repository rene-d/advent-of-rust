//! [Day 24: Electromagnetic Moat](https://adventofcode.com/2017/day/24)

use std::collections::VecDeque;

struct Puzzle {
    ports: Vec<(u32, u32)>,
    max_strength: u32,        // part 1 answer
    max_length_strength: u32, // part 2 answer
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        let ports = data
            .lines()
            .filter_map(|line| line.split_once('/'))
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .collect();

        Self {
            ports,
            max_strength: 0,
            max_length_strength: 0,
        }
    }

    fn solve(&mut self) {
        let mut max_length = 0;
        let mut queue = VecDeque::new();

        self.max_strength = 0;
        self.max_length_strength = 0;

        queue.push_front((0u32, 0, 1, self.ports.clone()));
        while let Some((pin, strength, length, ports)) = queue.pop_back() {
            //
            self.max_strength = self.max_strength.max(strength);

            // match expression is equivalent to the following, clippy is sometimes really weird
            // if length > max_length {
            //     max_length = length;
            //     self.max_length_strength = strength;
            // } else if length == max_length {
            //     self.max_length_strength = self.max_length_strength.max(strength);
            // }

            match length {
                _ if length > max_length => {
                    max_length = length;
                    self.max_length_strength = strength;
                }
                _ if length == max_length => {
                    self.max_length_strength = self.max_length_strength.max(strength);
                }
                _ => {}
            };

            for (i, &(a, b)) in ports.iter().enumerate() {
                let c = if a == pin {
                    b
                } else if b == pin {
                    a
                } else {
                    continue;
                };

                let mut np = ports.clone();
                np.remove(i);

                queue.push_front((c, strength + a + b, length + 1, np));
            }
        }
    }

    /// Solve part one.
    const fn part1(&self) -> u32 {
        self.max_strength
    }

    /// Solve part two.
    const fn part2(&self) -> u32 {
        self.max_length_strength
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new(&args.input);
    puzzle.solve();
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_2() {
        let data = aoc::load_input_data("test.txt");
        let mut puzzle = Puzzle::new(&data);
        puzzle.solve();
        assert_eq!(puzzle.part1(), 31);
        assert_eq!(puzzle.part2(), 19);
    }
}
