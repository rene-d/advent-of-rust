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

        // part two (optimized)
        if dir == 'R' {
            part2_pos += num;
            part2_count_zero += part2_pos / 100;
            part2_pos = part2_pos.rem_euclid(100);
        } else {
            part2_count_zero += num / 100;
            let num = num.rem_euclid(100);
            if (1..=num).contains(&part2_pos) {
                part2_count_zero += 1;
            }
            part2_pos = (part2_pos - num).rem_euclid(100);
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
