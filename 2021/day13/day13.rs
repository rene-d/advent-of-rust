//! [Day 13: Transparent Origami](https://adventofcode.com/2021/day/13)

use aoc::ocr::scan_5x6;

/// main function
fn main() {
    let data = aoc::load_input_data_vec(13);

    let n = 2000;
    let mut grid = vec![vec![0i8; n]; n];
    let mut part1 = false;

    for line in &data {
        if line.is_empty() || line.starts_with("fold") {
            break;
        }
        let (x, y) = line.split_once(',').unwrap();
        let xx = x.parse::<usize>().unwrap();
        let yy = y.parse::<usize>().unwrap();
        grid[yy][xx] = 1;
    }

    for line in &data {
        if !line.starts_with("fold") {
            continue;
        }

        if line.starts_with("fold along x=") {
            let (_, s) = line.split_once('=').unwrap();
            let fold = s.parse::<usize>().unwrap();
            for line in &mut grid {
                for x in 0..fold {
                    if line[fold + 1 + x] == 1 {
                        line[fold - 1 - x] = 1;
                    }
                }
            }
            for line in &mut grid {
                line.resize(fold, 0);
            }
        } else if line.starts_with("fold along y=") {
            let (_, s) = line.split_once('=').unwrap();
            let fold = s.parse::<usize>().unwrap();
            for y in 0..fold {
                for x in 0..grid[0].len() {
                    if grid[fold + 1 + y][x] == 1 {
                        grid[fold - 1 - y][x] = 1;
                    }
                }
            }
            grid.resize(fold, vec![0; grid[0].len()]);
        }

        if !part1 {
            part1 = true;
            let mut sum = 0i32;
            for row in &grid {
                for cell in row {
                    sum += i32::from(*cell);
                }
            }
            println!("{sum}");
        }
    }

    let mut crt = String::new();
    for row in &grid {
        for cell in row {
            crt.push(if *cell == 0 { '.' } else { '#' });
        }
        crt.push('\n');
    }
    println!("{}", scan_5x6(&crt));
}
