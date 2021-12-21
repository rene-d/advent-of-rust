use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use structopt::StructOpt;

/// parse command line arguments
#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = "input.txt", parse(from_os_str))]
    path: std::path::PathBuf,
}

type Grid = [[u8; 1000]; 1000];

const PIXEL_UNKNOWN: u8 = 0;
const PIXEL_OFF: u8 = 1;
const PIXEL_ON: u8 = 2;

/// main function
fn main() {
    let args = Cli::from_args();
    println!("reading data from: {}", args.path.display());
    let data = load_data(args.path);

    let decoder = data[0]
        .chars()
        .map(|c| match c {
            '.' => PIXEL_OFF,
            '#' => PIXEL_ON,
            _ => panic!("invalid input"),
        })
        .collect::<Vec<u8>>();

    if decoder.len() != 512 {
        panic!("invalid decoder length");
    }

    let mut grid: Grid = [[PIXEL_UNKNOWN; 1000]; 1000];

    let sx = data[2].len();
    let sy = data.len() - 2;

    let offset_x = (1000 - sx) / 2;
    let offset_y = (1000 - sy) / 2;

    for y in 0..sy {
        for x in 0..sx {
            let pixel = match data[y + 2].chars().nth(x).unwrap() {
                '#' => PIXEL_ON,
                '.' => PIXEL_OFF,
                _ => panic!("invalid input"),
            };
            grid[offset_y + y as usize][offset_x + x as usize] = pixel;
        }
    }

    let mut default_pixel = PIXEL_OFF; // default is off

    display(&grid);

    for step in 1..=50 {
        default_pixel = enhance(&mut grid, &decoder, default_pixel);
        display(&grid);

        if step == 2 {
            println!("lit pixels: {}", count_lit(&grid));
        }
    }

    println!("lit pixels: {}", count_lit(&grid));
}

fn display(grid: &Grid) {
    let extense = range(&grid);

    if grid.len() > 10 {
        return;
    }

    println!(
        "{} x {}  - {:?}  ",
        extense.2 - extense.0 + 1,
        extense.3 - extense.1 + 1,
        extense
    );
    println!("lit pixels: {}", count_lit(grid));

    for y in extense.1..=extense.3 {
        for x in extense.0..=extense.2 {
            print!(
                "{}",
                match grid[y][x] {
                    PIXEL_OFF => '.',
                    PIXEL_ON => '#',
                    _ => panic!("unknown pixel"),
                }
            );
        }
        println!();
    }
    println!();
}

fn count_lit(grid: &Grid) -> usize {
    let mut lit = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let pixel = grid[y][x];
            if pixel == PIXEL_ON {
                lit += 1;
            }
        }
    }
    lit
}

fn range(grid: &Grid) -> (usize, usize, usize, usize) {
    let mut minx = usize::MAX;
    let mut maxx = usize::MIN;
    let mut miny = usize::MAX;
    let mut maxy = usize::MIN;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let pixel = grid[y][x];
            if pixel != PIXEL_UNKNOWN {
                minx = minx.min(x);
                maxx = maxx.max(x);
                miny = miny.min(y);
                maxy = maxy.max(y);
            }
        }
    }
    (minx, miny, maxx, maxy)
}

fn enhance(grid: &mut Grid, decoder: &[u8], default_pixel: u8) -> u8 {
    let mut new_grid: Grid = [[PIXEL_UNKNOWN; 1000]; 1000];
    let extense = range(grid);

    for y in extense.1 - 1..=extense.3 + 1 {
        for x in extense.0 - 1..=extense.2 + 1 {
            let mut sum: usize = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    let mut pixel = grid[ny as usize][nx as usize];
                    if pixel == PIXEL_UNKNOWN {
                        pixel = default_pixel;
                    }
                    match pixel {
                        PIXEL_OFF => sum = sum * 2,    // pixel off
                        PIXEL_ON => sum = sum * 2 + 1, // pixel on
                        _ => panic!("unknown pixel"),
                    }
                }
            }
            new_grid[y][x] = decoder[sum];
        }
    }
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            grid[y][x] = new_grid[y][x];
        }
    }

    match default_pixel {
        PIXEL_OFF => decoder[0],          // pixel off decoded
        PIXEL_ON => decoder[0b111111111], // pixel on decoded
        _ => panic!("unknown pixel"),
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
