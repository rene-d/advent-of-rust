//! [Day 1: Secret Entrance](https://adventofcode.com/2025/day/1)

#[must_use]
pub fn solve(data: &str) -> (i32, i32) {
    let mut part1_pos = 50;
    let mut part1_count_zero = 0;

    let mut part2_pos = 50;
    let mut part2_count_zero = 0;

    for line in data.lines() {
        let dir = line.chars().next().unwrap();
        let num = line[1..].parse::<i32>().unwrap();

        let step = if dir == 'L' { -1 } else { 1 };

        // part one
        part1_pos = (part1_pos + step * num).rem_euclid(100);
        if part1_pos == 0 {
            part1_count_zero += 1;
        }

        // part two
        for _ in 0..num {
            part2_pos = (part2_pos + step).rem_euclid(100);
            if part2_pos == 0 {
                part2_count_zero += 1;
            }
        }
    }

    (part1_count_zero, part2_count_zero)
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
    fn parts() {
        let (p1, p2) = solve(TEST_INPUT);
        assert_eq!(p1, 3);
        assert_eq!(p2, 6);
    }
}
