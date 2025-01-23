//! Tools for moving around a grid.
//!
//! `GridU` coordinates are a tuple of `usize` integers.
//!
//! Advent-of-Rust 2022-2023

use std::ops::{Index, IndexMut};

use crate::Direction;

/// A rectangular grid of elements, used to store various data including mazes.
#[derive(Debug, Clone)]
pub struct GridU<T> {
    width: usize,
    height: usize,
    g: Vec<T>,
}

impl<T: Clone + Default> GridU<T> {
    /// Returns a grid with the given dimensions and the default value for each cell.
    #[must_use]
    pub fn with_size(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            g: vec![T::default(); width * height],
        }
    }

    /// Resize the grid in-place.
    /// If the grid is extended, new cells are filled with the default value of T.
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.g.resize(width * height, T::default());
    }
}

impl<T: Copy + Default> GridU<T> {
    pub fn rotate_clockwise(&self) -> Self {
        let mut rotated = Self::with_size(self.height, self.width);

        for x in 0..self.width {
            for y in 0..self.height {
                rotated[(self.height - y - 1, x)] = self[(x, y)];
            }
        }
        rotated
    }

    pub fn rotate_counterclockwise(&self) -> Self {
        let mut rotated = Self::with_size(self.height, self.width);

        for x in 0..self.width {
            for y in 0..self.height {
                rotated[(y, self.width - 1 - x)] = self[(x, y)];
            }
        }
        rotated
    }

    pub fn flip_vertical(&self) -> Self {
        let mut flipped = Self::with_size(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                flipped[(x, y)] = self[(self.width - 1 - x, y)];
            }
        }
        flipped
    }

    pub fn flip_horizontal(&self) -> Self {
        let mut flipped = Self::with_size(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                flipped[(x, y)] = self[(x, self.height - 1 - y)];
            }
        }
        flipped
    }

    /// Iterate over the 8 possible images of the grid:
    /// 4 rotations of the original grid and 4 rotations of the symmetric grid.
    /// Return successively:
    /// - the original grid
    /// - the grid rotated clockwise
    /// - the grid rotated clockwise x2
    /// - the grid rotated clockwise x3
    /// - the original grid vertically flipped (image in a vertical mirror)
    /// - the image of the grid rotated clockwise
    /// - the image of the grid rotated clockwise x2
    /// - the image of the grid rotated clockwise x3
    pub fn iter_pos(&self) -> impl Iterator<Item = Self> {
        let mut square = self.clone();
        (0..8).map(move |i| {
            if i >= 1 {
                square = square.rotate_clockwise();
            }
            if i == 4 {
                square = square.flip_vertical();
            }

            square.clone()
        })
    }
}

impl<T> Default for GridU<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> GridU<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            g: Vec::new(),
        }
    }

    /// Returns a tuple with the dimensions (width, height) of the grid.
    #[must_use]
    #[inline]
    pub const fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.g.iter().enumerate().map(move |(i, c)| {
            let x = i % self.width;
            let y = i / self.width;
            ((x, y), c)
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> {
        self.g.iter_mut().enumerate().map(|(i, c)| {
            let x = i % self.width;
            let y = i / self.width;
            ((x, y), c)
        })
    }

    /// Returns an iterator over the all four directions, within the limits of the grid.
    pub fn iter_directions(
        &self,
        (x, y): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        .filter_map(move |d| {
            if d == &Direction::North && y > 0 {
                Some((x, y - 1))
            } else if d == &Direction::South && y < self.height - 1 {
                Some((x, y + 1))
            } else if d == &Direction::East && x < self.width - 1 {
                Some((x + 1, y))
            } else if d == &Direction::West && x > 0 {
                Some((x - 1, y))
            } else {
                None
            }
        })
    }

    /// Returns an iterator over the all four directions, within the limits of the grid.
    pub fn iter_directions_all(
        &self,
        (x, y): (usize, usize),
    ) -> impl Iterator<Item = (Direction, Option<(usize, usize)>)> + '_ {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        .map(move |&d| {
            if d == Direction::North && y > 0 {
                (d, Some((x, y - 1)))
            } else if d == Direction::East && x < self.width - 1 {
                (d, Some((x + 1, y)))
            } else if d == Direction::South && y < self.height - 1 {
                (d, Some((x, y + 1)))
            } else if d == Direction::West && x > 0 {
                (d, Some((x - 1, y)))
            } else {
                (d, None)
            }
        })
    }

    /// Returns an iterator over the all eight neighbors, within the limits of the grid.
    /// # Panics
    pub fn iter_neighbors(
        &self,
        (x, y): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .filter_map(move |&(dx, dy)| {
            if let Some(x) = x.checked_add_signed(dx) {
                if let Some(y) = y.checked_add_signed(dy) {
                    if x < self.width && y < self.height {
                        return Some((x, y));
                    }
                }
            }

            None
        })
    }
}

impl GridU<u8> {
    /// Read a grid from a puzzle input.
    /// The grid is guaranteed to be rectangular even if lines are right stripped.
    pub fn parse(input: &str) -> Self {
        let lines: Vec<_> = input.lines().map(str::as_bytes).collect();

        let width = lines[0].len();
        let height = lines.len();

        let mut g: Vec<u8> = Vec::with_capacity(width * height);

        // lines.iter().for_each(|row| g.extend_from_slice(row));
        for row in &lines {
            g.extend_from_slice(row);
            g.extend((row.len()..width).map(|_| b' '));
        }

        Self { width, height, g }
    }
}

impl GridU<char> {
    /// Read a grid from a puzzle input.
    /// The grid is guaranteed to be rectangular even if lines are right stripped.
    /// # Panics
    /// If input is empty.
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

        Self { width, height, g }
    }
}

impl<T> Index<(usize, usize)> for GridU<T> {
    type Output = T;

    #[inline]
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.g[self.width * y + x]
    }
}

impl<T> IndexMut<(usize, usize)> for GridU<T> {
    #[inline]
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.g[self.width * y + x]
    }
}

impl std::fmt::Display for GridU<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.g[self.width * y + x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for GridU<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.g[self.width * y + x];
                write!(f, "{}", c as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
