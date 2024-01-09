//! [Day 7: The Sum of Its Parts](https://adventofcode.com/2018/day/7)

use std::collections::{HashMap, HashSet};

struct Puzzle {
    deps: HashMap<char, HashSet<char>>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            deps: HashMap::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            let line: Vec<_> = line.split_ascii_whitespace().collect();

            // Step A must be finished before step B can begin.
            let a = line[1].chars().next().unwrap();
            let b = line[7].chars().next().unwrap();

            self.deps.entry(b).or_default().insert(a);
            self.deps.entry(a).or_default();
        }
    }

    /// Solve part one.
    fn part1(&self) -> String {
        let mut result = String::new();

        let n = self.deps.len();
        let mut deps = self.deps.clone();
        let mut steps: Vec<_> = deps.keys().copied().collect();
        steps.sort_unstable();

        while result.len() < n {
            for (i, step) in steps.iter().enumerate() {
                if deps[step].is_empty() {
                    // first step to have no dependency, in alphabetical order
                    result.push(*step);

                    // remove it from the dependencies list of other steps
                    for k in deps.values_mut() {
                        k.remove(step);
                    }
                    steps.remove(i);
                    break;
                }
            }
        }

        result
    }

    fn solve_part2(&self, nb_workers: usize, base_delay: u32) -> u32 {
        let mut processed = 0;

        let n = self.deps.len();
        let mut deps = self.deps.clone();
        let mut steps: Vec<_> = deps.keys().copied().collect();
        steps.sort_unstable();

        let mut workers = vec![('_', 0u32); nb_workers];

        let mut seconds = 0;

        loop {
            for (worker_step, worker_timer) in &mut workers {
                if *worker_timer == 0 {
                    // worker is doing nothing
                    continue;
                }

                *worker_timer -= 1;

                if *worker_timer == 0 {
                    // process of step has finished

                    processed += 1;

                    // remove it from the dependencies list of other steps
                    for k in deps.values_mut() {
                        k.remove(worker_step);
                    }
                }
            }

            for (worker_step, worker_timer) in &mut workers {
                if *worker_timer != 0 {
                    // worker is busy
                    continue;
                }

                // find a step to work on

                for (i, step) in steps.iter().enumerate() {
                    if deps[step].is_empty() {
                        // first step with no dependency in alphabetical order

                        // affect it to the current worker
                        *worker_step = *step;
                        *worker_timer = (*step as u32) - ('A' as u32) + 1 + base_delay;

                        // remove it to the waiting list
                        steps.remove(i);

                        break;
                    }
                }
            }

            if processed >= n {
                break;
            }

            seconds += 1;
        }

        seconds
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        self.solve_part2(5, 60)
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
        assert_eq!(puzzle.part1(), "CABDFE");
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.solve_part2(2, 0), 15);
    }
}
