//! [Day 21: RPG Simulator 20XX](https://adventofcode.com/2015/day/21)

use std::cmp::{max, min};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Character {
    _name: String,
    hitpoints: i32,
    damage: i32,
    armor: i32,
}

impl Character {
    fn new(name: &str, hitpoints: i32, damage: i32, armor: i32) -> Self {
        Self {
            _name: name.to_string(),
            hitpoints,
            damage,
            armor,
        }
    }

    fn attack(&self, enemy: &mut Self) {
        let damage = max(0, self.damage - enemy.armor);
        enemy.hitpoints -= damage;

        if enemy.hitpoints < 0 {
            enemy.hitpoints = 0;
        }

        // Optional: Uncomment for debugging
        // println!(
        //     "The {} deals {}-{} = {} damage; the {} goes down to {} hit points.",
        //     self.name, self.damage, enemy.armor, damage, enemy.name, enemy.hitpoints
        // );
    }
}

fn combat(c1: &mut Character, c2: &mut Character) -> i32 {
    loop {
        c1.attack(c2);
        if c2.hitpoints == 0 {
            return 1;
        }

        c2.attack(c1);
        if c1.hitpoints == 0 {
            return 2;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

fn parse_items(input: &str) -> Vec<Item> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let cost = i32::from_str(parts[parts.len() - 3]).unwrap();
            let damage = i32::from_str(parts[parts.len() - 2]).unwrap();
            let armor = i32::from_str(parts[parts.len() - 1]).unwrap();
            Item {
                cost,
                damage,
                armor,
            }
        })
        .collect()
}

fn play(boss: &Character) -> (i32, i32) {
    let weapons = parse_items(
        r"
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0
",
    );

    let mut armors = parse_items(
        r"
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5
",
    );

    let mut rings = parse_items(
        r"
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3
",
    );

    // Add "no armor" and "no ring" options
    armors.push(Item {
        cost: 0,
        damage: 0,
        armor: 0,
    });
    rings.push(Item {
        cost: 0,
        damage: 0,
        armor: 0,
    });

    let mut min_win_cost = i32::MAX;
    let mut max_loose_cost = 0;

    for w in &weapons {
        for a in &armors {
            for r1 in &rings {
                for r2 in &rings {
                    // Cannot buy two of the same rings
                    if r1 == r2 {
                        continue;
                    }

                    let cost = w.cost + a.cost + r1.cost + r2.cost;
                    let damage = w.damage + a.damage + r1.damage + r2.damage;
                    let armor = w.armor + a.armor + r1.armor + r2.armor;

                    let mut player = Character::new("player", 100, damage, armor);
                    let mut boss_copy = boss.clone();

                    if combat(&mut player, &mut boss_copy) == 1 {
                        min_win_cost = min(min_win_cost, cost);
                    } else {
                        max_loose_cost = max(max_loose_cost, cost);
                    }
                }
            }
        }
    }

    (min_win_cost, max_loose_cost)
}

fn solve(data: &str) -> (i32, i32) {
    let mut boss = Character::new("boss", 0, 0, 0);

    for line in data.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        match parts.first() {
            Some(&"Hit Points") => boss.hitpoints = parts[1].parse().unwrap(),
            Some(&"Damage") => boss.damage = parts[1].parse().unwrap(),
            Some(&"Armor") => boss.armor = parts[1].parse().unwrap(),
            _ => {}
        }
    }

    play(&boss)
}

fn main() {
    let mut args = aoc::parse_args();
    args.run(solve);
}
