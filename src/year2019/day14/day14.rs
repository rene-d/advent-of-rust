//! [Day 14: Space Stoichiometry](https://adventofcode.com/2019/day/14)

use rustc_hash::FxHashMap;

type Formulae<'a> = FxHashMap<&'a str, (i64, Vec<(i64, &'a str)>)>;

struct Reaction<'a> {
    excess: FxHashMap<&'a str, i64>,
    formulae: &'a Formulae<'a>,
}

impl<'a> Reaction<'a> {
    fn calc(&mut self, mut amount: i64, chemical: &'a str) -> i64 {
        if chemical == "FUEL" {
            self.excess.clear();
        }

        if chemical == "ORE" {
            return amount;
        }

        let v_chemical = self.excess.entry(chemical).or_insert(0);
        if amount <= *v_chemical {
            *v_chemical -= amount;
            return 0;
        }

        amount -= *v_chemical;
        self.excess.insert(chemical, 0);

        let (produced, reagents) = self.formulae[chemical].clone();

        let nb_reactions = (produced - 1 + amount) / produced;

        *self.excess.entry(chemical).or_default() += nb_reactions * produced - amount;

        reagents
            .iter()
            .map(|i| self.calc(nb_reactions * i.0, i.1))
            .sum()
    }
}

struct Puzzle<'a> {
    formulae: Formulae<'a>,
}

impl<'a> Puzzle<'a> {
    /// Initialize from the puzzle input.
    fn new(data: &'a str) -> Self {
        let mut formulae = FxHashMap::default();

        for formula in data.lines() {
            let (input, output) = formula.split_once(" => ").unwrap();

            let (output_quantity, output_cheminal) = output.split_once(' ').unwrap();

            let mut reagents = Vec::new();

            for reagent in input.split(", ") {
                let (quantity_reagent, chemical_reagent) = reagent.split_once(' ').unwrap();
                reagents.push((quantity_reagent.parse().unwrap(), chemical_reagent));
            }

            formulae.insert(
                output_cheminal,
                (output_quantity.parse().unwrap(), reagents),
            );
        }

        Self { formulae }
    }

    /// Solve part one.
    fn part1(&self) -> i64 {
        let mut c = Reaction {
            excess: FxHashMap::default(),
            formulae: &self.formulae,
        };

        c.calc(1, "FUEL")
    }

    /// Solve part two.
    fn part2(&self) -> i64 {
        let mut c = Reaction {
            excess: FxHashMap::default(),
            formulae: &self.formulae,
        };

        let ore = 1_000_000_000_000;

        let mut a = 1;
        let mut b = ore;

        while b - a > 1 {
            let m = i64::midpoint(a, b);
            let c = c.calc(m, "FUEL");
            if c > ore {
                b = m;
            } else {
                a = m;
            }
        }

        a
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (i64, i64) {
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

    const SAMPLE_1: &str = include_str!("sample_1.txt");
    const SAMPLE_2: &str = include_str!("sample_2.txt");
    const SAMPLE_3: &str = include_str!("sample_3.txt");
    const SAMPLE_4: &str = include_str!("sample_4.txt");
    const SAMPLE_5: &str = include_str!("sample_5.txt");

    #[test]
    fn test1() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part1(), 31);
    }

    #[test]
    fn test2() {
        let puzzle = Puzzle::new(SAMPLE_2);
        assert_eq!(puzzle.part1(), 165);
    }

    #[test]
    fn test3() {
        let puzzle = Puzzle::new(SAMPLE_3);
        assert_eq!(puzzle.part1(), 13312);
        assert_eq!(puzzle.part2(), 82892753);
    }

    #[test]
    fn test4() {
        let puzzle = Puzzle::new(SAMPLE_4);
        assert_eq!(puzzle.part1(), 180697);
        assert_eq!(puzzle.part2(), 5586022);
    }
    #[test]
    fn test5() {
        let puzzle = Puzzle::new(SAMPLE_5);
        assert_eq!(puzzle.part1(), 2210736);
        assert_eq!(puzzle.part2(), 460664);
    }
}
