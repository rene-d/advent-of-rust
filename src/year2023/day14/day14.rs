//! [Day 14: Parabolic Reflector Dish](https://adventofcode.com/2023/day/14)

use rustc_hash::FxHashMap;

/// Parabolic Reflector Dish.
struct Dish {
    grid: Vec<Vec<char>>,
    sx: usize,
    sy: usize,
}

impl Dish {
    fn new(data: &str) -> Self {
        let mut dish = Self {
            grid: vec![],
            sx: 0,
            sy: 0,
        };

        for line in data.lines() {
            let mut row = vec![];

            for c in line.chars() {
                row.push(c);
            }
            dish.grid.push(row);
        }

        dish.sx = dish.grid[0].len();
        dish.sy = dish.grid.len();

        dish
    }

    fn get(&self, x: usize, y: usize) -> char {
        if x < self.sx && y < self.sy {
            *self.grid[y].get(x).unwrap()
        } else {
            '@'
        }
    }

    /// Tilt the dish northwards.
    fn north(&mut self) {
        for x in 0..self.sx {
            for y in 0..self.sy {
                if self.grid[y][x] == '.' {
                    for y2 in y..self.sy {
                        match self.get(x, y2) {
                            'O' => {
                                self.grid[y][x] = 'O';
                                self.grid[y2][x] = '.';
                                break;
                            }
                            '#' => break,
                            _ => (),
                        }
                    }
                }
            }
        }
    }

    /// Tilt the dish southwards.
    fn south(&mut self) {
        for x in 0..self.sx {
            for y in (0..self.sy).rev() {
                if self.grid[y][x] == '.' {
                    for y2 in (0..y).rev() {
                        match self.get(x, y2) {
                            'O' => {
                                self.grid[y][x] = 'O';
                                self.grid[y2][x] = '.';
                                break;
                            }
                            '#' => break,
                            _ => (),
                        }
                    }
                }
            }
        }
    }

    /// Tilt the dish westwards.
    fn west(&mut self) {
        for y in 0..self.sy {
            for x in 0..(self.sx) {
                if self.grid[y][x] == '.' {
                    for x2 in (x)..self.sx {
                        match self.get(x2, y) {
                            'O' => {
                                self.grid[y][x] = 'O';
                                self.grid[y][x2] = '.';
                                break;
                            }
                            '#' => break,
                            _ => (),
                        }
                    }
                }
            }
        }
    }

    /// Tilt the dish eastwards.
    fn east(&mut self) {
        for y in 0..self.sy {
            for x in (0..(self.sx)).rev() {
                if self.grid[y][x] == '.' {
                    for x2 in (0..x).rev() {
                        match self.get(x2, y) {
                            'O' => {
                                self.grid[y][x] = 'O';
                                self.grid[y][x2] = '.';
                                break;
                            }
                            '#' => break,
                            _ => (),
                        }
                    }
                }
            }
        }
    }

    /// Compute the load.
    fn load(&self) -> usize {
        let mut result = 0;
        for y in (0..self.sy).rev() {
            let mut n = 0;
            for x in 0..self.sx {
                if self.grid[y][x] == 'O' {
                    n += 1;
                }
            }
            result += n * (self.sy - y);
        }
        result
    }

    /// Return a hashable value that represents the actual state of the dish.
    fn state(&self) -> Vec<Vec<char>> {
        self.grid.clone()
    }
}

impl std::fmt::Display for Dish {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.grid {
            for c in row {
                match c {
                    'O' => f.write_str("\x1b[95mo\x1b[0m")?,
                    _ => write!(f, "{c}")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(feature = "anim")]
impl Dish {
    fn export_frame(&self, frame: u32) {
        const SCALE: u32 = 2;

        let mut imgbuf = image::ImageBuffer::new(self.sx as u32 * SCALE, self.sy as u32 * SCALE);

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let g = (0.4 * y as f32) as u8;
            let b = (0.4 * x as f32) as u8;
            *pixel = image::Rgb([0, g, b]);
        }

        for (y, row) in (0..).zip(self.grid.iter()) {
            for (x, c) in (0..).zip(row.iter()) {
                let color = match c {
                    'O' => image::Rgb([240, 200, 30]),
                    '#' => image::Rgb([40, 80, 250]),
                    '.' => continue,
                    _ => panic!("unexpected char {c}"),
                };

                for k in 0..(SCALE * SCALE) {
                    let pixel = imgbuf.get_pixel_mut(x * SCALE + k % SCALE, y * SCALE + k / SCALE);
                    *pixel = color;
                }
            }
        }

        imgbuf.save(format!("frame_{frame:03}.png")).unwrap();
    }

    /// Export frames to make an animation of the platform's tilt.
    fn anim(data: &str) -> std::io::Result<()> {
        let mut dish = Self::new(data);
        let mut seen = rustc_hash::FxHashSet::default();

        let mut frame = 0;

        let mut show = |dish: &Dish| {
            dish.export_frame(frame);
            frame += 1;
        };

        while seen.insert(dish.state()) {
            dish.north();
            show(&dish);

            dish.west();
            show(&dish);

            dish.south();
            show(&dish);

            dish.east();
            show(&dish);
        }

        use std::io::Write;

        let mut magick = std::process::Command::new("magick")
            .args(["-script", "-"])
            .stdin(std::process::Stdio::piped())
            // .stdout(std::process::Stdio::piped())
            .spawn()?;

        let child_stdin = magick.stdin.as_mut().unwrap();
        child_stdin.write(b"-delay 20 -loop 0\n")?;
        for i in (0..frame).step_by(9) {
            child_stdin.write_fmt(format_args!("frame_{i:03}.png\n"))?;
        }
        child_stdin.write(b"-write anim.gif\n")?;

        magick.wait_with_output()?;

        for i in 0..frame {
            let _ = std::fs::remove_file(format!("frame_{i:03}.png"));
        }

        Ok(())
    }
}

struct Puzzle<'a> {
    data: &'a str,
}

impl<'a> Puzzle<'a> {
    const fn new(data: &'a str) -> Self {
        Self { data }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let mut dish = Dish::new(self.data);
        dish.north();
        dish.load()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let mut dish = Dish::new(self.data);

        let cycles = 1_000_000_000;
        let mut seen = FxHashMap::default();

        for mut i in 1..=cycles {
            dish.north();
            dish.west();
            dish.south();
            dish.east();

            let key = dish.state();

            if seen.contains_key(&key) {
                // same configuration detected: we have a cycle
                let cycle_length = i - seen.get(&key).unwrap();

                // skip as many cycles as possible
                i += ((cycles - i) / cycle_length) * cycle_length;

                // then continue to reach the wanted cycle number
                while i < cycles {
                    i += 1;
                    dish.north();
                    dish.west();
                    dish.south();
                    dish.east();
                }

                // eprintln!("{dish}");

                // we've done
                return dish.load();
            }

            seen.insert(key, i);
        }

        0
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, usize) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();

    #[cfg(feature = "anim")]
    let _ = Dish::anim(&args.input());

    #[cfg(not(feature = "anim"))]
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 136);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 64);
    }
}
