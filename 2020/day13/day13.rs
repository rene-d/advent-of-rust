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
    const fn new() -> Self {
        Self {
            depart: 0,
            buses: vec![],
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        let mut lines = data.lines();
        self.depart = lines.next().unwrap().parse().unwrap();
        self.set_buses(lines.next().unwrap());
    }

    fn set_buses(&mut self, buses: &str) {
        self.buses = buses.split(',').map(str::to_string).collect();
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
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part1(), 295);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part2(), 1_068_781);
    }

    #[test]
    fn test03() {
        let mut puzzle = Puzzle::new();

        puzzle.set_buses("17,x,13,19");
        assert_eq!(puzzle.part2(), 3417);

        puzzle.set_buses("67,7,59,61");
        assert_eq!(puzzle.part2(), 754_018);

        puzzle.set_buses("67,x,7,59,61");
        assert_eq!(puzzle.part2(), 779_210);

        puzzle.set_buses("67,7,x,59,61");
        assert_eq!(puzzle.part2(), 1_261_476);

        puzzle.set_buses("1789,37,47,1889");
        assert_eq!(puzzle.part2(), 1_202_161_486);
    }
}
