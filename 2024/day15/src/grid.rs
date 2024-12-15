//! todo

use core::str;
use std::ops::{Index, IndexMut};

pub use crate::coord::Coord;

pub struct Grid {
    size: Coord,
    data: Vec<char>,
    dummy: char, // to silently ignore out of limits assignments
}

impl Grid {
    /// # Panics
    /// if dimensions are unconsistant
    #[must_use]
    pub fn new(width: i32, height: i32) -> Self {
        assert!(
            !(!width.is_positive() && !height.is_positive()),
            "Invalid dimensions"
        );

        let len: usize = (width * height).try_into().unwrap();
        let mut data = Vec::with_capacity(len);
        data.resize(len, ' ');
        Self {
            size: Coord::new(width, height),
            data,
            dummy: '.',
        }
    }

    #[must_use]
    pub fn width(&self) -> i32 {
        self.size.x
    }

    #[must_use]
    pub fn height(&self) -> i32 {
        self.size.y
    }

    /// # Panics
    #[must_use]
    pub fn parse(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();

        let width = lines.iter().map(|row| row.len()).max().unwrap();
        let height = lines.len();

        let mut g: Vec<char> = Vec::with_capacity(width * height);

        for row in &lines {
            g.extend(row.chars());
            g.extend((row.len()..width).map(|_| ' '));
        }

        Self {
            size: Coord::new(width.try_into().unwrap(), height.try_into().unwrap()),
            data: g,
            dummy: '.',
        }
    }

    /// # Panics
    pub fn iter(&self) -> impl Iterator<Item = (Coord, &char)> {
        self.data.iter().enumerate().map(move |(i, c)| {
            let i: i32 = i.try_into().unwrap();
            let x = i % self.size.x;
            let y = i / self.size.x;
            (Coord { x, y }, c)
        })
    }
}

/// # Panics
impl Index<Coord> for Grid {
    type Output = char;
    #[inline]
    fn index(&self, p: Coord) -> &Self::Output {
        if (0..self.size.x).contains(&p.x) && (0..self.size.y).contains(&p.y) {
            let idx = self.size.x * p.y + p.x;
            let idx: usize = idx.try_into().unwrap();

            &self.data[idx]
        } else {
            &'#'
        }
    }
}

impl IndexMut<Coord> for Grid {
    #[inline]
    fn index_mut(&mut self, p: Coord) -> &mut Self::Output {
        if (0..self.size.x).contains(&p.x) && (0..self.size.y).contains(&p.y) {
            let idx = self.size.x * p.y + p.x;
            let idx: usize = idx.try_into().unwrap();

            &mut self.data[idx]
        } else {
            &mut self.dummy
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let idx = self.size.x * y + x;
                let idx: usize = idx.try_into().unwrap();

                write!(f, "{}", self.data[idx])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
