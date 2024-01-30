use crate::army::Army;

const MAX_FIGHTS: usize = 10000;

#[derive(Clone)]
pub struct Combat {
    army1: Army,
    army2: Army,
}

impl Default for Combat {
    fn default() -> Self {
        Self::new()
    }
}

impl Combat {
    #[must_use]
    pub fn new() -> Self {
        Self {
            army1: Army::new(),
            army2: Army::new(),
        }
    }

    #[must_use]
    pub fn with_armies(army1: Army, army2: Army) -> Self {
        Self { army1, army2 }
    }

    pub fn set_army1_boost(&mut self, boost: u32) {
        self.army1.set_boost(boost);
    }

    #[must_use]
    pub fn immune_alive_units(&self) -> u32 {
        self.army1.alive_units()
    }

    #[must_use]
    pub fn infection_alive_units(&self) -> u32 {
        self.army2.alive_units()
    }

    #[must_use]
    pub fn fight_to_death(&self) -> u32 {
        for _ in 0..MAX_FIGHTS {
            if let Some(winner) = self.fight() {
                return winner.alive_units();
            }
        }
        0
    }

    /// # Panics
    /// si both armies are alive
    #[must_use]
    pub fn fight(&self) -> Option<&Army> {
        let mut fights = self.army1.select_fights(&self.army2);
        fights.extend(self.army2.select_fights(&self.army1));

        // sort by initiative, greater first
        fights.sort_unstable();

        for f in &fights {
            f.fight();
        }

        assert!(self.army1.is_alive() || self.army2.is_alive());

        if !self.army1.is_alive() {
            Some(&self.army2)
        } else if !self.army2.is_alive() {
            Some(&self.army1)
        } else {
            None
        }
    }
}

impl std::fmt::Display for Combat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.army1)?;
        write!(f, "{}", self.army2)
    }
}
