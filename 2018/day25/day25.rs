//! [Day 25: Four-Dimensional Adventure](https://adventofcode.com/2018/day/25)

use rustworkx_core::connectivity::number_connected_components;
use rustworkx_core::petgraph::graph::UnGraph;

type Point = [i32; 4];

/// Manhattan distance between two points.
fn dist(a: &Point, b: &Point) -> u32 {
    a.iter()
        .zip(b.iter())
        .map(|(x1, x2)| x1.abs_diff(*x2))
        .sum()
}

struct Puzzle {
    constellation: Vec<Point>,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            constellation: vec![],
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            let p: Point = line
                .trim()
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .unwrap();

            self.constellation.push(p);
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let mut edges = vec![];

        for (ia, a) in self.constellation.iter().enumerate() {
            for (ib, b) in self.constellation.iter().enumerate() {
                if dist(a, b) <= 3 {
                    edges.push((u32::try_from(ia).unwrap(), u32::try_from(ib).unwrap()));
                }
            }
        }

        let g = UnGraph::<i32, ()>::from_edges(&edges);
        number_connected_components(&g)
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_1.txt");
        assert_eq!(puzzle.part1(), 2);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_2.txt");
        assert_eq!(puzzle.part1(), 4);
    }

    #[test]
    fn test03() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_3.txt");
        assert_eq!(puzzle.part1(), 3);
    }

    #[test]
    fn test04() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_4.txt");
        assert_eq!(puzzle.part1(), 8);
    }
}
