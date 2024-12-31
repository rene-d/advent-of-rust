//! [Day 18: Like a GIF For Your Yard](https://adventofcode.com/2015/day/18)

const STEPS: usize = 100;

/// main function
fn main() {
    let args = aoc::parse_args();
    let data = args
        .input
        .lines()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();

    // grid initialization
    let mut grid = [[0_u8; 100]; 100];

    // part 1
    init_lights(&mut grid, &data);
    for _ in 0..STEPS {
        switch_lights(&mut grid);
    }
    println!("{}", count_lights(&grid));

    // part 2
    init_lights(&mut grid, &data);
    for _ in 0..STEPS {
        corners_on(&mut grid);
        switch_lights(&mut grid);
    }
    corners_on(&mut grid);
    println!("{}", count_lights(&grid));
}

fn corners_on(grid: &mut [[u8; 100]; 100]) {
    grid[0][0] = 1;
    grid[0][99] = 1;
    grid[99][0] = 1;
    grid[99][99] = 1;
}

fn init_lights(grid: &mut [[u8; 100]; 100], data: &[String]) {
    for (y, line) in data.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                grid[y][x] = 1;
            } else {
                grid[y][x] = 0;
            }
        }
    }
}

fn count_lights(grid: &[[u8; 100]; 100]) -> u32 {
    let mut count = 0;
    for line in grid {
        for c in line {
            if *c == 1 {
                count += 1;
            }
        }
    }
    count
}

fn switch_lights(grid: &mut [[u8; 100]; 100]) {
    let mut new_grid = [[0_u8; 100]; 100];
    for y in 0..100 {
        for x in 0..100 {
            let mut neighbors = 0;
            for y_off in -1..=1 {
                for x_off in -1..=1 {
                    if y_off == 0 && x_off == 0 {
                        continue;
                    }
                    if y + y_off < 0 || y + y_off >= 100 {
                        continue;
                    }
                    if x + x_off < 0 || x + x_off >= 100 {
                        continue;
                    }
                    neighbors += grid[usize::try_from(y + y_off).unwrap()]
                        [usize::try_from(x + x_off).unwrap()];
                }
            }

            if grid[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()] == 1 {
                // A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
                if neighbors == 2 || neighbors == 3 {
                    new_grid[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()] = 1;
                }
            } else {
                // A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
                if neighbors == 3 {
                    new_grid[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()] = 1;
                }
            }
        }
    }
    //    grid.copy_from_slice(&new_grid);
    for y in 0..100 {
        for x in 0..100 {
            grid[y][x] = new_grid[y][x];
        }
    }
}
