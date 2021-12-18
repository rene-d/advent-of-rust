// Day 17: Trick Shot
// https://adventofcode.com/2021/day/17

use std::env;

/// main function
fn main() {
    // target area: x=269..292, y=-68..-44
    // part1: 2278
    // part2: 996

    let mut target_x1 = 269;
    let mut target_x2 = 292;
    let mut target_y1 = -68;
    let mut target_y2 = -44;


    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    match args.len() {
        5 => {
            target_x1 = args[1].parse::<i32>().unwrap();
            target_x2 = args[2].parse::<i32>().unwrap();
            target_y1 = args[3].parse::<i32>().unwrap();
            target_y2 = args[4].parse::<i32>().unwrap();
        }
        _ => {}
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

                if vx > 0 {
                    // the probe's x velocity changes by 1 toward the value 0
                    vx -= 1;
                } else if vx < 0 {
                    // increases by 1 if it is less than 0
                    vx += 1;
                } else {
                    // does not change if it is already 0
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
