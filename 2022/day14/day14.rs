//! [Day 14: Regolith Reservoir](https://adventofcode.com/2022/day/14)

use rustc_hash::FxHashSet;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Coord {
    x: u32,
    y: u32,
}

struct Puzzle {
    wall: FxHashSet<Coord>,
    floor: u32,
    sand: usize,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            wall: FxHashSet::default(),
            floor: 0,
            sand: 0,
        }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, data: &str) {
        let lines = data.split('\n').collect::<Vec<_>>();

        for line in lines {
            if line.is_empty() {
                continue;
            }
            let path = line
                .split(" -> ")
                .map(|p| {
                    let mut xy = p.split(',');
                    let x = xy.next().unwrap().parse::<u32>().unwrap();
                    let y = xy.next().unwrap().parse::<u32>().unwrap();
                    Coord { x, y }
                })
                .collect::<Vec<_>>();

            for i in 0..(path.len() - 1) {
                let p1 = &path[i];
                let p2 = &path[i + 1];

                if p1.x == p2.x {
                    let y1 = p1.y.min(p2.y);
                    let y2 = p1.y.max(p2.y);
                    for y in y1..=y2 {
                        self.wall.insert(Coord { x: p1.x, y });
                    }
                } else if p1.y == p2.y {
                    let x1 = p1.x.min(p2.x);
                    let x2 = p1.x.max(p2.x);
                    for x in x1..=x2 {
                        self.wall.insert(Coord { x, y: p1.y });
                    }
                } else {
                    panic!("{p1:?} {p2:?} diagonal");
                }
            }
        }

        self.floor = self.wall.iter().map(|p| p.y).max().unwrap() + 2;
    }

    fn fall(&mut self, part2: bool) -> bool {
        let start = Coord { x: 500, y: 0 };

        let mut x = start.x;
        let mut y = start.y;

        loop {
            if y + 1 >= self.floor {
                if part2 {
                    break;
                }

                // sand is beyond the lowest rock
                return false;
            }

            if self.is_empty(x, y + 1) {
                // fall vertically
                y += 1;
            } else if self.is_empty(x - 1, y + 1) {
                // fall diagonally to the left
                y += 1;
                x -= 1;
            } else if self.is_empty(x + 1, y + 1) {
                // fall diagonally to the right
                y += 1;
                x += 1;
            } else {
                // sand is blocked
                break;
            }
        }

        self.wall.insert(Coord { x, y });

        // if part1, always return true (sand cannot be at the starting point)
        // if part2, return false if the sand is at the starting point
        !(x == start.x && y == start.y)
    }

    fn is_empty(&self, x: u32, y: u32) -> bool {
        !self.wall.contains(&Coord { x, y })
    }

    // Solves part one
    fn part1(&mut self) -> usize {
        for i in 0..10000 {
            if !self.fall(false) {
                self.sand = i;
                break;
            }
        }
        self.sand
    }

    // Solve part two
    fn part2(&mut self) -> usize {
        for i in 1..100_000 {
            if !self.fall(true) {
                self.sand += i;
                break;
            }
        }
        self.sand
    }
}

/// main function
fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure(&aoc::load_input_data("test.txt"));
    assert_eq!(puzzle.part1(), 24);
    assert_eq!(puzzle.part2(), 93);
}
