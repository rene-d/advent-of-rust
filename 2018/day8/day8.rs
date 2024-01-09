//! [Day 8: Memory Maneuver](https://adventofcode.com/2018/day/8)

struct Puzzle {
    nodes: Vec<usize>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { nodes: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.nodes = data
            .trim()
            .split_ascii_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
    }

    fn solve(&self, mut pos: usize, sum_all: bool) -> (usize, usize) {
        let children = self.nodes[pos];
        let metadata = self.nodes[pos + 1];
        pos += 2;

        let value = if sum_all {
            // part 1: sum all metadata values
            let mut value = 0;

            for _ in 0..children {
                let n;
                (pos, n) = self.solve(pos, sum_all);
                value += n;
            }

            value + self.nodes[pos..(pos + metadata)].iter().sum::<usize>()
        } else {
            // part 2: find the value of the root node
            if children == 0 {
                // no child: sum of metadata
                self.nodes[pos..(pos + metadata)].iter().sum()
            } else {
                // childrens: metadata are the indices of child values to sum
                let mut values = vec![];
                for _ in 0..children {
                    let m;
                    (pos, m) = self.solve(pos, sum_all);
                    values.push(m);
                }

                self.nodes[pos..(pos + metadata)]
                    .iter()
                    .filter(|&&n| 0 < n && n <= values.len())
                    .map(|n| values[n - 1])
                    .sum()
            }
        };

        (pos + metadata, value)
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.solve(0, true).1
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        self.solve(0, false).1
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
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
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 138);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 66);
    }
}
