use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    day_2_step1();
}

fn day_2_step1() {
    // file
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    // pos
    let mut h_pos: i32 = 0;
    let mut depth: i32 = 0;

    // step 1
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let mut splitted = line.split(' ');
        let direction = splitted.next().unwrap();
        let value: i32 = splitted.next().unwrap().parse().unwrap();

        match &direction as &str {
            "up" => {
                depth -= value;
            }
            "down" => {
                depth += value;
            }
            "forward" => {
                h_pos += value;
            }
            _ => {}
        }
    }
    println!("aoc2021 day2/step1 = {}", h_pos * depth);
}
