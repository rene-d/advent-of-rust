//! [Day 6: Chronal Coordinates](https://adventofcode.com/2018/day/6)

const N: usize = 512;

struct Puzzle {
    points: Vec<(i32, i32)>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut points = vec![];
        for line in data.lines() {
            if let Some((x, y)) = line.split_once(", ") {
                let x = x.parse().unwrap();
                let y = y.parse().unwrap();

                points.push((x, y));
            }
        }
        Self { points }
    }

    // fn show(&self) {
    //     let mut pts: FxHashSet<(usize, usize)> = FxHashSet::default();
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
        let mut grid = vec![[0u8; N]; N].into_boxed_slice();

        // find the areas of coordinates
        for y in 0..N {
            for x in 0..N {
                // find the nearest points
                let mut manhattan = vec![];

                for (i, &(px, py)) in (0..).zip(self.points.iter()) {
                    let x = i32::try_from(x).unwrap();
                    let y = i32::try_from(y).unwrap();

                    let d = px.abs_diff(x) + py.abs_diff(y);
                    manhattan.push((d, i));
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
                    d += (x - px).abs() + (y - py).abs();
                    if d >= limit {
                        break;
                    }
                }

                if d < limit {
                    result += 1;
                }
            }
        }
        result
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2(10_000))
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 17);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(32), 16);
    }
}
