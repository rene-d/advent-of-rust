use std::string::ParseError;

/// Kind of attacks. Units can deal damage with these attacks, can be immune or weak.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum AttackType {
    Fire,
    Radiation,
    Slashing,
    Bludgeoning,
    Cold,
}

impl std::str::FromStr for AttackType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "fire" => AttackType::Fire,
            "radiation" => AttackType::Radiation,
            "slashing" => AttackType::Slashing,
            "bludgeoning" => AttackType::Bludgeoning,
            "cold" => AttackType::Cold,
            _ => panic!(),
        })
    }
}
