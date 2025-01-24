//! [Day 9: Smoke Basin](https://adventofcode.com/2021/day/9)

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (i32, i32) {
    let data = data
        .lines()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();

    // parse the data
    let sy = data.len();
    let sx = data[0].len();
    let mut grid = vec![vec![0; sx]; sy];

    for (y, line) in data.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c.to_string().parse().unwrap();
        }
    }

    let mut basins = vec![];

    let mut risk = 0;
    for y in 0..sy {
        for x in 0..sx {
            let v = grid[y][x];
            if y > 0 && v >= grid[y - 1][x] {
                continue;
            }
            if x > 0 && v >= grid[y][x - 1] {
                continue;
            }
            if y < sy - 1 && v >= grid[y + 1][x] {
                continue;
            }
            if x < sx - 1 && v >= grid[y][x + 1] {
                continue;
            }
            risk += v + 1;

            basins.push(basin(&mut grid, y, x));
        }
    }

    // part 1: => risk

    // part 2
    basins.sort_by(|a, b| b.cmp(a));
    (risk, basins[0] * basins[1] * basins[2])
}

fn basin(grid: &mut [Vec<i32>], y: usize, x: usize) -> i32 {
    let mut stack = vec![(1, y, x)];
    let mut n = 0;
    while let Some((size, y, x)) = stack.pop() {
        if grid[y][x] == 9 {
            continue;
        }
        n += 1;
        grid[y][x] = 9;
        if y > 0 && grid[y - 1][x] != 0 {
            stack.push((size + 1, y - 1, x));
        }
        if x > 0 && grid[y][x - 1] != 0 {
            stack.push((size + 1, y, x - 1));
        }
        if y < grid.len() - 1 && grid[y + 1][x] != 0 {
            stack.push((size + 1, y + 1, x));
        }
        if x < grid[0].len() - 1 && grid[y][x + 1] != 0 {
            stack.push((size + 1, y, x + 1));
        }
    }
    n
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_solve() {
        assert_eq!(solve(TEST_INPUT), (15, 1134));
    }
}
