//! [Day 6: Chronal Coordinates](https://adventofcode.com/2018/day/6)

use rayon::prelude::*;

const N: usize = 512;
const N_I32: i32 = 512;

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
    fn part1(&self) -> u32 {
        let mut grid = vec![[0u8; N]; N].into_boxed_slice();

        // find the areas of coordinates
        grid.par_iter_mut().zip(0..N_I32).for_each(|(row, y)| {
            for (cell, x) in row.iter_mut().zip(0i32..N_I32) {
                let mut min_dist = u32::MAX;
                let mut closest = 255;

                for (&(px, py), i) in self.points.iter().zip(0..) {
                    let d = px.abs_diff(x) + py.abs_diff(y);

                    match d.cmp(&min_dist) {
                        std::cmp::Ordering::Less => {
                            min_dist = d;
                            closest = i;
                        }
                        std::cmp::Ordering::Equal => {
                            closest = 255;
                        }
                        std::cmp::Ordering::Greater => {}
                    }
                }
                *cell = closest;
            }
        });

        // start the count at 1 to use 0 as value for infinite
        let mut counts = vec![1; self.points.len()];

        for y in 0..N {
            for x in 0..N {
                let c = grid[y][x];

                if c == 255 {
                    // coordinates at equal distance of two or more points
                    // do not count
                    continue;
                }

                let idx = c as usize;
                if x == 0 || y == 0 || x == N - 1 || y == N - 1 {
                    // points on the edge of the map are considered as infinite areas
                    counts[idx] = 0;
                } else if counts[idx] != 0 {
                    counts[idx] += 1;
                }
            }
        }

        // sort by finite area size
        counts.sort_unstable();
        counts.last().unwrap() - 1 // remove 1 as we artificially added it
    }

    /// Solve part two.
    fn part2(&self, limit: i32) -> usize {
        let (mut xs, mut ys): (Vec<i32>, Vec<i32>) = self.points.iter().copied().unzip();
        xs.sort_unstable();
        ys.sort_unstable();

        let safe_costs = |coords: &[i32]| -> Vec<i32> {
            let min_val = *coords.first().unwrap();
            let max_val = *coords.last().unwrap();
            let mut costs = Vec::new();

            // center
            for t in min_val..=max_val {
                let cost: i32 = coords.iter().map(|c| (t - c).abs()).sum();
                if cost < limit {
                    costs.push(cost);
                }
            }

            // left
            for t in (min_val - limit..min_val).rev() {
                let cost: i32 = coords.iter().map(|c| (t - c).abs()).sum();
                if cost >= limit {
                    break;
                }
                costs.push(cost);
            }

            // right
            for t in (max_val + 1)..(max_val + limit) {
                let cost: i32 = coords.iter().map(|c| (t - c).abs()).sum();
                if cost >= limit {
                    break;
                }
                costs.push(cost);
            }
            costs
        };

        let x_costs = safe_costs(&xs);
        let mut y_costs = safe_costs(&ys);
        y_costs.sort_unstable();

        let mut count = 0;
        for cx in x_costs {
            let rem = limit - cx;
            if rem > 0 {
                let idx = y_costs.partition_point(|&cy| cy < rem);
                count += idx;
            }
        }
        count
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, usize) {
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
