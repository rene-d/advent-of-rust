//! [Day 14: Disk Defragmentation](https://adventofcode.com/2017/day/14)

use aoc::{knot, Coord, Grid};

const fn count_ones(value: u8) -> u32 {
    let mut count = 0;
    let mut value = value;

    while value != 0 {
        count += 1;
        value &= value - 1;
    }

    count
}

struct Puzzle {
    key: String,
}

impl Puzzle {
    const fn new() -> Self {
        Self { key: String::new() }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        self.key = data.trim().to_string();
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        (0..128)
            .map(|i| {
                knot::hash_raw(format!("{}-{i}", self.key).as_str())
                    .iter()
                    .copied()
                    .map(count_ones)
                    .sum::<u32>()
            })
            .sum()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let mut grid: Grid<u8> = Grid::<u8>::with_size(128, 128, 0, 0);

        for y in 0..128 {
            let row = knot::hash_raw(format!("{}-{y}", self.key).as_str());

            for (i, octet) in (0..).zip(row.iter()) {
                //
                for b in 0..8 {
                    let x = (i * 8) + b;
                    let o = (octet >> (7 - b)) & 1;
                    grid[Coord::new(x, y)] = o;
                }
            }
        }

        let mut queue = Vec::new();

        let mut result = 0;

        for y in 0..128 {
            for x in 0..128 {
                let xy = Coord::new(x, y);

                if grid[xy] == 0 {
                    continue;
                }

                result += 1;

                // bfs to find all adjacent used squares
                queue.push(xy);
                while let Some(xy) = queue.pop() {
                    grid[xy] = 0; // cancel the square so we don't need to maintain a 'visited' set

                    for (_, nxy) in grid.iter_directions(xy) {
                        if grid[nxy] == 1 {
                            queue.push(nxy);
                        }
                    }
                }
            }

            //
        }

        result
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part1(), 8108);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part2(), 1242);
    }
}
