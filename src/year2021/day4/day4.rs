//! [Day 4: Giant Squid](https://adventofcode.com/2021/day/4)

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

    let drawn = data[0]
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // load the grids
    let mut grids = vec![];
    let mut i = 2;
    while i < data.len() {
        let mut grid = [[0_i32; 5]; 5];

        for y in 0..5 {
            let line = data[i + y]
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<i32>>();

            grid[y][..5].clone_from_slice(&line[..5]);
        }

        grids.push(grid);

        i += 6;
    }

    let mut first_win = 0;
    let mut last_draw = 0;

    for draw in drawn {
        for grid in &mut grids {
            if grid[0][0] == -2 {
                // grid invalidated
                continue;
            }

            for line in grid.iter_mut() {
                for val in line {
                    if *val == draw {
                        *val = -1; // clear the case
                    }
                }
            }

            if win(grid) {
                last_draw = draw * sum(grid);
                if first_win == 0 {
                    first_win = last_draw;
                }
                grid[0][0] = -2; // invalidate the grid
            }
        }
    }

    (first_win, last_draw)
}

/// sum computes the sum of non-cleared cases
fn sum(grid: &[[i32; 5]; 5]) -> i32 {
    let mut s = 0;
    for line in grid {
        for val in line {
            if *val != -1 {
                s += *val;
            }
        }
    }
    s
}

/// `has_win` returns true if the grid has an cleared row or column
fn win(grid: &[[i32; 5]; 5]) -> bool {
    for i in 0..5 {
        if grid[i][0] == -1
            && grid[i][1] == -1
            && grid[i][2] == -1
            && grid[i][3] == -1
            && grid[i][4] == -1
        {
            return true;
        }

        if grid[0][i] == -1
            && grid[1][i] == -1
            && grid[2][i] == -1
            && grid[3][i] == -1
            && grid[4][i] == -1
        {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_solve() {
        assert_eq!(solve(TEST_INPUT), (4512, 1924));
    }
}
