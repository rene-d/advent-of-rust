//! [Day 17: Two Steps Forward](https://adventofcode.com/2016/day/17)

use std::collections::VecDeque;

struct Puzzle<'a> {
    password: &'a str,
}

impl<'a> Puzzle<'a> {
    const fn new(data: &'a str) -> Self {
        Self {
            password: data.trim_ascii(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> String {
        let mut base_digest = md5::Context::new();

        base_digest.consume(self.password);

        let mut q = VecDeque::new();

        q.push_back((0, 0, 0, String::new()));

        while let Some((x, y, steps, path)) = q.pop_front() {
            if (x, y) == (3, 3) {
                return path;
            }

            let mut digest = base_digest.clone();
            digest.consume(&path);
            let hash = digest.finalize();
            let hash: Vec<_> = format!("{hash:x}").chars().collect();

            let is_open = |i| 'b' <= hash[i] && hash[i] <= 'f';

            // north/up 1st digit of the hash
            if y > 0 && is_open(0) {
                let np = path.clone() + "U";
                q.push_back((x, y - 1, steps + 1, np));
            }

            // south/down, 2nd digit of the hash
            if y < 3 && is_open(1) {
                let np = path.clone() + "D";
                q.push_back((x, y + 1, steps + 1, np));
            }

            //  west/left, 3rd digit of the hash
            if x > 0 && is_open(2) {
                let np = path.clone() + "L";
                q.push_back((x - 1, y, steps + 1, np));
            }

            // east/right, 4th digit of the hash
            if x < 3 && is_open(3) {
                let np = path.clone() + "R";
                q.push_back((x + 1, y, steps + 1, np));
            }
        }

        String::new()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let mut max_steps = 0;

        let mut base_digest = md5::Context::new();
        base_digest.consume(self.password);

        let mut q = VecDeque::new();
        q.push_back((0, 0, 0, base_digest.clone()));

        while let Some((x, y, steps, digest)) = q.pop_front() {
            if (x, y) == (3, 3) {
                if max_steps < steps {
                    max_steps = steps;
                }
                continue;
            }

            let hash = digest.clone().finalize();
            let hash: Vec<_> = format!("{hash:x}").chars().collect();

            let is_open = |i| 'b' <= hash[i] && hash[i] <= 'f';

            // north/up 1st digit of the hash
            if y > 0 && is_open(0) {
                let mut np = md5::Context::new();
                digest.clone_into(&mut np);
                np.consume("U");
                q.push_back((x, y - 1, steps + 1, np));
            }

            // south/down, 2nd digit of the hash
            if y < 3 && is_open(1) {
                let mut np = md5::Context::new();
                digest.clone_into(&mut np);
                np.consume("D");
                q.push_back((x, y + 1, steps + 1, np));
            }

            //  west/left, 3rd digit of the hash
            if x > 0 && is_open(2) {
                let mut np = md5::Context::new();
                digest.clone_into(&mut np);
                np.consume("L");
                q.push_back((x - 1, y, steps + 1, np));
            }

            // east/right, 4th digit of the hash
            if x < 3 && is_open(3) {
                let mut np = md5::Context::new();
                digest.clone_into(&mut np);
                np.consume("R");
                q.push_back((x + 1, y, steps + 1, np));
            }
        }

        max_steps
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (String, u32) {
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

    #[test]
    fn test00() {
        let puzzle = Puzzle::new("hijkl");
        assert_eq!(puzzle.part1(), ""); // no path
    }

    #[test]
    fn test01() {
        let puzzle = Puzzle::new("ihgpwlah");
        assert_eq!(puzzle.part1(), "DDRRRD");
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new("kglvqrro");
        assert_eq!(puzzle.part1(), "DDUDRLRRUDRD");
    }

    #[test]
    fn test03() {
        let puzzle = Puzzle::new("ulqzkmiv");
        assert_eq!(puzzle.part1(), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }

    #[test]
    fn test04() {
        let puzzle = Puzzle::new("ihgpwlah");
        assert_eq!(puzzle.part2(), 370);
    }

    #[test]
    fn test05() {
        let puzzle = Puzzle::new("kglvqrro");
        assert_eq!(puzzle.part2(), 492);
    }

    #[test]
    fn test06() {
        let puzzle = Puzzle::new("ulqzkmiv");
        assert_eq!(puzzle.part2(), 830);
    }
}
