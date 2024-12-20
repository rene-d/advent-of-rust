//! todo

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub const LEFT: Coord = Coord { x: -1, y: 0 };
    pub const RIGHT: Coord = Coord { x: 1, y: 0 };
    pub const UP: Coord = Coord { x: 0, y: -1 };
    pub const DOWN: Coord = Coord { x: 0, y: 1 };
}

impl Coord {
    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub fn manhattan_distance(&self, rhs: &Coord) -> i32 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }
}

impl Add for Coord {
    type Output = Coord;
    fn add(self, rhs: Coord) -> Coord {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Coord {
    type Output = Coord;
    fn sub(self, rhs: Coord) -> Coord {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<i32> for Coord {
    type Output = Coord;

    fn mul(self, rhs: i32) -> Coord {
        Coord {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Coord> for i32 {
    type Output = Coord;
    fn mul(self, other: Coord) -> Coord {
        other * self
    }
}

impl MulAssign<i32> for Coord {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let a = Coord::new(1, 2);
        let b = Coord::new(-1, 2);
        let c = a + b;

        assert_eq!(c.x, 0);
        assert_eq!(c.y, 4);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Coord::new(1, 2);
        let b = Coord::new(-1, 2);

        a += b;

        assert_eq!(a.x, 0);
        assert_eq!(a.y, 4);
    }

    #[test]
    fn test_mul_coord_i32() {
        let a = Coord::new(1, -2);
        let c = a * 10;

        assert_eq!(c.x, 10);
        assert_eq!(c.y, -20);
    }

    #[test]
    fn test_mul_i32_coord() {
        let a = Coord::new(1, -2);
        let c = 10 * a;

        assert_eq!(c.x, 10);
        assert_eq!(c.y, -20);
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Coord::new(1, -2);
        a *= 10;

        assert_eq!(a.x, 10);
        assert_eq!(a.y, -20);
    }

    #[test]
    fn test_sub() {
        let a = Coord::new(1, 2);
        let b = Coord::new(-1, 2);
        let c = a - b;

        assert_eq!(c.x, 2);
        assert_eq!(c.y, 0);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Coord::new(1, 2);
        let b = Coord::new(-1, 2);

        a -= b;

        assert_eq!(a.x, 2);
        assert_eq!(a.y, 0);
    }
}
