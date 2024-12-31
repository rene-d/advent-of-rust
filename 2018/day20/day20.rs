//! [Day 20: A Regular Map](https://adventofcode.com/2018/day/20)

use std::collections::{HashMap, HashSet, VecDeque};

struct Puzzle {
    edges: HashMap<(i32, i32), HashSet<(i32, i32)>>,

    max_steps: u32,
    thousand_doors: u32,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
            max_steps: 0,
            thousand_doors: 0,
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        self.parse(data.trim());
    }

    fn parse(&mut self, input: &str) {
        let mut branchs = vec![];

        let mut x = 0;
        let mut y = 0;

        for c in input.chars() {
            match c {
                '^' | '$' => (),
                '(' => branchs.push((x, y)),
                ')' => {
                    (x, y) = branchs.pop().unwrap();
                }
                '|' => {
                    (x, y) = *branchs.last().unwrap();
                }
                _ => {
                    let (dx, dy) = match c {
                        'N' => (0, -1),
                        'E' => (1, 0),
                        'S' => (0, 1),
                        'W' => (-1, 0),
                        _ => panic!("unknown char '{c}"),
                    };
                    (*self.edges.entry((x, y)).or_default()).insert((dx, dy));

                    x += dx;
                    y += dy;
                }
            }
        }
    }

    fn solve(&mut self) {
        let mut q = VecDeque::new();
        let mut seen = HashSet::new();

        self.max_steps = 0;
        self.thousand_doors = 0;

        q.push_back((0, 0, 0));
        seen.insert((0, 0));

        while let Some((steps, x, y)) = q.pop_front() {
            self.max_steps = self.max_steps.max(steps);

            if steps >= 1000 {
                self.thousand_doors += 1;
            }

            if let Some(neighbors) = self.edges.get(&(x, y)) {
                for (dx, dy) in neighbors {
                    let nx = x + dx;
                    let ny = y + dy;

                    if seen.insert((nx, ny)) {
                        q.push_back((steps + 1, nx, ny));
                    }
                }
            }
        }
    }

    /// Solve part one.
    const fn part1(&self) -> u32 {
        self.max_steps
    }

    /// Solve part two.
    const fn part2(&self) -> u32 {
        self.thousand_doors
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
    puzzle.solve();
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
        puzzle.parse("^ENWWW(NEEE|SSE(EE|N))$");
        puzzle.solve();
        assert_eq!(puzzle.part1(), 10);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.parse("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$");
        puzzle.solve();
        assert_eq!(puzzle.part1(), 18);
    }

    #[test]
    fn test03() {
        let mut puzzle = Puzzle::new();
        puzzle.parse("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$");
        puzzle.solve();
        assert_eq!(puzzle.part1(), 23);
    }

    #[test]
    fn test04() {
        let mut puzzle = Puzzle::new();
        puzzle.parse("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$");
        puzzle.solve();
        assert_eq!(puzzle.part1(), 31);
    }
}
