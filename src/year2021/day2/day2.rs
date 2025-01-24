//! [Day 2: Dive!](https://adventofcode.com/2021/day/2)

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (i32, i32) {
    (part1(data), part2(data))
}

fn part1(data: &str) -> i32 {
    let mut pos_h = 0;
    let mut pos_v = 0;

    for line in data.lines() {
        if let Some((direction, step_str)) = line.rsplit_once(' ') {
            let step = step_str.parse::<i32>().unwrap();

            if direction == "forward" {
                pos_h += step;
            } else if direction == "down" {
                pos_v += step;
            } else if direction == "up" {
                pos_v -= step;
            }
        }
    }

    pos_h * pos_v
}

fn part2(data: &str) -> i32 {
    let mut pos_h = 0;
    let mut pos_v = 0;
    let mut aim = 0;

    for line in data.lines() {
        if let Some((direction, step_str)) = line.rsplit_once(' ') {
            let step = step_str.parse::<i32>().unwrap();

            if direction == "down" {
                aim += step;
            } else if direction == "up" {
                aim -= step;
            } else if direction == "forward" {
                pos_h += step;
                pos_v += aim * step;
            }
        }
    }
    pos_h * pos_v
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test01() {
        assert_eq!(part1(TEST_INPUT), 150);
    }

    #[test]
    fn test02() {
        assert_eq!(part2(TEST_INPUT), 900);
    }
}
