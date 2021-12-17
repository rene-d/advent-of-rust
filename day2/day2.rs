// Day 2: Dive!
// https://adventofcode.com/2021/day/2

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut hpos = 0;
    let mut vpos = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(line_ok) = line {
                if let Some((direction, _step)) = line_ok.rsplit_once(' ') {
                    let step = _step.parse::<i32>().unwrap();

                    if direction == "forward" {
                        hpos += step;
                    } else if direction == "down" {
                        vpos += step;
                    } else if direction == "up" {
                        vpos -= step;
                    }
                }
            }
        }
    }

    println!("{}", hpos * vpos);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
