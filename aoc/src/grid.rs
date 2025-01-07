//! `Grid` class, to use with Coord.
//!
//! Advent-of-Rust 2024

use core::str;
use std::ops::{Index, IndexMut};

use crate::Coord;
use crate::Direction;

const NEIGHBORS: [Coord; 8] = [
    Coord::new(0, -1),  // N
    Coord::new(1, -1),  // NE
    Coord::new(1, 0),   // E
    Coord::new(1, 1),   // SE
    Coord::new(0, 1),   // S
    Coord::new(-1, 1),  // SW
    Coord::new(-1, 0),  // W
    Coord::new(-1, -1), // NW
];

#[derive(Debug, Clone)]
pub struct Grid<T> {
    size: Coord,
    data: Vec<T>,
    exterior: T, // default value if out of limits
    dummy: T,    // silently ignore out-of-bounds assignments
}

impl<T: Clone + Default> Grid<T> {
    /// Construct a new, empty `Grid<T>`.
    ///
    /// Dimensions are (0,0) and grid is not really usable.
    #[must_use]
    pub fn new() -> Self {
        Self {
            size: Coord::ZERO,
            data: Vec::new(),
            exterior: T::default(),
            dummy: T::default(),
        }
    }

    /// Construct an grid with the given height and width.
    ///
    /// # Panics
    /// if dimensions are unconsistant (negative or zero).
    #[must_use]
    pub fn with_size(width: i32, height: i32, value: T, exterior: T) -> Self {
        assert!(
            !(!width.is_positive() && !height.is_positive()),
            "Invalid dimensions"
        );

        let len: usize = (width * height).try_into().expect("Out of limits");
        let mut data = Vec::with_capacity(len);
        data.resize(len, value);

        Self {
            size: Coord::new(width, height),
            data,
            exterior,
            dummy: T::default(), // no matter, never read
        }
    }

    /// Set value returned by Index if pos is outside the grid limits
    #[inline]
    pub fn set_exterior(&mut self, exterior: T) {
        self.exterior = exterior;
    }

    /// Return the width of the grid;
    #[inline]
    #[must_use]
    pub const fn width(&self) -> i32 {
        self.size.x
    }

    /// Return the height of the grid;
    #[inline]
    #[must_use]
    pub const fn height(&self) -> i32 {
        self.size.y
    }

    /// Test in a point is in grid
    #[inline]
    #[must_use]
    pub const fn is_in_grid(&self, pos: Coord) -> Option<Coord> {
        if 0 <= pos.x && pos.x < self.size.x && 0 <= pos.y && pos.y < self.size.y {
            Some(pos)
        } else {
            None
        }
    }

    /// Iterate over all cells of the grid
    pub fn iter_cells(&self) -> impl Iterator<Item = (Coord, &T)> {
        (0..).zip(self.data.iter()).map(move |(i, c)| {
            let x = i % self.size.x;
            let y = i / self.size.x;
            (Coord { x, y }, c)
        })
    }

    /// Returns an iterator over the all four directions, within the limits of the grid.
    pub fn iter_directions(&self, xy: Coord) -> impl Iterator<Item = (Direction, Coord)> + '_ {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        .filter_map(move |&d| match d {
            Direction::North if xy.y > 0 => Some((d, xy + Coord::NORTH)),
            Direction::West if xy.x > 0 => Some((d, xy + Coord::WEST)),
            Direction::South if xy.y < self.size.y - 1 => Some((d, xy + Coord::SOUTH)),
            Direction::East if xy.x < self.size.x - 1 => Some((d, xy + Coord::EAST)),
            _ => None,
        })
    }

    /// Returns an iterator over the all four directions.
    pub fn iter_directions_all(
        &self,
        xy: Coord,
    ) -> impl Iterator<Item = (Direction, Option<Coord>)> + '_ {
        [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
        .iter()
        .map(move |&d| {
            let np = xy
                + match d {
                    Direction::North => Coord::NORTH,
                    Direction::South => Coord::SOUTH,
                    Direction::East => Coord::EAST,
                    Direction::West => Coord::WEST,
                };
            (d, self.is_in_grid(np))
        })
    }

    /// Returns an iterator over the all eight neighbors, within the limits of the grid.
    pub fn iter_neighbors(&self, xy: Coord) -> impl Iterator<Item = Coord> + '_ {
        NEIGHBORS
            .iter()
            .filter_map(move |&dxy| self.is_in_grid(xy + dxy))
    }
}

