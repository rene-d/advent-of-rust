//! [Day 13: Mine Cart Madness](https://adventofcode.com/2018/day/13)

#![allow(clippy::too_many_lines)]

use aoc::Direction;
use aoc::GridU;
use std::collections::HashMap;
use std::collections::VecDeque;

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

struct Puzzle {
    verbose: bool,
    grid: GridU<char>,
    carts: Vec<Cart>,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            verbose: false,
            grid: GridU::<char>::new(),
            carts: vec![],
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        self.grid = GridU::<char>::parse(data);

        for ((x, y), c) in self.grid.iter_mut() {
            match c {
                '<' => {
                    self.carts.push(Cart {
                        x,
                        y,
                        d: Direction::West,
                        turn: 0,
                        destroyed: false,
                    });
                    *c = '-';
                }
                '>' => {
                    self.carts.push(Cart {
                        x,
                        y,
                        d: Direction::East,
                        turn: 0,
                        destroyed: false,
                    });
                    *c = '-';
                }
                '^' => {
                    self.carts.push(Cart {
                        x,
                        y,
                        d: Direction::North,
                        turn: 0,
                        destroyed: false,
                    });
                    *c = '|';
                }
                'v' => {
                    self.carts.push(Cart {
                        x,
                        y,
                        d: Direction::South,
                        turn: 0,
                        destroyed: false,
                    });
                    *c = '|';
                }
                _ => (),
            };
        }
    }

    fn show(&self, carts: &[Cart]) {
        let carts = carts
            .iter()
            .filter(|cart| !cart.destroyed)
            .map(|cart| ((cart.x, cart.y), cart.d))
            .collect::<HashMap<_, _>>();

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

            match nc.d {
                Direction::West => {
                    nc.x -= 1;
                    match self.grid[(nc.x, nc.y)] {
                        '-' => (),
                        '/' => nc.d = Direction::South,
                        '\\' => nc.d = Direction::North,
                        '+' => {
                            nc.d = match nc.turn {
                                0 => Direction::South, // turn left
                                1 => Direction::West,  // go straight
                                2 => Direction::North, // turn right
                                _ => unreachable!(),
                            };
                            nc.turn = (nc.turn + 1) % 3;
                        }
                        _ => panic!(),
                    }
                }

                Direction::East => {
                    nc.x += 1;
                    match self.grid[(nc.x, nc.y)] {
                        '-' => (),
                        '/' => nc.d = Direction::North,
                        '\\' => nc.d = Direction::South,
                        '+' => {
                            nc.d = match nc.turn {
                                0 => Direction::North, // turn left
                                1 => Direction::East,  // go straight
                                2 => Direction::South, // turn right
                                _ => unreachable!(),
                            };
                            nc.turn = (nc.turn + 1) % 3;
                        }
                        _ => panic!(),
                    }
                }

                Direction::North => {
                    nc.y -= 1;
                    match self.grid[(nc.x, nc.y)] {
                        '|' => (),
                        '/' => nc.d = Direction::East,
                        '\\' => nc.d = Direction::West,
                        '+' => {
                            nc.d = match nc.turn {
                                0 => Direction::West,  // turn left
                                1 => Direction::North, // go straight
                                2 => Direction::East,  // turn right
                                _ => unreachable!(),
                            };
                            nc.turn = (nc.turn + 1) % 3;
                        }
                        _ => panic!(),
                    }
                }

                Direction::South => {
                    nc.y += 1;
                    match self.grid[(nc.x, nc.y)] {
                        '|' => (),
                        '/' => nc.d = Direction::West,
                        '\\' => nc.d = Direction::East,
                        '+' => {
                            nc.d = match nc.turn {
                                0 => Direction::East,  // turn left
                                1 => Direction::South, // go straight
                                2 => Direction::West,  // turn right
                                _ => unreachable!(),
                            };
                            nc.turn = (nc.turn + 1) % 3;
                        }
                        _ => panic!(),
                    }
                }
            }

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

            if self.verbose {
                self.show(&carts);
            }

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

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.verbose = args.verbose;
    puzzle.configure(&args.input);
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
        puzzle.configure(&aoc::load_input_data("sample_4.txt"));
        assert_eq!(puzzle.part1(), "7,3");
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("sample_6.txt"));
        assert_eq!(puzzle.part2(), "6,4");
    }
}
