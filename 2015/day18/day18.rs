//! [Day 18: Like a GIF For Your Yard](https://adventofcode.com/2015/day/18)

type Grid = [[u8; 100]; 100];
const STEPS: u32 = 100;

/// main function
fn main() {
    let mut args = aoc::parse_args();
    args.run(solve);
}

fn solve(data: &str) -> (u32, u32) {
    (part1(data), part2(data))
}

fn part1(data: &str) -> u32 {
    // grid initialization
    let mut grid = [[0_u8; 100]; 100];

    // part 1
    init_lights(&mut grid, data);
    for _step in 0..STEPS {
        switch_lights(&mut grid);

        #[cfg(feature = "ascii")]
        print_ascii(&grid);

        #[cfg(feature = "anim")]
        export_frame(1, _step, &grid);
    }

    count_lights(&grid)
}

fn part2(data: &str) -> u32 {
    // grid initialization
    let mut grid = [[0_u8; 100]; 100];

    // part 2
    init_lights(&mut grid, data);
    for _step in 0..STEPS {
        corners_on(&mut grid);
        switch_lights(&mut grid);

        #[cfg(feature = "ascii")]
        print_ascii(&grid);

        #[cfg(feature = "anim")]
        export_frame(2, _step, &grid);
    }
    corners_on(&mut grid);

    count_lights(&grid)
}

fn corners_on(grid: &mut Grid) {
    grid[0][0] = 1;
    grid[0][99] = 1;
    grid[99][0] = 1;
    grid[99][99] = 1;
}

fn init_lights(grid: &mut Grid, data: &str) {
    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            if c == b'#' {
                grid[y][x] = 1;
            } else {
                grid[y][x] = 0;
            }
        }
    }
}

fn count_lights(grid: &Grid) -> u32 {
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

fn switch_lights(grid: &mut Grid) {
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

#[cfg(feature = "ascii")]
fn print_ascii(grid: &Grid) {
    print!("\x1b[H\x1b[2J");
    for y in 0..100 {
        for x in 0..100 {
            print!("{}", if grid[y][x] == 1 { 'X' } else { ' ' });
        }
        println!();
    }
    std::thread::sleep(std::time::Duration::from_millis(50));
}

#[cfg(feature = "anim")]
fn export_frame(part: u8, step: u32, grid: &Grid) {
    let mut imgbuf = image::ImageBuffer::new(200, 200);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let g = (0.4 * y as f32) as u8;
        let b = (0.4 * x as f32) as u8;
        *pixel = image::Rgb([0, g, b]);
    }

    for y in 0..100 {
        for x in 0..100 {
            let color = grid[y][x];

            if color == 1 {
                let x = x as u32;
                let y = y as u32;
                for k in 0..4 {
                    let pixel = imgbuf.get_pixel_mut(x * 2 + k % 2, y * 2 + k / 2);
                    *pixel = image::Rgb([255, 0, 0]);
                }
            }
        }
    }

    imgbuf.save(format!("frame_{part}_{step:03}.png")).unwrap();
}
