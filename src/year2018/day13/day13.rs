//! [Day 13: Mine Cart Madness](https://adventofcode.com/2018/day/13)

use aoc::Direction;
use aoc::GridU;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

type Grid = GridU<char>;

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
struct Cart {
    x: usize,
    y: usize,
    d: Direction,
    turn: i8, // 0:turn left, 1:go straigh, 2:turn right
    destroyed: bool,
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl Cart {
    fn update(&mut self, grid: &Grid) {
        match self.d {
            Direction::West => {
                self.x -= 1;
                match grid[(self.x, self.y)] {
                    '-' => (),
                    '/' => self.d = Direction::South,
                    '\\' => self.d = Direction::North,
                    '+' => {
                        self.d = match self.turn {
                            0 => Direction::South, // turn left
                            1 => Direction::West,  // go straight
                            2 => Direction::North, // turn right
                            _ => unreachable!(),
                        };
                        self.turn = (self.turn + 1) % 3;
                    }
                    _ => panic!(),
                }
            }

            Direction::East => {
                self.x += 1;
                match grid[(self.x, self.y)] {
                    '-' => (),
                    '/' => self.d = Direction::North,
                    '\\' => self.d = Direction::South,
                    '+' => {
                        self.d = match self.turn {
                            0 => Direction::North, // turn left
                            1 => Direction::East,  // go straight
                            2 => Direction::South, // turn right
                            _ => unreachable!(),
                        };
                        self.turn = (self.turn + 1) % 3;
                    }
                    _ => panic!(),
                }
            }

            Direction::North => {
                self.y -= 1;
                match grid[(self.x, self.y)] {
                    '|' => (),
                    '/' => self.d = Direction::East,
                    '\\' => self.d = Direction::West,
                    '+' => {
                        self.d = match self.turn {
                            0 => Direction::West,  // turn left
                            1 => Direction::North, // go straight
                            2 => Direction::East,  // turn right
                            _ => unreachable!(),
                        };
                        self.turn = (self.turn + 1) % 3;
                    }
                    _ => panic!(),
                }
            }

            Direction::South => {
                self.y += 1;
                match grid[(self.x, self.y)] {
                    '|' => (),
                    '/' => self.d = Direction::West,
                    '\\' => self.d = Direction::East,
                    '+' => {
                        self.d = match self.turn {
                            0 => Direction::East,  // turn left
                            1 => Direction::South, // go straight
                            2 => Direction::West,  // turn right
                            _ => unreachable!(),
                        };
                        self.turn = (self.turn + 1) % 3;
                    }
                    _ => panic!(),
                }
            }
        }
    }
}

struct Puzzle {
    grid: Grid,
    carts: Vec<Cart>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut grid = GridU::<char>::parse(data);
        let mut carts = Vec::new();

        for ((x, y), c) in grid.iter_mut() {
            match c {
                '<' => {
                    carts.push(Cart {
                        x,
                        y,
                        d: Direction::West,
                        turn: 0,
                        destroyed: false,
                    });
                    *c = '-';
                }
                '>' => {
                    carts.push(Cart {
                        x,
                        y,
                        d: Direction::East,
                        turn: 0,
                        destroyed: false,
                    });
                    *c = '-';
                }
                '^' => {
                    carts.push(Cart {
                        x,
                        y,
                        d: Direction::North,
                        turn: 0,
                        destroyed: false,
                    });
                    *c = '|';
                }
                'v' => {
                    carts.push(Cart {
                        x,
                        y,
                        d: Direction::South,
                        turn: 0,
                        destroyed: false,
                    });
                    *c = '|';
                }
                _ => (),
            }
        }

        Self { grid, carts }
    }

    #[allow(dead_code)]
    fn show(&self, carts: &[Cart]) {
        let carts = carts
            .iter()
            .filter(|cart| !cart.destroyed)
            .map(|cart| ((cart.x, cart.y), cart.d))
            .collect::<FxHashMap<_, _>>();

        for (xy, c) in self.grid.iter() {
            if let Some(d) = carts.get(&xy) {
                print!("\x1b[32m{}\x1b[0m", d.arrow());
            } else {
                print!("{c}");
            }
            if xy.0 == self.grid.size().0 - 1 {
                println!();
            }
        }
    }

    fn move_carts(&self, carts: &mut Vec<Cart>, destroy: bool) -> Option<Cart> {
        let mut new_carts: Vec<Cart> = vec![];

        let mut q: VecDeque<_> = carts.iter().copied().collect();

        'q: while let Some(cart) = q.pop_front() {
            let mut nc = cart;

            nc.update(&self.grid);

            for z in &mut new_carts {
                if nc.x == z.x && nc.y == z.y {
                    z.destroyed = true;
                    if destroy {
                        continue 'q;
                    }
                    return Some(nc);
                }
            }

            for z in &mut q {
                if nc.x == z.x && nc.y == z.y {
                    z.destroyed = true;
                    if destroy {
                        continue 'q;
                    }
                    return Some(nc);
                }
            }

            new_carts.push(nc);
        }

        *carts = new_carts
            .iter()
            .filter(|cart| !cart.destroyed)
            .copied()
            .collect();

        None
    }

    /// Solve part one.
    fn part1(&self) -> String {
        let mut carts = self.carts.clone();

        for _ in 0..100_000 {
            carts.sort_unstable();

            // if self.verbose {
            //     self.show(&carts);
            // }

            if let Some(c) = self.move_carts(&mut carts, false) {
                return format!("{},{}", c.x, c.y);
            }
        }

        "no solution".to_string()
    }

    /// Solve part two.
    fn part2(&self) -> String {
        let mut carts = self.carts.clone();

        for _ in 0..100_000 {
            carts.sort_unstable();

            self.move_carts(&mut carts, true);

            if carts.is_empty() {
                break;
            }

            if carts.len() == 1 {
                return format!("{},{}", carts[0].x, carts[0].y);
            }
        }

        "no solution".to_string()
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (String, String) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_4: &str = include_str!("sample_4.txt");
    const SAMPLE_6: &str = include_str!("sample_6.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(SAMPLE_4);
        assert_eq!(puzzle.part1(), "7,3");
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(SAMPLE_6);
        assert_eq!(puzzle.part2(), "6,4");
    }
}
