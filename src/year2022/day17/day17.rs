//! [Day 17: Pyroclastic Flow](https://adventofcode.com/2022/day/17)

use rustc_hash::FxHashMap;

// Rocks structure with variable sizes
const ROCKS: &[&[&[u8]]] = &[
    &[&[1, 1, 1, 1]],
    &[&[0, 1, 0], &[1, 1, 1], &[0, 1, 0]],
    &[&[1, 1, 1], &[0, 0, 1], &[0, 0, 1]],
    &[&[1], &[1], &[1], &[1]],
    &[&[1, 1], &[1, 1]],
];

struct Cave {
    rows: Vec<[u8; 7]>,
    bottom: usize,
    jet_count: usize,
    rock_count: usize,
}

impl Cave {
    fn new() -> Self {
        Self {
            rows: vec![[1, 1, 1, 1, 1, 1, 1]],
            bottom: 0,
            jet_count: 0,
            rock_count: 0,
        }
    }
    // Function to calculate the current height of the cave
    fn height(&self) -> usize {
        self.rows.len() + self.bottom
    }

    // Function to check if a rock overlaps with existing rocks in the cave
    fn overlap(&self, x: usize, y: usize, rock: &[&[u8]]) -> bool {
        let rock_width = rock[0].len();

        for (i, rock_row) in rock.iter().enumerate() {
            for j in 0..rock_width {
                if y + i < self.height()
                    && self.rows[y + i - self.bottom][x + j] != 0
                    && rock_row[j] == 1
                {
                    return true;
                }
            }
        }
        false
    }

    /// Fall of a single rock
    fn fall(&mut self, jets: &[u8]) {
        let rock = ROCKS[self.rock_count % ROCKS.len()];
        self.rock_count += 1;

        let rock_width = rock[0].len();

        let mut y = self.height() + 3;
        let mut x = 2;

        loop {
            // current jet of gas
            let gas = jets[self.jet_count % jets.len()];
            self.jet_count += 1;

            // shift the rock left or right if possible
            if gas == b'>' && x + rock_width < 7 && !self.overlap(x + 1, y, rock) {
                x += 1;
            } else if gas == b'<' && x > 0 && !self.overlap(x - 1, y, rock) {
                x -= 1;
            }

            // make the rock fall if possible
            if self.overlap(x, y - 1, rock) {
                break;
            }
            y -= 1;
        }

        // Place the rock in the cave
        for (i, rock_row) in rock.iter().enumerate() {
            if y + i >= self.height() {
                self.rows.push([0; 7]);
            }
            for j in 0..rock_width {
                if rock_row[j] == 1 {
                    self.rows[y + i - self.bottom][x + j] = 1;
                }
            }
        }

        // If the cave grows too much, remove older rows and adjust the bottom
        if self.rows.len() > 200 {
            self.rows.drain(0..100);
            self.bottom += 100;
        }
    }

    fn make_key(&self, jets_len: usize) -> (usize, usize, Vec<u8>) {
        let mut top = vec![];

        for y in (0..self.rows.len()).rev() {
            let mut mask = 0;
            for (i, &c) in self.rows[y].iter().enumerate() {
                if c != 0 {
                    mask |= 1 << i;
                }
            }
            top.push(mask);
            if mask == 127 {
                break;
            }
        }

        (
            self.rock_count % ROCKS.len(),
            self.jet_count % jets_len,
            top,
        )
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, u64) {
    let jets: Vec<u8> = data.trim().bytes().collect();

    let mut cave = Cave::new();

    // Part 1 and Part 2
    let mut part1 = 0;
    let mut part2 = 0;

    let mut heights = vec![0];
    let mut keys = FxHashMap::default();
    let mut start = 0;
    let mut end = 0;

    for n in 1..10000 {
        cave.fall(&jets);

        if n == 2022 {
            part1 = cave.height() - 1;
        }

        if end == 0 {
            heights.push(cave.height() - 1);
            let key = cave.make_key(jets.len());
            if n > 2000 {
                if let Some(&prev) = keys.get(&key) {
                    start = prev;
                    end = n;
                }
            }
            keys.insert(key, n);
        }

        if part1 != 0 && end != 0 {
            break;
        }
    }

    if start != 0 && end != 0 {
        let remaining_rocks = 1_000_000_000_000 - u64::try_from(start).unwrap();
        let cycle_length = u64::try_from(end - start).unwrap();

        let q = remaining_rocks / cycle_length;
        let r = usize::try_from(remaining_rocks % cycle_length).unwrap();

        part2 = u64::try_from(heights[start + r]).unwrap() + q * u64::try_from(heights[end] - heights[start]).unwrap();
    }

    (part1, part2)
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
    fn part1() {
        let puzzle = solve(TEST_INPUT);
        assert_eq!(puzzle.0, 3068);
    }

    #[test]
    fn part2() {
        let puzzle = solve(TEST_INPUT);
        assert_eq!(puzzle.1, 1514285714288);
    }
}
