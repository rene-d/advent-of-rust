//! [Day 6: Chronal Coordinates](https://adventofcode.com/2018/day/6)

#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]

const N: usize = 512;

struct Puzzle {
    points: Vec<(usize, usize)>,
}

impl Puzzle {
    const fn new() -> Self {
        Self { points: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        for line in data.lines() {
            if let Some((x, y)) = line.split_once(", ") {
                let x = x.parse().unwrap();
                let y = y.parse().unwrap();

                self.points.push((x, y));
            }
        }
    }

    // fn show(&self) {
    //     let mut pts: HashSet<(usize, usize)> = HashSet::new();
    //     pts.extend(self.points.iter());
    //     for y in 0..N {
    //         for x in 0..N {
    //             let c = grid[y][x];
    //             if c == 255 {
    //                 print!(".");
    //             } else {
    //                 if pts.contains(&(x, y)) {
    //                     //print!("\x1B[1;37m{}\x1B[0m", (c%26 + 65) as char);
    //                     print!("{}", (c % 26 + 65) as char);
    //                 } else {
    //                     print!("{}", (c % 26 + 97) as char);
    //                 }
    //             }
    //         }
    //         println!();
    //     }
    // }

    /// Solve part one.
    #[allow(clippy::needless_range_loop)] // much comprehensive (according to me...)
    fn part1(&self) -> u32 {
        let mut grid = [[0u8; N]; N];

        // find the areas of coordinates
        for y in 0..N {
            for x in 0..N {
                // find the nearest points
                let mut manhattan = vec![];

                for (i, &(px, py)) in self.points.iter().enumerate() {
                    let d = px.abs_diff(x) + py.abs_diff(y);
                    manhattan.push((d, i as u8));
                }

                manhattan.sort_unstable();

                if manhattan[0].0 == manhattan[1].0 {
                    grid[y][x] = 255;
                } else {
                    grid[y][x] = manhattan[0].1;
                }
            }
        }

        // start the count at 1 to use 0 as value for infinite
        let mut counts = [1; 50];

        for y in 0..N {
            for x in 0..N {
                let c = grid[y][x];

                if c == 255 {
                    // coordinates at equal distance of two or more points
                    // do not count
                    continue;
                }

                if x == 0 || y == 0 || x == N - 1 || y == N - 1 {
                    // points on the edge of the map are considered as infinite areas
                    counts[c as usize] = 0;
                } else if c != 255 && counts[c as usize] != 0 {
                    counts[c as usize] += 1;
                }
            }
        }

        // sort by finite area size
        counts.sort_unstable();
        counts.last().unwrap() - 1 // remove 1 as we artificially added it
    }

    /// Solve part two.
    fn part2(&self, limit: i32) -> u32 {
        let mut result = 0;

        for y in -limit..(limit + 400) {
            for x in -limit..(limit + 400) {
                let mut d = 0;

                for &(px, py) in &self.points {
                    d += x.abs_diff(px as i32) + y.abs_diff(py as i32);
                    if d >= limit as u32 {
                        break;
                    }
                }

                if d < (limit as u32) {
                    result += 1;
                }
            }
        }
        result
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2(10_000));
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part1(), 17);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part2(32), 16);
    }
}
