use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use structopt::StructOpt;

/// parse command line arguments
#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = "input.txt", parse(from_os_str))]
    path: std::path::PathBuf,

    #[structopt(default_value = "100")]
    steps: usize,
}

/// main function
fn main() {
    let args = Cli::from_args();
    println!("reading data from: {}", args.path.display());
    let data = load_data(args.path);

    // grid initialization
    let mut grid = [[0u8; 100]; 100];

    // part 1
    init_lights(&mut grid, &data);
    for _ in 0..args.steps {
        switch_lights(&mut grid);
    }
    println!("{}", count_lights(&grid));

    // part 2
    init_lights(&mut grid, &data);
    for _ in 0..args.steps {
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
    for line in grid.iter() {
        for c in line.iter() {
            if *c == 1 {
                count += 1;
            }
        }
    }
    count
}

fn switch_lights(grid: &mut [[u8; 100]; 100]) {
    let mut new_grid = [[0u8; 100]; 100];
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
                    neighbors += grid[(y + y_off) as usize][(x + x_off) as usize];
                }
            }

            if grid[y as usize][x as usize] == 1 {
                // A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
                if neighbors == 2 || neighbors == 3 {
                    new_grid[y as usize][x as usize] = 1;
                }
            } else {
                // A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
                if neighbors == 3 {
                    new_grid[y as usize][x as usize] = 1;
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

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// load data from file
fn load_data(path: std::path::PathBuf) -> Vec<String> {
    let mut data = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            data.push(line);
        }
    }
    data
}
