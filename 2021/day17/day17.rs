//! [Day 17: Trick Shot](https://adventofcode.com/2021/day/17)

use std::env;
use std::fs;
use text_io::scan;

/// main function
fn main() {
    let abs_min: i32;
    let abs_max: i32;
    let ord_min: i32;
    let ord_max: i32;

    let args: Vec<String> = env::args().collect();

    match args.len() {
        5 => {
            abs_min = args[1].parse::<i32>().unwrap();
            abs_max = args[2].parse::<i32>().unwrap();
            ord_min = args[3].parse::<i32>().unwrap();
            ord_max = args[4].parse::<i32>().unwrap();
        }
        2 => {
            let data = fs::read_to_string(&args[1]).unwrap();
            scan!(data.bytes() => "target area: x={}..{}, y={}..{}",
                  abs_min, abs_max, ord_min, ord_max);
        }
        1 => {
            let data = fs::read_to_string("input.txt").unwrap();
            scan!(data.bytes() => "target area: x={}..{}, y={}..{}",
                  abs_min, abs_max, ord_min, ord_max);
        }
        _ => {
            panic!("Invalid number of arguments");
        }
    }

    // println!("target area: x={}..{}, y={}..{}", abs_min, abs_max, ord_min, ord_max);

    let mut part1 = 0;
    let mut part2 = 0;

    for vx0 in 0..300 {
        for vy0 in -300..2000 {
            let mut hit = false;
            let mut y_max = 0;

            let mut x = 0;
            let mut y = 0;

            let mut vx = vx0;
            let mut vy = vy0;

            for _ in 0..2000 {
                x += vx; // probe's x position increases by its x velocity
                y += vy; // probe's y position increases by its y velocity

                y_max = y_max.max(y); // the highest y position

                // Adjust the x velocity of the probe
                if vx > 0 {
                    vx -= 1;
                } else if vx < 0 {
                    vx += 1;
                }

                vy -= 1; // the probe's y velocity decreases by 1.

                if abs_min <= x && x <= abs_max && ord_min <= y && y <= ord_max {
                    hit = true;
                    break;
                }

                // we are beyond the target area
                if x > abs_max || y < ord_min {
                    break;
                }
            }

            if hit {
                part2 += 1;
                part1 = part1.max(y_max);
            }
        }
    }

    println!("{}", part1);
    println!("{}", part2);
}
