//! [Day 23: A Long Walk](https://adventofcode.com/2023/day/23)

use std::collections::{HashMap, HashSet, VecDeque};

use day23::grid::{Coord, Grid};

const FOREST: char = '#';

struct Puzzle {
    grid: Grid,
}

impl Puzzle {
    const fn new() -> Self {
        Self { grid: Grid::new() }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.grid = Grid::parse(&data);
    }

    /// Solve part one.
    ///
    /// Nota: should be rewritten to use the new introduced Grid class
    fn part1(&self) -> u32 {
        // start posiiton
        let sx = 1;
        let sy = 0;

        // target position
        let tx = self.grid.width() - 2;
        let ty = self.grid.height() - 1;

        // min steps hike
        let mut m = 0;

        let mut q = VecDeque::new();
        let mut seen = HashSet::new();

        q.push_back((0, sx, sy, 0, 0));
        while let Some((c, x, y, px, py)) = q.pop_front() {
            if seen.contains(&(c, x, y)) {
                continue;
            }
            seen.insert((c, x, y));

            if x == tx && y == ty {
                m = m.max(c);
            }

            // if x+1 is inside the grid,
            // and the move is authorized (path or slop),
            // and we're not going backwards,
            // then queue the move
            if x < self.grid.width() - 1 && px != x + 1 && ".>".contains(self.grid[(x + 1, y)]) {
                q.push_back((c + 1, x + 1, y, x, y));
            }

            if x > 0 && px != x - 1 && ".<".contains(self.grid[(x - 1, y)]) {
                q.push_back((c + 1, x - 1, y, x, y));
            }

            if y < self.grid.height() - 1 && py != y + 1 && ".v".contains(self.grid[(x, y + 1)]) {
                q.push_back((c + 1, x, y + 1, x, y));
            }

            if y > 0 && py != y - 1 && ".^".contains(self.grid[(x, y - 1)]) {
                q.push_back((c + 1, x, y - 1, x, y));
            }
        }

        m
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        // cf. https://stackoverflow.com/questions/16946888/is-it-possible-to-make-a-recursive-closure-in-rust
        struct Dfs<'s> {
            f: &'s dyn Fn(&Dfs, Coord, &mut HashSet<Coord>) -> i32,
        }

        let grid = &self.grid;
        let mut adj: HashMap<Coord, HashMap<Coord, i32>> = HashMap::new();

        for (pos, &c) in grid.iter_cells() {
            if c != FOREST {
                let e = adj.entry(pos).or_default();
                for neigh in grid.iter_directions(pos) {
                    if grid[neigh] != FOREST {
                        e.insert(neigh, 1);
                    }
                }
            }
        }

        while let Some((p, qs)) = adj.iter().find(|(_, qs)| qs.len() == 2) {
            let p = *p;

            let mut it = qs.iter();
            let &q1 = it.next().unwrap().0;
            let &q2 = it.next().unwrap().0;

            let n = adj[&q1][&p] + adj[&p][&q2];

            adj.entry(q1).or_default().insert(q2, n);
            adj.entry(q2).or_default().insert(q1, n);

            adj.remove(&p);
            adj.entry(q1).or_default().remove(&p);
            adj.entry(q2).or_default().remove(&p);
        }

        let start_pos = Coord::new(1, 0);
        let end_pos = Coord::new(grid.width() - 2, grid.width() - 1);

        let dfs = Dfs {
            f: &|dfs, p, visited| {
                if p == end_pos {
                    return 0;
                }

                visited.insert(p);
                let mut steps = i32::MIN;
                for (np, dist) in &adj[&p] {
                    if !visited.contains(np) {
                        steps = steps.max(dist + (dfs.f)(dfs, *np, visited));
                    }
                }
                visited.remove(&p);
                steps
            },
        };

        (dfs.f)(&dfs, start_pos, &mut HashSet::new())
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
        assert_eq!(puzzle.part1(), 94);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 154);
    }
}