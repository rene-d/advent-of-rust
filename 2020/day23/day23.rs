//! [Day 23: Crab Cups](https://adventofcode.com/2020/day/23)

struct Puzzle {
    cups: Vec<usize>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { cups: Vec::new() }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.cups = data
            .chars()
            .filter_map(|x| x.to_digit(10))
            .map(|x| x as usize)
            .collect();
    }

    fn solve(&self, nb_cups: usize, nb_moves: usize) -> u64 {
        // cups are labeled from 1 to 9 (and from 10 to ncups), cup no. 0 is *unused*

        let cups = |i: usize| {
            if i >= self.cups.len() {
                i + 1
            } else {
                self.cups[i]
            }
        };

        let minus_one = |i: usize| {
            if i == 1 {
                nb_cups
            } else {
                i - 1
            }
        };

        let mut pos = vec![0usize; nb_cups + 1];

        for i in 0..nb_cups {
            pos[cups(i)] = cups((i + 1) % nb_cups);
        }

        let mut current = cups(0);

        for _ in 0..nb_moves {
            let pickup = pos[current];

            let idx = pos[pos[pos[pickup]]];
            pos[current] = idx;

            let mut dest = minus_one(current);

            while dest == pickup || dest == pos[pickup] || dest == pos[pos[pickup]] {
                dest = minus_one(dest);
            }

            let idx = pos[pos[pickup]];
            pos[idx] = pos[dest];

            pos[dest] = pickup;
            current = pos[current];
        }

        if nb_cups > self.cups.len() {
            // answer for part 2
            (pos[pos[1]] as u64) * (pos[1] as u64)
        } else {
            // answer for part 1
            let mut result = 0;
            let mut cup = pos[1];
            while cup != 1 {
                result = result * 10 + cup as u64;
                cup = pos[cup];
            }
            result
        }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        self.solve(self.cups.len(), 100)
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        self.solve(1_000_000, 10_000_000)
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
        assert_eq!(puzzle.part1(), 67_384_529);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 149_245_887_792);
    }
}
