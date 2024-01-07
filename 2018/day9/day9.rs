//! [Day 9: Marble Mania](https://adventofcode.com/2018/day/9)

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

fn solve(nb_players: u32, nb_marbles: u32) -> u32 {
    let mut marbles = vec![0u32];
    let mut scores = vec![0u32; nb_players as usize];

    let mut current = 1;

    for i in 1..=nb_marbles {
        if i % 23 == 0 {
            current = (current + marbles.len() - 7) % marbles.len();
            scores[(i % nb_players) as usize] += i + marbles[current];
            marbles.remove(current);
        } else {
            current = (current + 2) % marbles.len();
            marbles.insert(current, i);
        }
    }
    *scores.iter().max().unwrap()
}

struct Puzzle {
    elves: u32,
    points: u32,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            elves: 9,
            points: 25,
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        let row = data.split_ascii_whitespace().collect::<Vec<_>>();
        match &row[..] {
            [n, _, _, _, _, _, p, _] => {
                self.elves = n.parse().unwrap();
                self.points = p.parse().unwrap();
            }
            _ => panic!("bad input: {data}"),
        };
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        solve(self.elves, self.points)
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        solve(self.elves, self.points * 100)
    }
}

fn main() {
    let args = Args::parse();
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
        let puzzle = Puzzle::new();

        assert_eq!(puzzle.part1(), 32);
    }

    #[test]
    fn test02() {
        assert_eq!(solve(10, 1618), 8317);
    }

    #[test]
    fn test03() {
        assert_eq!(solve(13, 7999), 146373);
    }

    #[test]
    fn test04() {
        assert_eq!(solve(21, 6111), 54718);
    }

    #[test]
    fn test05() {
        assert_eq!(solve(30, 5807), 37305);
    }
}
