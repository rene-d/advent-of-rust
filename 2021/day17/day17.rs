// Day 17: Trick Shot
// https://adventofcode.com/2021/day/17

use std::cmp::Ordering;
use std::env;
use std::fs;
use text_io::scan;

/// main function
fn main() {
    let target_x1: i32;
    let target_x2: i32;
    let target_y1: i32;
    let target_y2: i32;

    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    match args.len() {
        5 => {
            target_x1 = args[1].parse::<i32>().unwrap();
            target_x2 = args[2].parse::<i32>().unwrap();
            target_y1 = args[3].parse::<i32>().unwrap();
            target_y2 = args[4].parse::<i32>().unwrap();
        }
        2 => {
            let data = fs::read_to_string(args[1].to_string()).unwrap();
            scan!(data.bytes() => "target area: x={}..{}, y={}..{}",
                  target_x1, target_x2, target_y1, target_y2);
        }
        1 => {
            let data = fs::read_to_string("input.txt").unwrap();
            scan!(data.bytes() => "target area: x={}..{}, y={}..{}",
                  target_x1, target_x2, target_y1, target_y2);
        }
        _ => {
            panic!("Invalid number of arguments");
        }
    }

    println!(
        "target area: x={}..{}, y={}..{}",
        target_x1, target_x2, target_y1, target_y2
    );

    let mut part1 = 0;
    let mut part2 = 0;

    for vx0 in 0..1000 {
        for vy0 in -200..1000 {
            let mut hit = false;
            let mut y_max = 0;

            let mut x = 0;
            let mut y = 0;

            let mut vx = vx0;
            let mut vy = vy0;

            for _ in 0..1000 {
                x += vx; // probe's x position increases by its x velocity
                y += vy; // probe's y position increases by its y velocity

                if y > y_max {
                    y_max = y; // the highest y position
                }

                // Decrement the velocity of the probe
                match vx.cmp(&0) {
                    Ordering::Greater => vx -= 1,
                    Ordering::Less => vx += 1,
                    Ordering::Equal => {}
                }

                vy -= 1; // the probe's y velocity decreases by 1.

                if target_x1 <= x && x <= target_x2 && target_y1 <= y && y <= target_y2 {
                    hit = true;
                }
            }

            if hit {
                part2 += 1;
                if part1 < y_max {
                    part1 = y_max;
                }
            }
        }
    }

    println!("{}", part1);
    println!("{}", part2);
}
