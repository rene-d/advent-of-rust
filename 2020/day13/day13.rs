//! [Day 13: Shuttle Search](https://adventofcode.com/2020/day/13)

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

struct Puzzle {
    depart: u32,
    buses: Vec<String>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            depart: 0,
            buses: vec![],
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

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

        for (t, id) in self.buses.iter().enumerate() {
            if id != "x" {
                let id: i64 = id.parse().unwrap();
                let t = i64::try_from(t).unwrap();
                modulii.push(id);
                residues.push(id - t);
            }
        }

        chinese_remainder(&residues, &modulii).unwrap()
    }
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 295);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 1068781);
    }

    #[test]
    fn test03() {
        let mut puzzle = Puzzle::new();

        puzzle.set_buses("17,x,13,19");
        assert_eq!(puzzle.part2(), 3417);

        puzzle.set_buses("67,7,59,61");
        assert_eq!(puzzle.part2(), 754018);

        puzzle.set_buses("67,x,7,59,61");
        assert_eq!(puzzle.part2(), 779210);

        puzzle.set_buses("67,7,x,59,61");
        assert_eq!(puzzle.part2(), 1261476);

        puzzle.set_buses("1789,37,47,1889");
        assert_eq!(puzzle.part2(), 1202161486);
    }
}

fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
