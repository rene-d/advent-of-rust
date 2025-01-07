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
            let m = (a + b) / 2;
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

fn main() {
    let args = aoc::parse_args();
    let puzzle = Puzzle::new(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let data = aoc::load_input_data("sample_1.txt");
        let puzzle = Puzzle::new(&data);
        assert_eq!(puzzle.part1(), 31);
    }

    #[test]
    fn test2() {
        let data = aoc::load_input_data("sample_2.txt");
        let puzzle = Puzzle::new(&data);
        assert_eq!(puzzle.part1(), 165);
    }

    #[test]
    fn test3() {
        let data = aoc::load_input_data("sample_3.txt");
        let puzzle = Puzzle::new(&data);
        assert_eq!(puzzle.part1(), 13312);
        assert_eq!(puzzle.part2(), 82892753);
    }

    #[test]
    fn test4() {
        let data = aoc::load_input_data("sample_4.txt");
        let puzzle = Puzzle::new(&data);
        assert_eq!(puzzle.part1(), 180697);
        assert_eq!(puzzle.part2(), 5586022);
    }
    #[test]
    fn test5() {
        let data = aoc::load_input_data("sample_5.txt");
        let puzzle = Puzzle::new(&data);
        assert_eq!(puzzle.part1(), 2210736);
        assert_eq!(puzzle.part2(), 460664);
    }
}
