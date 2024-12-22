//! [Day 22: Wizard Simulator 20XX](https://adventofcode.com/2015/day/22)

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

enum CastResult {
    PlayerWins,
    BossWins,
    CannotCast,
}

#[derive(Debug, EnumIter)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Clone)]
struct State {
    boss_dmg: i32,
    boss_hp: i32,
    hp: i32,
    mana: i32,
    timer_recharge: i32,
    timer_shield: i32,
    timer_poison: i32,
    mana_used: i32,
}

impl State {
    const fn new(boss_dmg: i32, boss_hp: i32, hp: i32, mana: i32) -> Self {
        Self {
            boss_dmg,
            boss_hp,
            hp,
            mana,
            timer_recharge: 0,
            timer_shield: 0,
            timer_poison: 0,
            mana_used: 0,
        }
    }

    fn apply_effects(&mut self) -> Option<CastResult> {
        if self.timer_recharge > 0 {
            self.timer_recharge -= 1;
            self.mana += 101;
        }

        if self.timer_shield > 0 {
            self.timer_shield -= 1;
        }

        if self.timer_poison > 0 {
            self.timer_poison -= 1;
            self.boss_hp -= 3;

            if self.boss_hp <= 0 {
                return Some(CastResult::PlayerWins);
            }
        }

        None
    }

    fn cast(&mut self, spell: &Spell, hard_mode: bool) -> Option<CastResult> {
        // player's turn

        if hard_mode {
            self.hp -= 1;
            if self.hp <= 0 {
                return Some(CastResult::BossWins);
            }
        }

        let s = self.apply_effects();
        if s.is_some() {
            return s;
        }

        match spell {
            Spell::MagicMissile => {
                self.mana -= 53;
                self.mana_used += 53;
                self.boss_hp -= 4;
            }
            Spell::Drain => {
                self.mana -= 73;
                self.mana_used += 73;
                self.hp += 2;
                self.boss_hp -= 2;
            }
            Spell::Shield => {
                if self.timer_shield != 0 {
                    return Some(CastResult::CannotCast);
                }
                self.mana -= 113;
                self.mana_used += 113;
                self.timer_shield = 6;
            }
            Spell::Poison => {
                if self.timer_poison != 0 {
                    return Some(CastResult::CannotCast);
                }
                self.mana -= 173;
                self.mana_used += 173;
                self.timer_poison = 6;
            }
            Spell::Recharge => {
                if self.timer_recharge != 0 {
                    return Some(CastResult::CannotCast);
                }
                self.mana -= 229;
                self.mana_used += 229;
                self.timer_recharge = 5;
            }
        }
        if self.mana < 0 {
            return Some(CastResult::CannotCast);
        }
        if self.boss_hp <= 0 {
            return Some(CastResult::PlayerWins);
        }

        // boss's turn

        let s = self.apply_effects();
        if s.is_some() {
            return s;
        }

        let armor = if self.timer_shield > 0 { 7 } else { 0 };

        self.hp -= 1.max(self.boss_dmg - armor);
        if self.hp <= 0 {
            return Some(CastResult::BossWins);
        }

        None
    }
}

struct Puzzle {
    boss_hp: i32,
    boss_dmg: i32,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            boss_hp: 0,
            boss_dmg: 0,
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.trim().lines() {
            if let Some(hp) = line.strip_prefix("Hit Points: ") {
                self.boss_hp = hp.parse().unwrap();
            } else if let Some(dmg) = line.strip_prefix("Damage: ") {
                self.boss_dmg = dmg.parse().unwrap();
            } else {
                panic!("bad input: {line}");
            }
        }
    }

    fn play(&self, hard_mode: bool) -> i32 {
        let mut states = vec![State::new(self.boss_dmg, self.boss_hp, 50, 500)];

        while !states.is_empty() {
            let mut mana_used = i32::MAX;
            let mut new = vec![];

            for state in &states {
                for spell in Spell::iter() {
                    let mut s = state.clone();

                    match s.cast(&spell, hard_mode) {
                        Some(CastResult::PlayerWins) => {
                            mana_used = mana_used.min(s.mana_used);
                            break;
                        }
                        Some(CastResult::BossWins) => {
                            break;
                        }
                        None => new.push(s),
                        Some(CastResult::CannotCast) => (),
                    }
                }
            }
            states = new;

            if mana_used != i32::MAX {
                return mana_used;
            }
        }

        0
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        self.play(false)
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        self.play(true)
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
