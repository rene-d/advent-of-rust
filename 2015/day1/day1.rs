//! [Day 1: Not Quite Lisp](https://adventofcode.com/2015/day/1)

/// main function
fn main() {
    let args = aoc::parse_args();
    let data = &args.input;

    for line in data.lines() {
        let mut floor = 0;
        let mut position = 0;
        let mut enter = 0;
        for c in line.chars() {
            position += 1;
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => panic!("invalid character"),
            }
            if floor == -1 && enter == 0 {
                enter = position;
            }
        }
        println!("{floor}");
        println!("{enter}");
    }
}
