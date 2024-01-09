//! [Day 5: Hydrothermal Venture](https://adventofcode.com/2021/day/5)

use regex::Regex;

/// main function
fn main() {
    let data = aoc::load_input_data_vec(5);

    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();

    // --- Part One ---
    let mut grid = vec![[0_i16; 1000]; 1000];

    for line in &data {
        let drawn = re.captures(line).unwrap();

        let mut x1 = drawn[1].parse::<usize>().unwrap();
        let mut y1 = drawn[2].parse::<usize>().unwrap();
        let mut x2 = drawn[3].parse::<usize>().unwrap();
        let mut y2 = drawn[4].parse::<usize>().unwrap();

        if x1 == x2 {
            if y1 > y2 {
                std::mem::swap(&mut y1, &mut y2);
            }
            for y in y1..=y2 {
                grid[x1][y] += 1;
            }
        } else if y1 == y2 {
            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
            }
            for row in grid.iter_mut().take(x2 + 1).skip(x1) {
                row[y1] += 1;
            }
        }
    }

    let mut sum = 0;
    for line in &grid {
        for val in line {
            if *val > 1 {
                sum += 1;
            }
        }
    }
    println!("{sum:?}");

    // --- Part Two ---
    for line in &data {
        let drawn = re.captures(line).unwrap();

        let mut x1 = drawn[1].parse::<usize>().unwrap();
        let mut y1 = drawn[2].parse::<usize>().unwrap();
        let mut x2 = drawn[3].parse::<usize>().unwrap();
        let mut y2 = drawn[4].parse::<usize>().unwrap();

        if x1 != x2 && y1 != y2 {
            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
                std::mem::swap(&mut y1, &mut y2);
            }
            if y1 < y2 {
                for (x, row) in grid.iter_mut().enumerate().take(x2 + 1).skip(x1) {
                    row[y1 + (x - x1)] += 1;
                }
            } else {
                for (x, row) in grid.iter_mut().enumerate().take(x2 + 1).skip(x1) {
                    row[y1 - (x - x1)] += 1;
                }
            }
        }
    }

    let mut sum2 = 0;
    for line in &grid {
        for val in line {
            if *val > 1 {
                sum2 += 1;
            }
        }
    }
    println!("{sum2:?}");
}