impl<T: Clone + Default> Default for Grid<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;
    #[inline]
    fn index(&self, p: Coord) -> &Self::Output {
        if (0..self.size.x).contains(&p.x) && (0..self.size.y).contains(&p.y) {
            let idx = self.size.x * p.y + p.x;

            // cannot really panic since we have tested before the range
            usize::try_from(idx).map_or(&self.exterior, |idx| &self.data[idx])
        } else {
            &self.exterior
        }
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, p: Coord) -> &mut Self::Output {
        if (0..self.size.x).contains(&p.x) && (0..self.size.y).contains(&p.y) {
            let idx = self.size.x * p.y + p.x;

            usize::try_from(idx).map_or(&mut self.dummy, |idx| &mut self.data[idx])
        } else {
            &mut self.dummy
        }
    }
}

impl<T> Index<(i32, i32)> for Grid<T> {
    type Output = T;
    #[inline]
    fn index(&self, p: (i32, i32)) -> &Self::Output {
        if (0..self.size.x).contains(&p.0) && (0..self.size.y).contains(&p.1) {
            let idx = self.size.x * p.1 + p.0;

            usize::try_from(idx).map_or(&self.exterior, |idx| &self.data[idx])
        } else {
            &self.exterior
        }
    }
}

impl<T> IndexMut<(i32, i32)> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, p: (i32, i32)) -> &mut Self::Output {
        if (0..self.size.x).contains(&p.0) && (0..self.size.y).contains(&p.1) {
            let idx = self.size.x * p.1 + p.0;

            usize::try_from(idx).map_or(&mut self.dummy, |idx| &mut self.data[idx])
        } else {
            &mut self.dummy
        }
    }
}

//
// implement iterators for Grid<T>
//

pub struct IterMut<'a, T> {
    grid_size: usize,
    width: i32,
    inner: std::slice::IterMut<'a, T>,
}

impl<'a, T: Clone + Default> IterMut<'a, T> {
    fn new(inner: &'a mut Grid<T>) -> Self {
        Self {
            grid_size: inner.data.len(),
            width: inner.width(),
            inner: inner.data.iter_mut(),
        }
    }
}

impl<'a, T: Clone + Default> Iterator for IterMut<'a, T> {
    type Item = (Coord, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.inner.next() {
            let idx: i32 = (self.grid_size - self.inner.len() - 1).try_into().unwrap();
            let pos = Coord::new(idx % self.width, idx / self.width);
            Some((pos, c))
        } else {
            None
        }
    }
}

pub struct Iter<'a, T> {
    grid_size: usize,
    width: i32,
    inner: std::slice::Iter<'a, T>,
}

impl<'a, T: Clone + Default> Iter<'a, T> {
    fn new(inner: &'a Grid<T>) -> Self {
        Self {
            grid_size: inner.data.len(),
            width: inner.width(),
            inner: inner.data.iter(),
        }
    }
}

impl<'a, T: Clone + Default> Iterator for Iter<'a, T> {
    type Item = (Coord, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.width <= 0 {
            None
        } else if let Some(c) = self.inner.next() {
            let idx: i32 = (self.grid_size - self.inner.len() - 1).try_into().unwrap();
            let pos = Coord::new(idx % self.width, idx / self.width);
            Some((pos, c))
        } else {
            None
        }
    }
}

impl<T: Clone + Default> Grid<T> {
    #[must_use]
    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut::new(self)
    }
}

impl<'a, T: Clone + Default> IntoIterator for &'a Grid<T> {
    type Item = (Coord, &'a T);
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

impl<'a, T: Clone + Default> IntoIterator for &'a mut Grid<T> {
    type Item = (Coord, &'a mut T);
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        IterMut::new(self)
    }
}

//
// char specializarion
//

impl From<&str> for Grid<char> {
    #[inline]
    #[must_use]
    fn from(value: &str) -> Self {
        Self::parse(value)
    }
}

impl Grid<char> {
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
            exterior: '#',
            dummy: '?',
        }
    }
}

