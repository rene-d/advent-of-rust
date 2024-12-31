//! [Day 24: Immune System Simulator 20XX](https://adventofcode.com/2018/day/24)

use day24::combat::Combat;

struct Puzzle {
    combat: Combat,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            combat: Combat::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let (army1, army2) = data.split_once("\n\n").unwrap();

        self.combat = Combat::with_armies(army1.parse()?, army2.parse()?);

        Ok(true)
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let combat = self.combat.clone();

        combat.fight_to_death()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let mut a = 0;
        let mut b = 2000;

        while a <= b {
            let m = (a + b) / 2;

            let mut combat: Combat = self.combat.clone();
            combat.set_army1_boost(m);
            let _ = combat.fight_to_death();

            match combat.infection_alive_units() {
                units if units > 0 => a = m + 1,
                0 => b = m - 1,
                _ => (),
            };
        }

        let mut combat: Combat = self.combat.clone();
        combat.set_army1_boost(a);
        let _ = combat.fight_to_death();
        combat.immune_alive_units()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input)?;
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
    Ok(())
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt")).unwrap();
        assert_eq!(puzzle.part1(), 5216);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt")).unwrap();

        puzzle.combat.set_army1_boost(1570);

        assert_eq!(puzzle.part1(), 51);
    }
}
