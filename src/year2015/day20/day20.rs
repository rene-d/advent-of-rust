//! [Day 20: Infinite Elves and Infinite Houses](https://adventofcode.com/2015/day/20)

use rayon::prelude::*;

struct Puzzle {
    house_present: usize,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            house_present: data.trim_ascii().parse::<usize>().unwrap(),
        }
    }

    fn part1(&self) -> usize {
        const CHUNK_SIZE: usize = 65_536;

        // Estimate upper bound for chunks
        let max_chunks = (self.house_present / 10) / CHUNK_SIZE + 2;

        let result = (0..max_chunks).into_par_iter().find_map_first(|chunk_idx| {
            let start_idx = chunk_idx * CHUNK_SIZE + 1;
            let end_idx = start_idx + CHUNK_SIZE;
            let mut houses = vec![0; CHUNK_SIZE];

            for elf in 1..end_idx {
                let mut house = if elf >= start_idx {
                    elf
                } else {
                    start_idx.div_ceil(elf) * elf
                };

                while house < end_idx {
                    houses[house - start_idx] += elf * 10;
                    house += elf;
                }
            }

            for (i, &presents) in houses.iter().enumerate() {
                if presents >= self.house_present {
                    return Some(start_idx + i);
                }
            }
            None
        });

        result.unwrap_or(0)
    }

    fn part2(&self) -> usize {
        const CHUNK_SIZE: usize = 262_144; // 2^18
        let mut houses = vec![0; CHUNK_SIZE];
        let mut start_idx = 1;

        loop {
            houses.fill(0);
            let end_idx = start_idx + CHUNK_SIZE;

            // Iterate elves that can contribute
            // The constraint is 50 * elf >= house, so elf >= house / 50
            // Since house >= start_idx, we need elf >= start_idx / 50 roughly.
            // But strict lower bound is just 1..end_idx, and we filter by condition.
            for elf in 1..end_idx {
                // Constraint: house <= 50 * elf
                let max_house = elf * 50;
                if max_house < start_idx {
                    continue;
                }

                let mut house = if elf >= start_idx {
                    elf
                } else {
                    start_idx.div_ceil(elf) * elf
                };

                while house < end_idx && house <= max_house {
                    houses[house - start_idx] += elf * 11;
                    house += elf;
                }
            }

            for (i, &presents) in houses.iter().enumerate() {
                if presents >= self.house_present {
                    let house_number = start_idx + i;
                    return house_number;
                }
            }

            start_idx += CHUNK_SIZE;
        }
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, usize) {
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

    #[test]
    fn test01() {
        let puzzle = Puzzle::new("10");
        assert_eq!(puzzle.part1(), 1);
        assert_eq!(puzzle.part2(), 1);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new("30");
        assert_eq!(puzzle.part1(), 2);
        assert_eq!(puzzle.part2(), 2);
    }

    #[test]
    fn test03() {
        let puzzle = Puzzle::new("40");
        assert_eq!(puzzle.part1(), 3);
        assert_eq!(puzzle.part2(), 3);
    }

    #[test]
    fn test04() {
        let puzzle = Puzzle::new("60");
        assert_eq!(puzzle.part1(), 4);
        assert_eq!(puzzle.part2(), 4);
    }

    #[test]
    fn test05() {
        let puzzle = Puzzle::new("70");
        assert_eq!(puzzle.part1(), 4);
        assert_eq!(puzzle.part2(), 4);
    }

    #[test]
    fn test06() {
        let puzzle = Puzzle::new("80");
        assert_eq!(puzzle.part1(), 6);
        assert_eq!(puzzle.part2(), 6);
    }

    #[test]
    fn test07() {
        let puzzle = Puzzle::new("120");
        assert_eq!(puzzle.part1(), 6);
        assert_eq!(puzzle.part2(), 6);
    }

    #[test]
    fn test08() {
        let puzzle = Puzzle::new("130");
        assert_eq!(puzzle.part1(), 8);
        assert_eq!(puzzle.part2(), 6);
    }

    #[test]
    fn test09() {
        let puzzle = Puzzle::new("150");
        assert_eq!(puzzle.part1(), 8);
        assert_eq!(puzzle.part2(), 8);
    }
}
