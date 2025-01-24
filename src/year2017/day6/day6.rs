//! [Day 6: Memory Reallocation](https://adventofcode.com/2017/day/6)

use rustc_hash::FxHashSet;

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
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

    (part1, part2)
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
    fn part1_2() {
        let puzzle = solve(TEST_INPUT);
        assert_eq!(puzzle.0, 5);
        assert_eq!(puzzle.1, 4);
    }
}
