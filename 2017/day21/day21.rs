//! [Day 21: Fractal Art](https://adventofcode.com/2017/day/21)

use rustc_hash::FxHashMap;

type Square = aoc::Square<u8>;

struct Puzzle {
    rules: FxHashMap<Square, Square>,
    start: Square,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        let mut p = Self {
            rules: FxHashMap::default(),
            start: Square::parse(".#./..#/###", '/'),
        };

        for line in data.lines() {
            let (src, dest) = line.split_once(" => ").unwrap();

            let src = Square::parse(src, '/');
            let dest = Square::parse(dest, '/');

            for s in src.iter_pos() {
                p.rules.insert(s, dest.clone());
            }
        }
        p
    }

    fn enhance(&self, grid: &Square) -> Square {
        let n = grid.size();

        let m = if n % 2 == 0 {
            2
        } else if n % 3 == 0 {
            3
        } else {
            panic!();
        };

        let new_n = (n / m) * (m + 1);
        let mut enhanced_grid = Square::new(new_n);

        for y in 0..(n / m) {
            for x in 0..(n / m) {
                let square = grid.get_square(m * x, m * y, m);
                let enhancement = &self.rules[&square];

                enhanced_grid.put_square(x * (m + 1), y * (m + 1), enhancement);
            }
        }

        enhanced_grid
    }

    fn solve(&self, iterations: usize) -> usize {
        let mut grid = self.start.clone();

        for _ in 0..iterations {
            grid = self.enhance(&grid);
        }

        bytecount::count(grid.values(), b'#')
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.solve(5)
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        self.solve(18)
    }
}

fn main() {
    let args = aoc::parse_args();
    let puzzle = Puzzle::new(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let data = aoc::load_input_data("test.txt");
        let puzzle = Puzzle::new(&data);
        assert_eq!(puzzle.solve(2), 12);
    }
}
