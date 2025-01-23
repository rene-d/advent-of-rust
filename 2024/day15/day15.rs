//! [Day 15: Warehouse Woes](https://adventofcode.com/2024/day/15)

use rustc_hash::FxHashSet;
use std::collections::VecDeque;

use aoc::Coord;

type Grid = aoc::Grid<char>;

fn score(grid: &Grid) -> i32 {
    grid.iter_cells()
        .filter(|(_, &c)| c == 'O' || c == '[')
        .map(|(xy, _)| 100 * xy.y + xy.x)
        .sum()
}

fn init_first_warehouse(input: &str) -> (Grid, Coord) {
    let mut grid = Grid::parse(input);
    let mut start = Coord { x: 0, y: 0 };

    for (pos, &c) in &grid {
        if c == '@' {
            start = pos;
            break;
        }
    }
    grid[start] = '.';

    (grid, start)
}

fn init_second_warehouse(input: &str) -> (Grid, Coord) {
    let simple = Grid::parse(input);
    let mut grid = Grid::with_size(simple.width() * 2, simple.height(), ' ', '#');
    let mut start = Coord::new(0, 0);

    for (Coord { x, y }, &c) in simple.iter_cells() {
        let pos1 = Coord { x: x * 2, y };
        let pos2 = Coord { x: x * 2 + 1, y };
        match c {
            '@' => {
                start = pos1;
                grid[pos1] = '.';
                grid[pos2] = '.';
            }
            'O' => {
                grid[pos1] = '[';
                grid[pos2] = ']';
            }
            _ => {
                grid[pos1] = c;
                grid[pos2] = c;
            }
        };
    }

    (grid, start)
}

fn move_boxes(grid: &mut Grid, robot: &mut Coord, d: Coord) {
    let mut seen = FxHashSet::default();

    let mut queue = VecDeque::new();
    queue.push_back(*robot);
    while let Some(pos) = queue.pop_front() {
        if seen.contains(&pos) {
            continue;
        }
        seen.insert(pos);

        let new_pos = pos + d;

        match grid[new_pos] {
            '#' => {
                return;
            }
            '[' => {
                queue.push_back(new_pos);
                queue.push_back(new_pos + Coord::RIGHT);
            }
            ']' => {
                queue.push_back(new_pos + Coord::LEFT);
                queue.push_back(new_pos);
            }
            '.' => (),
            _ => panic!(),
        };
    }

    while !seen.is_empty() {
        let mut seen_new = FxHashSet::default();

        for &pos in &seen {
            let new_pos = pos + d;

            if seen.contains(&new_pos) {
                seen_new.insert(pos);
            } else {
                grid[new_pos] = grid[pos];
                grid[pos] = '.';
            }
        }

        seen = seen_new;
    }

    *robot += d;
}

#[cfg(feature = "anim")]
fn save_warehouse(
    grid: &Grid,
    robot: Coord,
    n: u32,
    moves: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    const SCALE: u32 = 11;

    // limit the frame count
    if moves > 1000 && n % 20 != 0 {
        return Ok(());
    }
    if moves > 100 && n % 5 != 0 {
        return Ok(());
    }

    let width = u32::try_from(grid.width())? * SCALE;
    let height = u32::try_from(grid.height())? * SCALE;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = u8::try_from(((3 * x) / 10) % 256)?;
        let b = u8::try_from(((3 * y) / 10) % 256)?;
        *pixel = image::Rgb([r, 0, b]);
    }

    {
        let x = u32::try_from(robot.x)? * SCALE;
        let y = u32::try_from(robot.y)? * SCALE;

        for k in 0..((SCALE - 2) * (SCALE - 2)) {
            let pixel = imgbuf.get_pixel_mut(x + 1 + k % (SCALE - 2), y + 1 + k / (SCALE - 2));
            *pixel = image::Rgb([5, 255, 5]);
        }
    }

    for (pos, &c) in grid.iter_cells() {
        let x = u32::try_from(pos.x)? * SCALE;
        let y = u32::try_from(pos.y)? * SCALE;

        if c == '#' {
            for k in 0..(SCALE * SCALE) {
                let pixel = imgbuf.get_pixel_mut(x + k % SCALE, y + k / SCALE);
                *pixel = image::Rgb([240, 10, 70]);
            }
        }

        if c == 'O' {
            for k in 0..((SCALE - 1) * (SCALE - 2)) {
                let pixel = imgbuf.get_pixel_mut(x + 1 + k % (SCALE - 1), y + 1 + k / (SCALE - 2));
                *pixel = image::Rgb([0x9C, 0xDC, 0xFE]);
            }
        }

        if c == '[' {
            for k in 0..((SCALE - 1) * (SCALE - 2)) {
                let pixel = imgbuf.get_pixel_mut(x + 1 + k % (SCALE - 1), y + 1 + k / (SCALE - 2));
                *pixel = image::Rgb([0x9C, 0xDC, 0xFE]);
            }
        }

        if c == ']' {
            for k in 0..((SCALE - 1) * (SCALE - 2)) {
                let pixel = imgbuf.get_pixel_mut(x + k % (SCALE - 1), y + 1 + k / (SCALE - 2));
                *pixel = image::Rgb([0x9C, 0xDC, 0xFE]);
            }
        }
    }

    imgbuf.save(format!("frame{n:05}.png"))?;

    Ok(())
}

