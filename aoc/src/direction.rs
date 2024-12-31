/// The four directions
#[derive(PartialEq, Clone, Copy, Eq, Hash, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Returns the character used in puzzles of the direction.
    #[must_use]
    pub const fn arrow(self) -> char {
        match &self {
            Self::North => '^',
            Self::West => '<',
            Self::South => 'v',
            Self::East => '>',
        }
    }
}

impl std::fmt::Display for Direction {
    /// Formats the direction with its usual character.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.arrow())
    }
}
