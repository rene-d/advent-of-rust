use std::ops::AddAssign;

use crate::group::Fight;
use crate::group::Group;

#[derive(Debug, Clone)]
pub struct Army {
    name: String,
    groups: Vec<Group>,
}

impl Army {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            name: String::new(),
            groups: vec![],
        }
    }
}

impl Default for Army {
    fn default() -> Self {
        Self::new()
    }
}

impl std::str::FromStr for Army {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut a = Self::new();
        let mut id = 0;

        for line in s.lines() {
            if let Some(line) = line.strip_suffix(':') {
                a.name = line.to_string();
            } else {
                let mut g: Group = line.parse()?;
                id.add_assign(1);
                g.set_id(&a.name, id);
                a.groups.push(g);
            }
        }

        Ok(a)
    }
}

impl Army {
    pub fn set_boost(&mut self, boost: u32) {
        for group in &mut self.groups {
            group.set_boost(boost);
        }
    }

    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.groups.iter().any(Group::is_alive)
    }

    #[must_use]
    pub fn alive_units(&self) -> u32 {
        self.groups.iter().map(Group::alive_units).sum()
    }

    #[must_use]
    pub fn select_fights<'a>(&'a self, other: &'a Self) -> Vec<Fight<'a>> {
        let mut fights: Vec<Fight<'a>> = vec![];

        let mut attackers = self.alive_groups();
        let mut targets = other.alive_groups();

        // sort by effective power then initiative
        attackers.sort_unstable();

        for attacker in &attackers {
            if let Some(i) = attacker.select_target(&targets) {
                fights.push(Fight {
                    attacker,
                    opponent: targets[i],
                });

                targets.remove(i);
            }
        }

        fights
    }

    #[must_use]
    pub fn alive_groups(&self) -> Vec<&Group> {
        self.groups.iter().filter(|g| g.is_alive()).collect()
    }
}

impl std::fmt::Display for Army {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for g in &self.groups {
            writeln!(f, "{g}")?;
        }
        Ok(())
    }
}