struct Puzzle {
    data: String,
    moves: Vec<char>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let (a, b) = data.split_once("\n\n").unwrap();

        Self {
            data: a.to_string(),
            moves: b.chars().collect(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        let (mut grid, mut robot) = init_first_warehouse(&self.data);

        #[cfg(feature = "anim")]
        let mut n = 1;

        #[cfg(feature = "anim")]
        let _ = save_warehouse(&grid, robot, 0, 0);

        for m in &self.moves {
            let d = match m {
                '>' => Coord::RIGHT,
                '<' => Coord::LEFT,
                'v' => Coord::DOWN,
                '^' => Coord::UP,
                _ => continue,
            };

            match grid[robot + d] {
                '.' => {
                    robot += d;
                }
                'O' => {
                    let mut i = 1;
                    while grid[robot + i * d] == 'O' {
                        i += 1;
                    }
                    if grid[robot + i * d] == '.' {
                        grid[robot + i * d] = 'O';
                        grid[robot + d] = '.';

                        robot += d;
                    }
                }
                '#' => (),
                _ => panic!(),
            };

            #[cfg(feature = "anim")]
            {
                let _ = save_warehouse(&grid, robot, n, self.moves.len());
                n += 1;
            }
        }

        score(&grid)
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        let (mut grid, mut robot) = init_second_warehouse(&self.data);

        #[cfg(feature = "anim")]
        let mut n = 1;

        #[cfg(feature = "anim")]
        let _ = save_warehouse(&grid, robot, 0, 0);

        for m in &self.moves {
            let d = match m {
                '>' => Coord::RIGHT,
                '<' => Coord::LEFT,
                'v' => Coord::DOWN,
                '^' => Coord::UP,
                _ => continue,
            };

            match grid[robot + d] {
                '.' => {
                    robot += d;
                }
                'O' | '[' | ']' => {
                    move_boxes(&mut grid, &mut robot, d);
                }
                _ => {}
            }

            #[cfg(feature = "anim")]
            {
                let _ = save_warehouse(&grid, robot, n, self.moves.len());
                n += 1;
            }
        }

        score(&grid)
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (i32, i32) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();

    #[cfg(feature = "anim")]
    if args.has_option("--anim1") {
        let puzzle = Puzzle::new(&args.input);
        puzzle.part1();
        std::process::exit(0);
    }

    #[cfg(feature = "anim")]
    if args.has_option("--anim2") {
        let puzzle = Puzzle::new(&args.input);
        puzzle.part2();
        std::process::exit(0);
    }

    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_1: &str = include_str!("sample_1.txt");
    const SAMPLE_2: &str = include_str!("sample_2.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part1(), 10092);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(SAMPLE_2);
        assert_eq!(puzzle.part1(), 2028);
    }

    #[test]
    fn test03() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part2(), 9021);
    }
}
