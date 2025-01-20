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
    fn new(data: &str) -> Self {
        Self {
            constellation: data
                .lines()
                .map(|line| {
                    let p: Point = line
                        .trim()
                        .split(',')
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                        .as_slice()
                        .try_into()
                        .unwrap();
                    p
                })
                .collect(),
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
    let mut args = aoc::parse_args();
    args.run(|data| {
        let puzzle = Puzzle::new(data);
        (puzzle.part1(), aoc::CHRISTMAS)
    });
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_1: &str = include_str!("sample_1.txt");
    const SAMPLE_2: &str = include_str!("sample_2.txt");
    const SAMPLE_3: &str = include_str!("sample_3.txt");
    const SAMPLE_4: &str = include_str!("sample_4.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part1(), 2);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(SAMPLE_2);
        assert_eq!(puzzle.part1(), 4);
    }

    #[test]
    fn test03() {
        let puzzle = Puzzle::new(SAMPLE_3);
        assert_eq!(puzzle.part1(), 3);
    }

    #[test]
    fn test04() {
        let puzzle = Puzzle::new(SAMPLE_4);
        assert_eq!(puzzle.part1(), 8);
    }
}
