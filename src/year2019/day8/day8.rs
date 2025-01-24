//! [Day 8: Space Image Format](https://adventofcode.com/2019/day/8)

struct Puzzle {
    data: Vec<u8>,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        Self {
            data: data.trim_ascii().bytes().collect(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        let mut layers = Vec::new();

        for layer in self.data.chunks(25 * 6) {
            let mut zeros = 0;
            let mut ones = 0;
            let mut twos = 0;

            for c in layer {
                match c {
                    b'0' => zeros += 1,
                    b'1' => ones += 1,
                    b'2' => twos += 1,
                    _ => {}
                }
            }

            layers.push((zeros, ones * twos));
        }

        layers.sort_unstable();

        layers[0].1
    }

    fn make_image(&self, w: usize, h: usize) -> String {
        let mut image = aoc::GridU::<char>::with_size(w, h);

        for ((x, y), c) in image.iter_mut() {
            for i in 0..(self.data.len() / (w * h)) {
                let pixel = self.data[(x + y * w) + i * w * h];
                match pixel {
                    b'0' => {
                        *c = '.';
                        break;
                    }
                    b'1' => {
                        *c = '#';
                        break;
                    }
                    _ => {}
                }
            }
        }

        format!("{image}")
    }

    /// Solve part two.
    fn part2(&self) -> String {
        aoc::ocr::scan_5x6(&self.make_image(25, 6))
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (i32, String) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2() {
        let puzzle = Puzzle::new("0222112222120000");
        assert_eq!(puzzle.make_image(2, 2), ".#\n#.\n");
    }
}
