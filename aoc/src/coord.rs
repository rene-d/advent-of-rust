//! `Coord` class represents a point, a vector or a direction in 2D.
//!
//! Advent-of-Rust 2024

use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const LEFT: Self = Self { x: -1, y: 0 };
    pub const RIGHT: Self = Self { x: 1, y: 0 };
    pub const UP: Self = Self { x: 0, y: -1 };
    pub const DOWN: Self = Self { x: 0, y: 1 };

    pub const WEST: Self = Self { x: -1, y: 0 };
    pub const EAST: Self = Self { x: 1, y: 0 };
    pub const NORTH: Self = Self { x: 0, y: -1 };
    pub const SOUTH: Self = Self { x: 0, y: 1 };
}

impl Coord {
    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub const fn manhattan_distance(self, rhs: Self) -> i32 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }

    #[inline]
    #[must_use]
    pub const fn clockwise(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    pub const fn counter_clockwise(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    #[inline]
    #[must_use]
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }
}

impl Default for Coord {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Hash for Coord {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.x.to_ne_bytes());
        state.write(&self.y.to_ne_bytes());
    }
}

impl Add for Coord {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Coord {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Coord {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Coord {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<i32> for Coord {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: i32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Coord> for i32 {
    type Output = Coord;
    #[inline]
    fn mul(self, other: Coord) -> Coord {
        other * self
    }
}

impl MulAssign<i32> for Coord {
    #[inline]
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl From<u8> for Coord {
    #[inline]
    #[must_use]
    fn from(value: u8) -> Self {
        match value {
            b'^' | b'U' => Self::UP,
            b'v' | b'D' => Self::DOWN,
            b'<' | b'L' => Self::LEFT,
            b'>' | b'R' => Self::RIGHT,
            _ => unreachable!(),
        }
    }
}

impl From<char> for Coord {
    #[inline]
    #[must_use]
    fn from(value: char) -> Self {
        match value {
            '^' | 'U' => Self::UP,
            'v' | 'D' => Self::DOWN,
            '<' | 'L' => Self::LEFT,
            '>' | 'R' => Self::RIGHT,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let a = Coord::new(1, 2);
        let b = Coord::new(-1, 2);
        let c = a + b;

        assert_eq!(c.x, 0);
        assert_eq!(c.y, 4);
    }

    #[test]
    fn add_assign() {
        let mut a = Coord::new(1, 2);
        let b = Coord::new(-1, 2);

        a += b;

        assert_eq!(a.x, 0);
        assert_eq!(a.y, 4);
    }

    #[test]
    fn mul_coord_i32() {
        let a = Coord::new(1, -2);
        let c = a * 10;

        assert_eq!(c.x, 10);
        assert_eq!(c.y, -20);
    }

    #[test]
    fn mul_i32_coord() {
        let a = Coord::new(1, -2);
        let c = 10 * a;

        assert_eq!(c.x, 10);
        assert_eq!(c.y, -20);
    }

    #[test]
    fn mul_assign() {
        let mut a = Coord::new(1, -2);
        a *= 10;

        assert_eq!(a.x, 10);
        assert_eq!(a.y, -20);
    }

    #[test]
    fn sub() {
        let a = Coord::new(1, 2);
        let b = Coord::new(-1, 2);
        let c = a - b;

        assert_eq!(c.x, 2);
        assert_eq!(c.y, 0);
    }

    #[test]
    fn sub_assign() {
        let mut a = Coord::new(1, 2);
        let b = Coord::new(-1, 2);

        a -= b;

        assert_eq!(a.x, 2);
        assert_eq!(a.y, 0);
    }

    #[test]
    fn clockwise() {
        let mut a = Coord::RIGHT;

        a = a.clockwise();
        assert_eq!(a, Coord::DOWN);

        a = a.clockwise();
        assert_eq!(a, Coord::LEFT);

        a = a.clockwise();
        assert_eq!(a, Coord::UP);

        a = a.clockwise();
        assert_eq!(a, Coord::RIGHT);
    }

    #[test]
    fn counter_clockwise() {
        let mut a = Coord::RIGHT;

        a = a.counter_clockwise();
        assert_eq!(a, Coord::UP);

        a = a.counter_clockwise();
        assert_eq!(a, Coord::LEFT);

        a = a.counter_clockwise();
        assert_eq!(a, Coord::DOWN);

        a = a.counter_clockwise();
        assert_eq!(a, Coord::RIGHT);
    }

    #[test]
    fn from() {
        let a = Coord::from('^');
        assert_eq!(a, Coord::UP);

        let a = Coord::from(b'D');
        assert_eq!(a, Coord::DOWN);
    }
}
