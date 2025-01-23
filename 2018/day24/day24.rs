//! [Day 24: Immune System Simulator 20XX](https://adventofcode.com/2018/day/24)

mod army;
mod attacktype;
mod combat;
mod group;

use combat::Combat;

struct Puzzle {
    combat: Combat,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let (army1, army2) = data.split_once("\n\n").unwrap();

        Self {
            combat: Combat::with_armies(army1.parse().unwrap(), army2.parse().unwrap()),
        }
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

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
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
        assert_eq!(puzzle.part1(), 5216);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new(TEST_INPUT);

        puzzle.combat.set_army1_boost(1570);

        assert_eq!(puzzle.part1(), 51);
    }
}
