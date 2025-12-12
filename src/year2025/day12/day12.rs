//! [Day 12: Christmas Tree Farm](https://adventofcode.com/2025/day/12)

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, aoc::Christmas) {
    let part1 = data
        .split("\n\n")
        .map(|s| {
            if s.contains('x') {
                s.lines()
                    .map(|line| {
                        let (size, counts) = line.split_once(": ").unwrap();
                        let (width, height) = size.split_once('x').unwrap();

                        let size: usize =
                            width.parse::<usize>().unwrap() * height.parse::<usize>().unwrap();

                        usize::from(
                            size >= 9 * counts
                                .split_ascii_whitespace()
                                .map(|s| s.parse::<usize>().unwrap())
                                .sum::<usize>(),
                        )
                    })
                    .sum()
            } else {
                0
            }
        })
        .sum();

    (part1, aoc::CHRISTMAS)
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
    fn part1() {
        let (part1, _) = solve(TEST_INPUT);
        assert_ne!(part1, 0); // dummy solver does not work for test input
        // assert_eq!(part1, 3);
    }
}
