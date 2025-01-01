//! [Day 3: Crossed Wires](https://adventofcode.com/2019/day/3)

use rustc_hash::FxHashSet;

#[derive(Debug)]
struct Instr {
    delta: (i32, i32),
    distance: u32,
}

fn draw(instrs: &[Instr]) -> FxHashSet<(i32, i32)> {
    let mut line = FxHashSet::default();

    let (mut x, mut y) = (0, 0);

    for i in instrs {
        (0..i.distance).for_each(|_| {
            x += i.delta.0;
            y += i.delta.1;

            line.insert((x, y));
        });
    }

    line
}

fn steps(instrs: &[Instr], target: (i32, i32)) -> u32 {
    let mut count = 0u32;
    let (mut x, mut y) = (0, 0);

    for i in instrs {
        for _ in 0..i.distance {
            x += i.delta.0;
            y += i.delta.1;
            count += 1;

            if (x, y) == target {
                return count;
            }
        }
    }

    0
}

const fn manhattan(p: (i32, i32)) -> i32 {
    p.0.abs() + p.1.abs()
}

struct Puzzle {
    paths: Vec<Vec<Instr>>,
}

impl Puzzle {
    const fn new() -> Self {
        Self { paths: Vec::new() }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        for line in data.lines() {
            self.paths.push(
                line.split(',')
                    .map(|s| Instr {
                        delta: match s.chars().nth(0).unwrap() {
                            'U' => (0, 1),
                            'D' => (0, -1),
                            'L' => (-1, 0),
                            'R' => (1, 0),
                            _ => panic!(),
                        },
                        distance: s[1..].parse().unwrap(),
                    })
                    .collect::<Vec<_>>(),
            );
        }
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        let wire0 = draw(&self.paths[0]);
        let wire1 = draw(&self.paths[1]);

        wire0
            .intersection(&wire1)
            .map(|&p| manhattan(p))
            .min()
            .unwrap()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let wire0 = draw(&self.paths[0]);
        let wire1 = draw(&self.paths[1]);

        wire0
            .intersection(&wire1)
            .map(|&target| steps(&self.paths[0], target) + steps(&self.paths[1], target))
            .min()
            .unwrap()
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
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
        puzzle.configure(&aoc::load_input_data("test1.txt"));
        assert_eq!(puzzle.part1(), 159);
        assert_eq!(puzzle.part2(), 610);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test2.txt"));
        assert_eq!(puzzle.part1(), 135);
        assert_eq!(puzzle.part2(), 410);
    }
}
