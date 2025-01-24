//! [Day 15: Timing is Everything](https://adventofcode.com/2016/day/15)

use regex::Regex;

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, usize) {
    let mut discs = Vec::new();
    let mut positions = Vec::new();

    let re = Regex::new(r"Disc #\d+ has (\d+) positions; at time=0, it is at position (\d+).").unwrap();

    for line in data.lines() {
        let caps = re.captures(line).unwrap();

        discs.push(caps[2].parse::<usize>().unwrap());
        positions.push(caps[1].parse::<usize>().unwrap());
    }

    // part 1
    let mut part1 = 0;

    let nb_discsc = discs.len();
    for time in 0.. {
        if (0..nb_discsc).all(|i| (discs[i] + i + time + 1) % positions[i] == 0) {
            part1 = time;
            break;
        }
    }

    // part 2
    let mut part2 = 0;

    discs.push(0);
    positions.push(11);

    let nb_discsc = discs.len();
    for time in 0.. {
        if (0..nb_discsc).all(|i| (discs[i] + i + time + 1) % positions[i] == 0) {
            part2 = time;
            break;
        }
    }

    (part1, part2)
}
