//! [Day 2: Cube Conundrum](https://adventofcode.com/2023/day/2)

struct Puzzle<'a> {
    data: &'a str, // raw puzzle input, it is parsed when needed
}

impl<'a> Puzzle<'a> {
    const fn new(data: &'a str) -> Self {
        Self { data }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut sum = 0;
        for line in self.data.lines() {
            let s: Vec<_> = line.split(':').collect();

            let valid = || {
                let subsets = s.last().unwrap();
                for subset in subsets.split(';') {
                    for cube in subset.split(',') {
                        let cube: Vec<_> = cube.trim().split(' ').collect();

                        let n = cube.first().unwrap().parse::<u32>().unwrap();
                        let color = cube.last();

                        let max_value = match color {
                            Some(&"red") => 12,
                            Some(&"green") => 13,
                            Some(&"blue") => 14,
                            _ => panic!(),
                        };

                        if n > max_value {
                            return false;
                        }
                    }
                }

                true
            };

            if valid() {
                // get the id from "Game <id>"
                let id = s
                    .first()
                    .unwrap()
                    .strip_prefix("Game ")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();

                sum += id;
            }
        }
        sum
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let mut sum = 0;
        for line in self.data.lines() {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            // get the string at the right of ':'
            let subsets = line.split(':').last().unwrap();

            // split the sets of cubes
            for subset in subsets.split(';') {
                // split the cubes
                for cube in subset.split(',') {
                    // cube is "<n> <color>"
                    let cube: Vec<_> = cube.trim().split(' ').collect();

                    let n = cube.first().unwrap().parse::<u32>().unwrap();
                    let color = cube.last();

                    // adjust max count of each colors
                    match color {
                        Some(&"red") => red = red.max(n),
                        Some(&"green") => green = green.max(n),
                        Some(&"blue") => blue = blue.max(n),
                        _ => panic!(),
                    };
                }
            }

            // compute the power of the set of cubes
            let power = red * green * blue;

            // sum of the power of 'these) sets
            sum += power;
        }

        sum
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
        assert_eq!(puzzle.part1(), 8);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 2286);
    }
}
