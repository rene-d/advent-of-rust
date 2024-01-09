//! [Day 20: Trench Map](https://adventofcode.com/2021/day/20)

type Grid = Vec<Vec<u8>>;

const PIXEL_UNKNOWN: u8 = 0;
const PIXEL_OFF: u8 = 1;
const PIXEL_ON: u8 = 2;

/// main function
fn main() {
    let data = aoc::load_input_data_vec(20);

    let decoder = data[0]
        .chars()
        .map(|c| match c {
            '.' => PIXEL_OFF,
            '#' => PIXEL_ON,
            _ => panic!("invalid input"),
        })
        .collect::<Vec<u8>>();

    assert!(decoder.len() == 512, "invalid decoder length");

    let mut grid: Grid = vec![vec![PIXEL_UNKNOWN; 1000]; 1000];

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
            grid[offset_y + y][offset_x + x] = pixel;
        }
    }

    let mut default_pixel = PIXEL_OFF; // default is off

    display(&grid);

    for step in 1..=50 {
        default_pixel = enhance(&mut grid, &decoder, default_pixel);
        display(&grid);

        if step == 2 {
            println!("{}", count_lit(&grid));
        }
    }

    println!("{}", count_lit(&grid));
}

fn display(grid: &Grid) {
    let extense = range(grid);

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

    for line in grid.iter().take(extense.3 + 1).skip(extense.1) {
        for val in line.iter().take(extense.2 + 1).skip(extense.0) {
            print!(
                "{}",
                match *val {
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
    for line in grid {
        for val in line {
            if *val == PIXEL_ON {
                lit += 1;
            }
        }
    }
    lit
}

fn range(grid: &Grid) -> (usize, usize, usize, usize) {
    let mut min_abs = usize::MAX;
    let mut max_abs = usize::MIN;
    let mut min_ord = usize::MAX;
    let mut max_ord = usize::MIN;

    for (y, line) in grid.iter().enumerate() {
        for (x, val) in line.iter().enumerate() {
            if *val != PIXEL_UNKNOWN {
                min_abs = min_abs.min(x);
                max_abs = max_abs.max(x);
                min_ord = min_ord.min(y);
                max_ord = max_ord.max(y);
            }
        }
    }
    (min_abs, min_ord, max_abs, max_ord)
}

fn enhance(grid: &mut Grid, decoder: &[u8], default_pixel: u8) -> u8 {
    let mut new_grid: Grid = vec![vec![PIXEL_UNKNOWN; 1000]; 1000];
    let extense = range(grid);

    for (y, line) in new_grid
        .iter_mut()
        .enumerate()
        .take(extense.3 + 2)
        .skip(extense.1 - 1)
    {
        for (x, val) in line
            .iter_mut()
            .enumerate()
            .take(extense.2 + 2)
            .skip(extense.0 - 1)
        {
            let mut sum: usize = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let nx = isize::try_from(x).unwrap() + dx;
                    let ny = isize::try_from(y).unwrap() + dy;
                    let nx = usize::try_from(nx).unwrap();
                    let ny = usize::try_from(ny).unwrap();
                    let mut pixel = grid[ny][nx];
                    if pixel == PIXEL_UNKNOWN {
                        pixel = default_pixel;
                    }
                    match pixel {
                        PIXEL_OFF => sum *= 2,         // pixel off
                        PIXEL_ON => sum = sum * 2 + 1, // pixel on
                        _ => panic!("unknown pixel"),
                    }
                }
            }
            *val = decoder[sum];
        }
    }
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            grid[y][x] = new_grid[y][x];
        }
    }

    match default_pixel {
        PIXEL_OFF => decoder[0],            // pixel off decoded
        PIXEL_ON => decoder[0b1_1111_1111], // pixel on decoded
        _ => panic!("unknown pixel"),
    }
}