impl std::fmt::Display for Grid<char> {
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

//
// u8 specialization
//

impl From<&str> for Grid<u8> {
    #[inline]
    #[must_use]
    fn from(value: &str) -> Self {
        Self::parse(value)
    }
}

impl Grid<u8> {
    /// Read a grid from a puzzle input.
    /// The grid is guaranteed to be rectangular even if lines are right stripped.
    /// # Panics
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

        Self {
            size: Coord::new(width.try_into().unwrap(), height.try_into().unwrap()),
            data: g,
            exterior: b'#',
            dummy: 0,
        }
    }
}

impl std::fmt::Display for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let idx = self.size.x * y + x;
                let idx: usize = idx.try_into().unwrap();

                write!(f, "{}", self.data[idx] as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

//
// tests
//

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_char() {
        let g = Grid::<char>::from(
            "\
#####
#a..#
#...#
#####
",
        );

        assert_eq!(g.size.x, 5);
        assert_eq!(g.size.y, 4);

        assert_eq!(g[Coord::new(1, 1)], 'a');
    }

    #[test]
    fn parse_u8() {
        let g = Grid::<u8>::from(
            "\
#####
#a..#
#...#
#####
",
        );

        assert_eq!(g.size.x, 5);
        assert_eq!(g.size.y, 4);

        assert_eq!(g[Coord::new(1, 1)], b'a');
    }

    #[test]
    fn set_get() {
        let mut grid = Grid::<char>::with_size(5, 5, ' ', '#');

        grid[(0, 0)] = 'A';
        grid[Coord::new(1, 0)] = 'B';

        assert_eq!(grid[Coord::new(0, 0)], 'A');
        assert_eq!(grid[(1, 0)], 'B');

        assert!(grid.is_in_grid(Coord::new(2, 2)).is_some());
        assert!(grid.is_in_grid(Coord::new(5, 0)).is_none());
        assert!(grid.is_in_grid(Coord::new(0, -1)).is_none());
    }

    #[test]
    fn iter_spec() {
        let grid: Grid<char> = Grid::<char>::with_size(5, 5, ' ', '#');

        assert_eq!(grid.iter_cells().count(), 5 * 5);

        // upper left corner has 2 orthogonal cells: (1,0) (0,1)
        assert_eq!(grid.iter_directions(Coord::ZERO).count(), 2);

        // upper left corner has 3 adjacent cells: (1,0) (1,1) (0,1)
        assert_eq!(grid.iter_neighbors(Coord::ZERO).count(), 3);

        // 812
        // 7 3
        // 654
        let grid = Grid::<char>::with_size(3, 3, ' ', '#');
        let mut square = Grid::<char>::with_size(3, 3, ' ', '#');
        for (c, p) in ('1'..).zip(grid.iter_neighbors(Coord::new(1, 1))) {
            square[p] = c;
        }
        assert_eq!(format!("{square}"), "812\n7 3\n654\n");

        //  N
        // W E
        //  S
        let grid = Grid::<char>::with_size(3, 3, ' ', '#');
        let mut plus = Grid::<char>::with_size(3, 3, ' ', '#');
        for (&c, (_, p)) in ['N', 'E', 'S', 'W']
            .iter()
            .zip(grid.iter_directions(Coord::new(1, 1)))
        {
            plus[p] = c;
        }
        assert_eq!(format!("{plus}"), " N \nW E\n S \n");
    }

    #[test]
    fn iter() {
        let grid = Grid::<char>::from("AB\nCD");
        for (pos, &c) in grid.iter().skip(2) {
            assert_eq!(pos, Coord::new(0, 1)); // enumeration starts at the second line since we skip 2 eleemnts
            assert_eq!(c, 'C');
            break;
        }

        let mut abcd = String::new();
        for (_, &c) in &grid {
            abcd.push(c);
        }
        assert_eq!(abcd, "ABCD");
    }

    #[test]
    fn iter_mut() {
        let mut grid = Grid::<char>::from("AB\nCD");

        for (_, c) in grid.iter_mut() {
            *c = c.to_ascii_lowercase();
        }

        let abcd: String = grid.iter().map(|(_, c)| c).collect();
        assert_eq!(abcd, "abcd");

        for (_, c) in &mut grid {
            *c = c.to_ascii_uppercase();
        }

        let abcd: String = grid.iter().map(|(_, c)| c).collect();
        assert_eq!(abcd, "ABCD");
    }
}
