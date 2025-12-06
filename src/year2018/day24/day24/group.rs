use regex::Regex;
use std::cell::Cell;
use std::error::Error;

use super::attacktype::AttackType;

//  __
// /__.__    ._
// \_||(_)|_||_)
//           |

/// A group of soldiers. Each unit shares the same number of hit points, has the same immunities and weaknesses, and deals the same damage.
/// The initiative is the unit's preferred rank for attacking.
#[derive(Debug, Clone)]
pub struct Group {
    army: String,
    id: u32,
    num_units: Cell<u32>,
    hp: u32,
    immunities: Vec<AttackType>,
    weaknesses: Vec<AttackType>,
    damage: u32,
    damage_type: AttackType,
    initiative: u32,
}

impl std::str::FromStr for Group {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re =  Regex::new(r"^(\d+) units each with (\d+) hit points( \(.+\))? with an attack that does (\d+) (\w+) damage at initiative (\d+)$").unwrap();

        let m = re.captures(s).ok_or("cannot parse line")?;

        let mut g = Self {
            army: String::new(),
            id: 0,
            num_units: Cell::new(m[1].parse()?),
            hp: m[2].parse()?,
            immunities: vec![],
            weaknesses: vec![],
            damage: m[4].parse()?,
            damage_type: m[5].parse()?,
            initiative: m[6].parse()?,
        };

        if let Some(s) = m.get(3) {
            for s in s
                .as_str()
                .strip_prefix(" (")
                .ok_or("missing (")?
                .strip_suffix(')')
                .ok_or("missing )")?
                .split(';')
            {
                let s = s.trim_start();

                if let Some(weak) = s.strip_prefix("weak to ") {
                    g.weaknesses = weak.split(", ").map(|s| s.parse().unwrap()).collect();
                } else if let Some(immune) = s.strip_prefix("immune to ") {
                    g.immunities = immune.split(", ").map(|s| s.parse().unwrap()).collect();
                } else {
                    return Err("bad input".into());
                }
            }
        }

        Ok(g)
    }
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}#{:X} ({} hp {} dmg {} {:?})",
            self.army,
            self.id,
            self.num_units.get(),
            self.hp,
            self.damage,
            self.damage_type
        )
    }
}

impl Group {
    pub fn set_id(&mut self, army: &str, id: u32) {
        self.army = army.to_string();
        self.id = id;
    }

    #[allow(dead_code)]
    pub fn abbrev(&self) -> String {
        if self.army == "Infection" {
            format!("i{}", self.id)
        } else {
            format!("u{}", self.id)
        }
    }

    pub const fn set_boost(&mut self, boost: u32) {
        self.damage += boost;
    }

    pub const fn alive_units(&self) -> u32 {
        self.num_units.get()
    }

    pub const fn is_alive(&self) -> bool {
        self.num_units.get() != 0
    }

    const fn effective_power(&self) -> u32 {
        self.num_units.get() * self.damage
    }

    fn is_immune(&self, attack: &AttackType) -> bool {
        self.immunities.contains(attack)
    }

    fn is_weak(&self, attack: &AttackType) -> bool {
        self.weaknesses.contains(attack)
    }

    pub fn attack_damage(&self, other: &Self) -> u32 {
        if other.is_immune(&self.damage_type) {
            return 0;
        }

        let damage = self.effective_power();
        if other.is_weak(&self.damage_type) {
            damage * 2
        } else {
            damage
        }
    }

    fn take_damage(&self, damage: u32) {
        let units_lost = damage / self.hp;
        let old = self.num_units.get();
        self.num_units.set(old.saturating_sub(units_lost));
    }

    pub fn select_target(&self, others: &[&Self]) -> Option<usize> {
        // filter with
        //  - alive units
        //  - can deal damage
        // sort by
        //  - damage
        //  - effective power
        //  - initiative

        others
            .iter()
            .enumerate()
            .filter(|&(_, other)| other.is_alive() && self.attack_damage(other) != 0)
            .max_by_key(|&(_, other)| {
                (
                    self.attack_damage(other),
                    other.effective_power(),
                    other.initiative,
                )
            })
            .map(|(i, _)| i)
    }
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.effective_power()
            .cmp(&other.effective_power())
            .reverse()
            .then(self.initiative.cmp(&other.initiative).reverse())
    }
}

impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        self.effective_power().eq(&other.effective_power()) && self.initiative.eq(&other.initiative)
    }
}

impl Eq for Group {}

//  _
// |_o _ |__|_
// | |(_|| ||_
//     _|

pub struct Fight<'a> {
    pub attacker: &'a Group,
    pub opponent: &'a Group,
}

impl Fight<'_> {
    pub fn fight(&self) {
        if self.attacker.is_alive() {
            let damage = self.attacker.attack_damage(self.opponent);
            self.opponent.take_damage(damage);
        }
    }
}

impl PartialOrd for Fight<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fight<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.attacker
            .initiative
            .cmp(&other.attacker.initiative)
            .reverse()
    }
}

impl PartialEq for Fight<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.attacker.initiative.eq(&other.attacker.initiative)
    }
}

impl Eq for Fight<'_> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_group() {
        let g1: Group = "17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2".parse().unwrap();
        let g2: Group = "989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3".parse().unwrap();
        let g3: Group = "801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1".parse().unwrap();
        let g4: Group = "4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4".parse().unwrap();

        assert!(g1.is_alive());
        assert!(g2.is_alive());
        assert!(g3.is_alive());
        assert!(g4.is_alive());

        assert_eq!(g1.effective_power(), 17 * 4507);

        assert_eq!(g1.attack_damage(&g2), 0); // g1 deals fire damage but g2 is immune to fire
        assert_eq!(g1.attack_damage(&g3), 4507 * 17); // g3
        assert_eq!(g1.attack_damage(&g4), 2 * 4507 * 17); // g4 is weak to fire, damage are doubled

        let others = [&g2, &g3, &g4];
        let target = g1.select_target(&others);
        assert_eq!(target, Some(2));
    }
}
