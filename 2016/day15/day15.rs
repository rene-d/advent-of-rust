//! [Day 15: Timing is Everything](https://adventofcode.com/2016/day/15)

use regex::Regex;

fn main() {
    let data = aoc::load_input_data(15);

    let mut discs = Vec::new();
    let mut positions = Vec::new();

    for line in data.lines() {
        let re = Regex::new(r"Disc #\d+ has (\d+) positions; at time=0, it is at position (\d+).").unwrap();

        let caps = re.captures(line).unwrap();

        discs.push(caps[2].parse::<usize>().unwrap());
        positions.push(caps[1].parse::<usize>().unwrap());
    }

    // part 1
    let nb_discsc = discs.len();
    for time in 0.. {
        if (0..nb_discsc).all(|i| (discs[i] + i + time + 1) % positions[i] == 0) {
            println!("{time}");
            break;
        }
    }

    // part 2
    discs.push(0);
    positions.push(11);

    let nb_discsc = discs.len();
    for time in 0.. {
        if (0..nb_discsc).all(|i| (discs[i] + i + time + 1) % positions[i] == 0) {
            println!("{time}");
            break;
        }
    }
}
