//! Tools for moving around a grid.

use std::ops::{Index, IndexMut};

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

/// The four directions
#[derive(PartialEq, Clone, Copy, Eq, Hash, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Returns an iterator over the all four directions, within the limits of the grid.
    pub fn iter(x: u32, y: u32, width: u32, height: u32) -> impl Iterator<Item = (u32, u32, Self)> {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        .filter_map(move |&d| {
            if d == Direction::North && y > 0 {
                Some((x, y - 1, d))
            } else if d == Direction::South && y < height - 1 {
                Some((x, y + 1, d))
            } else if d == Direction::East && x < width - 1 {
                Some((x + 1, y, d))
            } else if d == Direction::West && x > 0 {
                Some((x - 1, y, d))
            } else {
                None
            }
        })
    }

    /// Returns the character used in puzzles of the direction.
    #[must_use]
    pub fn arrow(&self) -> char {
        match &self {
            Direction::North => '^',
            Direction::West => '<',
            Direction::South => 'v',
            Direction::East => '>',
        }
    }
}

impl std::fmt::Display for Direction {
    /// Formats the direction with its usual character.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.arrow())
    }
}

#[macro_export]
macro_rules! grid {
    () => {
        $crate::grid::Grid::new()
    };
}

/// A rectangular grid of elements, used to store various data including mazes.
#[derive(Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    g: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
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

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Grid<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            g: vec![],
        }
    }

    /// Returns a tuple with the dimensions (width, height) of the grid.
    #[must_use]
    #[inline]
    pub fn size(&self) -> (usize, usize) {
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
    pub fn iter_directions_full(
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
    pub fn iter_neighbors(
        &self,
        (x, y): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        NEIGHBORS.iter().filter_map(move |(dx, dy)| {
            let x = isize::try_from(x).unwrap() + *dx;
            let y = isize::try_from(y).unwrap() + *dy;

            if 0 <= x && x < (self.width as isize) && 0 <= y && y < (self.height as isize) {
                Some((x as usize, y as usize))
            } else {
                None
            }
        })
    }
}

impl Grid<u8> {
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

        Grid { width, height, g }
    }
}

impl Grid<char> {
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

        Grid { width, height, g }
    }
}

//?????????
// impl<T: Copy> Grid<T> {
//     /// Returns the content of a cell.
//     #[must_use]
//     #[inline]
//     pub fn cell(&self, xy: (usize, usize)) -> T {
//         let index = self.width * xy.1 + xy.0;
//         self.g[index]
//     }
// }

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.g[self.width * y + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.g[self.width * y + x]
    }
}

impl std::fmt::Display for Grid<char> {
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

impl std::fmt::Display for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.g[self.width * y + x] as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
