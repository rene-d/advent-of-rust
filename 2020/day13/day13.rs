//! [Day 13: Shuttle Search](https://adventofcode.com/2020/day/13)

use aoc::math::SignedMathOps;

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * p.mod_inv(modulus)? * p;
    }

    Some(sum.rem_euclid(prod))
}

struct Puzzle {
    depart: u32,
    buses: Vec<String>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let data = data.split_once('\n').unwrap();

        Self {
            depart: data.0.parse().unwrap(),
            buses: data.1.trim_ascii().split(',').map(str::to_string).collect(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let (minutes, id) = self
            .buses
            .iter()
            .filter(|&id| id != "x")
            .map(|id| {
                let id = id.parse::<u32>().unwrap();
                (((self.depart / id) + 1) * id - self.depart, id)
            })
            .min()
            .unwrap();
        minutes * id
    }

    /// Solve part two.
    fn part2(&self) -> i64 {
        let mut residues = vec![];
        let mut modulii = vec![];

        for (id, t) in self.buses.iter().zip(0..) {
            if id != "x" {
                let id: i64 = id.parse().unwrap();
                modulii.push(id);
                residues.push(id - t);
            }
        }

        chinese_remainder(&residues, &modulii).unwrap()
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u32, i64) {
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

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 295);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 1_068_781);
    }

    #[test]
    fn test03() {
        let puzzle = Puzzle::new("0\n17,x,13,19");
        assert_eq!(puzzle.part2(), 3417);

        let puzzle = Puzzle::new("0\n67,7,59,61");
        assert_eq!(puzzle.part2(), 754_018);

        let puzzle = Puzzle::new("0\n67,x,7,59,61");
        assert_eq!(puzzle.part2(), 779_210);

        let puzzle = Puzzle::new("0\n67,7,x,59,61");
        assert_eq!(puzzle.part2(), 1_261_476);

        let puzzle = Puzzle::new("0\n1789,37,47,1889");
        assert_eq!(puzzle.part2(), 1_202_161_486);
    }
}
