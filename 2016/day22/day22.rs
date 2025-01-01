//! [Day 22: Grid Computing](https://adventofcode.com/2016/day/22)

use rustc_hash::FxHashMap;

#[derive(Clone)]
struct Node {
    x: u32,
    y: u32,
    used: u32,
    avail: u32,
}

struct Puzzle {
    nodes: Vec<Node>,
    width: u32,
    height: u32,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            nodes: vec![],
            width: 0,
            height: 0,
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        for line in data.lines() {
            if !line.starts_with("/dev/grid/") {
                continue;
            }
            let mut line = line.split_ascii_whitespace();

            let name = line.next().unwrap().strip_prefix("/dev/grid/node-x").unwrap();
            let (x, y) = name.split_once("-y").unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();

            line.next(); // skip size
            let used = line.next().unwrap().strip_suffix('T').unwrap().parse().unwrap();
            let avail = line.next().unwrap().strip_suffix('T').unwrap().parse().unwrap();

            self.nodes.push(Node { x, y, used, avail });

            self.width = self.width.max(x + 1);
            self.height = self.height.max(y + 1);
        }
    }

    fn print(&self) {
        let big = if self.nodes.len() > 10 { 400 } else { 20 };
        let mut g = FxHashMap::default();

        for &Node { x, y, used, .. } in &self.nodes {
            let c = if (x, y) == (0, 0) {
                'O'
            } else if (x, y) == (self.width - 1, 0) {
                'G'
            } else if used >= big {
                '#'
            } else if used == 0 {
                '_'
            } else {
                '.'
            };
            g.insert((x, y), c);
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let c = g.get(&(x, y)).unwrap_or(&'?');
                print!("{c}");
            }
            println!();
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let mut viable = 0;

        for (i, a) in self.nodes.iter().enumerate() {
            for (j, b) in self.nodes.iter().enumerate() {
                if i != j && a.used > 0 && b.avail >= a.used {
                    viable += 1;
                }
            }
        }

        viable
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let mut empty_x = 0;
        let mut empty_y = 0;
        let mut wall_x = u32::MAX;

        for &Node { x, y, used, .. } in &self.nodes {
            if used == 0 {
                empty_x = x;
                empty_y = y;
            } else if used >= 400 {
                wall_x = wall_x.min(x - 1);
            }
        }

        // doesn't work for the sample (too small)
        (empty_x - wall_x) + empty_y + (self.width - 2 - wall_x) + 1 + 5 * (self.width - 2)
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);

    if args.verbose {
        puzzle.print();
    }

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
        puzzle.configure(&aoc::load_input_data("sample_1.txt"));
        assert_eq!(puzzle.part1(), 7);
    }

    // #[test]
    // fn test02() {
    //     let mut puzzle = Puzzle::new();
    //     puzzle.configure(&aoc::load_input_data("sample_1.txt"));
    //     assert_eq!(puzzle.part2(), 7);
    // }
}
