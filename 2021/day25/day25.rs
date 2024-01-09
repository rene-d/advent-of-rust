//! [Day 25: Sea Cucumber](https://adventofcode.com/2021/day/25)

/// main function
fn main() {
    let data = aoc::load_input_data_vec(25);

    let nx = data[0].len();
    let ny = data.len();
    let mut grid = vec![vec!['.'; nx]; ny];

    // load the grid
    for (y, line) in data.iter().enumerate() {
        for (x, col) in line.chars().enumerate() {
            grid[y][x] = col;
        }
    }

    // move the sea cucumbers
    let mut step = 1;
    while do_move(&mut grid) {
        step += 1;
    }
    println!("{step}");
}

fn do_move(grid: &mut [Vec<char>]) -> bool {
    let mut moved = false;

    let nx = grid[0].len();
    let ny = grid.len();

    // don't move blocked sea cucumbers
    for y in 0..ny {
        for x in 0..nx {
            if grid[y][x] == '>' && grid[y][x] == grid[y][(x + 1) % nx] {
                grid[y][x] = 'H';
            }
            if grid[y][x] == 'v' && grid[y][x] == grid[(y + 1) % ny][x] {
                grid[y][x] = 'V';
            }
        }
    }

    // During a single step, the east-facing herd moves first,
    for line in grid.iter_mut() {
        for x in 0..nx {
            if line[x] == '>' && line[(x + 1) % nx] == '.' {
                line[(x + 1) % nx] = 'H';
                line[x] = '.';
                moved = true;
            }
        }
    }

    // then the south-facing herd moves.
    for y in 0..ny {
        for x in 0..nx {
            let c = grid[y][x];
            if c == 'v' && grid[(y + 1) % ny][x] == '.' {
                grid[(y + 1) % ny][x] = 'V';
                grid[y][x] = '.';
                moved = true;
            }
        }
    }

    // restore blocked and moving sea cucumbers
    for line in grid {
        for val in line {
            match val {
                'H' => *val = '>',
                'V' => *val = 'v',
                _ => (),
            }
        }
    }

    // indicate if any sea cucumbers moved
    moved
}
