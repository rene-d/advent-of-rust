//! [Day 25: Sea Cucumber](https://adventofcode.com/2021/day/25)

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (i32, aoc::Christmas) {
    let data = data.lines().collect::<Vec<&str>>();

    let nx = data[0].len();
    let ny = data.len();
    let mut grid = vec![vec![b'.'; nx]; ny];

    // load the grid
    for (y, line) in data.iter().enumerate() {
        for (x, col) in line.bytes().enumerate() {
            grid[y][x] = col;
        }
    }

    // move the sea cucumbers
    let mut step = 1;
    while do_move(&mut grid) {
        step += 1;
    }

    (step, aoc::CHRISTMAS)
}

fn do_move(grid: &mut [Vec<u8>]) -> bool {
    let mut moved = false;

    let nx = grid[0].len();
    let ny = grid.len();

    // don't move blocked sea cucumbers
    for y in 0..ny {
        for x in 0..nx {
            if grid[y][x] == b'>' && grid[y][x] == grid[y][(x + 1) % nx] {
                grid[y][x] = b'H';
            }
            if grid[y][x] == b'v' && grid[y][x] == grid[(y + 1) % ny][x] {
                grid[y][x] = b'V';
            }
        }
    }

    // During a single step, the east-facing herd moves first,
    for line in grid.iter_mut() {
        for x in 0..nx {
            if line[x] == b'>' && line[(x + 1) % nx] == b'.' {
                line[(x + 1) % nx] = b'H';
                line[x] = b'.';
                moved = true;
            }
        }
    }

    // then the south-facing herd moves.
    for y in 0..ny {
        for x in 0..nx {
            let c = grid[y][x];
            if c == b'v' && grid[(y + 1) % ny][x] == b'.' {
                grid[(y + 1) % ny][x] = b'V';
                grid[y][x] = b'.';
                moved = true;
            }
        }
    }

    // restore blocked and moving sea cucumbers
    for line in grid {
        for val in line {
            match val {
                b'H' => *val = b'>',
                b'V' => *val = b'v',
                _ => (),
            }
        }
    }

    // indicate if any sea cucumbers moved
    moved
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn part1() {
        assert_eq!(solve(TEST_INPUT).0, 58);
    }
}
