//! [Day 6: Memory Reallocation](https://adventofcode.com/2017/day/6)

use rustc_hash::FxHashSet;

struct Puzzle {
    part1: u32,
    part2: u32,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn solve(data: &str) -> Self {
        let mut banks: Vec<u32> = data
            .split_ascii_whitespace()
            .map_while(|i| i.parse().ok())
            .collect();
        let size = u32::try_from(banks.len()).unwrap();

        let mut seen = FxHashSet::default();
        let mut iterations = 0;

        let mut part1 = 0;
        let part2;

        let mut boucle = Vec::new();

        loop {
            let state = banks.clone();

            if seen.contains(&state) {
                // we have detected a loop
                if part1 == 0 {
                    part1 = iterations;
                    boucle.clone_from(&state);
                } else if boucle == state {
                    // count iterations within the first loop
                    part2 = iterations - part1;
                    break;
                }
            }

            seen.insert(state);

            // find the max
            let mut blocks_max = 0;
            let mut index_max = 0;
            for (i, &blocks) in banks.iter().enumerate() {
                if blocks_max < blocks {
                    index_max = i;
                    blocks_max = blocks;
                }
            }

            // redistribute blocks
            banks[index_max] = 0;
            let realloc = 1.max(blocks_max / size);
            while blocks_max > 0 {
                index_max = (index_max + 1) % banks.len();
                banks[index_max] += realloc;
                blocks_max = blocks_max.saturating_sub(realloc);
            }

            iterations += 1;
        }

        Self { part1, part2 }
    }
}

fn main() {
    let args = aoc::parse_args();
    let puzzle = Puzzle::solve(&args.input);
    println!("{}", puzzle.part1);
    println!("{}", puzzle.part2);
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_2() {
        let data = aoc::load_input_data("test.txt");
        let puzzle = Puzzle::solve(&data);
        assert_eq!(puzzle.part1, 5);
        assert_eq!(puzzle.part2, 4);
    }
}
