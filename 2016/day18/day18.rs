//! [Day 18: Like a Rogue](https://adventofcode.com/2016/day/18)

struct Puzzle {
    tiles: Vec<u32>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            tiles: data.trim_ascii().chars().map(|c| u32::from(c == '.')).collect(),
        }
    }

    fn guess(tiles: &mut [u32]) {
        let prev = tiles.to_vec();

        for i in 0..prev.len() {
            let left = if i == 0 { 1 } else { prev[i - 1] };
            let center = prev[i];
            let right = if i == prev.len() - 1 { 1 } else { prev[i + 1] };

            tiles[i] = match (left, center, right) {
                // (0, 0, 1) | (1, 0, 0) | (0, 1, 1) | (1, 1, 0) => 0,
                (0, 0 | 1, 1) | (1, 0 | 1, 0) => 0,
                _ => 1,
            };
        }
    }

    fn solve(&self, rows: usize) -> u32 {
        let mut result = self.tiles.iter().sum::<u32>();

        let mut tiles = self.tiles.clone();

        for _ in 1..rows {
            Self::guess(&mut tiles);
            result += tiles.iter().sum::<u32>();
        }
        result
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        self.solve(40)
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        self.solve(400_000)
    }
}

fn main() {
    let mut args = aoc::parse_args();
    args.run(|data| {
        let puzzle = Puzzle::new(data);
        (puzzle.part1(), puzzle.part2())
    });
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let puzzle = Puzzle::new("..^^.");
        assert_eq!(puzzle.solve(3), 6);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(".^^.^.^^^^");
        assert_eq!(puzzle.solve(10), 38);
    }
}
