//! [Day 6: Probably a Fire Hazard](https://adventofcode.com/2015/day/6)

use regex::Regex;

/// main function
fn main() {
    let mut args = aoc::parse_args();
    args.run(solve);
}

fn solve(data: &str) -> (u32, u32) {
    let data = data.lines().collect::<Vec<_>>();
    (part1(&data), part2(&data))
}

fn part2(data: &[&str]) -> u32 {
    let mut grid = vec![[0_i8; 1000]; 1000];

    let re = Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();

    for line in data {
        let drawn = re.captures(line).unwrap();

        let op = drawn[1].to_string();
        let x1 = drawn[2].parse::<usize>().unwrap();
        let y1 = drawn[3].parse::<usize>().unwrap();
        let x2 = drawn[4].parse::<usize>().unwrap();
        let y2 = drawn[5].parse::<usize>().unwrap();

        if op == "turn on" {
            for line in grid.iter_mut().take(x2 + 1).skip(x1) {
                for val in line.iter_mut().take(y2 + 1).skip(y1) {
                    *val += 1;
                }
            }
        } else if op == "turn off" {
            for line in grid.iter_mut().take(x2 + 1).skip(x1) {
                for val in line.iter_mut().take(y2 + 1).skip(y1) {
                    if *val > 0 {
                        *val -= 1;
                    }
                }
            }
        } else if op == "toggle" {
            for line in grid.iter_mut().take(x2 + 1).skip(x1) {
                for val in line.iter_mut().take(y2 + 1).skip(y1) {
                    *val += 2;
                }
            }
        }
    }

    let mut count: u32 = 0;
    for row in &grid {
        for cell in row {
            count += u32::try_from(*cell).unwrap();
        }
    }

    count
}

fn part1(data: &[&str]) -> u32 {
    let mut grid = vec![[0_i8; 1000]; 1000];

    let re = Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();

    for line in data {
        let drawn = re.captures(line).unwrap();

        let op = drawn[1].to_string();
        let x1 = drawn[2].parse::<usize>().unwrap();
        let y1 = drawn[3].parse::<usize>().unwrap();
        let x2 = drawn[4].parse::<usize>().unwrap();
        let y2 = drawn[5].parse::<usize>().unwrap();

        if op == "turn on" {
            for line in grid.iter_mut().take(x2 + 1).skip(x1) {
                for val in line.iter_mut().take(y2 + 1).skip(y1) {
                    *val = 1;
                }
            }
        } else if op == "turn off" {
            for line in grid.iter_mut().take(x2 + 1).skip(x1) {
                for val in line.iter_mut().take(y2 + 1).skip(y1) {
                    *val = 0;
                }
            }
        } else if op == "toggle" {
            for line in grid.iter_mut().take(x2 + 1).skip(x1) {
                for val in line.iter_mut().take(y2 + 1).skip(y1) {
                    *val = 1 - *val;
                }
            }
        }
    }

    let mut count: u32 = 0;
    for row in &grid {
        for cell in row {
            count += u32::try_from(*cell).unwrap();
        }
    }

    count
}
