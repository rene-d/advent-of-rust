//! [Day 13: Transparent Origami](https://adventofcode.com/2021/day/13)

use aoc::ocr::scan_5x6;

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (i32, String) {
    let n = 2000;
    let mut grid = vec![vec![0i8; n]; n];
    let mut part1 = 0;

    for line in data.lines() {
        if line.is_empty() || line.starts_with("fold") {
            break;
        }
        let (x, y) = line.split_once(',').unwrap();
        let xx = x.parse::<usize>().unwrap();
        let yy = y.parse::<usize>().unwrap();
        grid[yy][xx] = 1;
    }

    for line in data.lines() {
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

        if part1 == 0 {
            for row in &grid {
                for cell in row {
                    part1 += i32::from(*cell);
                }
            }
        }
    }

    let mut crt = String::new();
    for row in &grid {
        for cell in row {
            crt.push(if *cell == 0 { '.' } else { '#' });
        }
        crt.push('\n');
    }

    (part1, scan_5x6(&crt))
}
