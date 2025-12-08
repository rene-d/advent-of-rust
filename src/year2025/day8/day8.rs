//! [Day 8: Playground](https://adventofcode.com/2025/day/8)

use rustc_hash::FxHashMap;

struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    const fn square_dist(&self, rhs: &Self) -> i64 {
        (rhs.x - self.x).pow(2) + (rhs.y - self.y).pow(2) + (rhs.z - self.z).pow(2)
    }
}

struct Puzzle {
    points: Vec<Point3D>,
    edges: Vec<(i64, usize, usize)>,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        let mut points = vec![];
        let mut edges = vec![];

        for line in data.lines() {
            let xyz: Vec<_> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();

            points.push(Point3D {
                x: xyz[0],
                y: xyz[1],
                z: xyz[2],
            });
        }

        for i in 0..(points.len() - 1) {
            for j in (i + 1)..points.len() {
                edges.push((points[i].square_dist(&points[j]), i, j));
            }
        }
        edges.sort_unstable_by_key(|e| e.0);

        Self { points, edges }
    }

    /// Solve part one.
    fn part1(&self, connections: usize) -> i32 {
        let mut dsu = aoc::UnionFind::new(self.points.len());

        for i in 0..connections.min(self.edges.len()) {
            let e = self.edges[i];
            dsu.unite(e.1, e.2);
        }

        let mut comps = FxHashMap::default();
        for i in 0..self.points.len() {
            let r = dsu.find(i);
            *comps.entry(r).or_insert(0) += 1;
        }

        let mut sizes: Vec<_> = comps.values().copied().collect();
        sizes.sort_unstable_by(|a, b| b.cmp(a));
        sizes.iter().take(3).product()
    }

    /// Solve part two.
    /// [Kruskal's algorithm](https://en.wikipedia.org/wiki/Kruskal%27s_algorithm)
    fn part2(&self) -> i64 {
        let mut dsu = aoc::UnionFind::new(self.points.len());

        for &(_, i, j) in &self.edges {
            if dsu.unite(i, j) {
                // If this union makes everything connected, it's the answer
                if dsu.component_count() == 1 {
                    return self.points[i].x * self.points[j].x;
                }
            }
        }
        0
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (i32, i64) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(1000), puzzle.part2())
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
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(10), 40);
    }

    #[test]
    fn part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 25272);
    }
}
