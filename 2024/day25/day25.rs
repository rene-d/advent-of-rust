//! [Day 25: Code Chronicle](https://adventofcode.com/2024/day/25)

/// Solve part one.
fn part1(data: &str) -> u64 {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for schematics in data.split("\n\n") {
        let heights: Vec<_> = (0..5)
            .map(|x| {
                schematics
                    .lines()
                    .filter(|row| row.chars().nth(x).unwrap() == '#')
                    .count()
                    - 1
            })
            .collect();

        if schematics.starts_with("#####") {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    let mut answer = 0;

    for key in &keys {
        for lock in &locks {
            if key.iter().zip(lock.iter()).all(|(a, b)| a + b <= 5) {
                answer += 1;
            }
        }
    }

    answer
}

fn solve(data: &str) -> (u64, aoc::Christmas) {
    (part1(data), aoc::CHRISTMAS)
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
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3);
    }
}
