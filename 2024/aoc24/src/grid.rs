//! Grid class, to use with Coord.
//!
//! Advent-of-Rust 2024

use core::str;
use std::ops::{Index, IndexMut};

pub use crate::coord::Coord;

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

#[derive(Debug)]
pub struct Grid {
    size: Coord,
    data: Vec<char>,
    exterior: char, // default value if out of limits
    dummy: char,    // silently ignore out-of-bounds assignments
}

impl Grid {
    /// Construct a new, empty Grid.
    ///
    /// Dimensions are (0,0) and grid is not really usable.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            size: Coord::ZERO,
            data: Vec::new(),
            exterior: '#',
            dummy: '?',
        }
    }

    /// Construct an grid with the given height and width.
    ///
    /// # Panics
    /// if dimensions are unconsistant (negative of zero).
    #[must_use]
    pub fn with_size(width: i32, height: i32) -> Self {
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
            exterior: '#',
            dummy: '?', // no matter, never read
        }
    }

    /// Set the value returned for out-of-limits coords.
    #[inline]
    pub const fn set_exterior(&mut self, exterior: char) {
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
    pub const fn is_in_grid(&self, pos: Coord) -> bool {
        0 <= pos.x && pos.x < self.size.x && 0 <= pos.y && pos.y < self.size.y
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
            exterior: '#',
            dummy: '?',
        }
    }

    /// Iterate over all cells of the grid
    pub fn iter_cells(&self) -> impl Iterator<Item = (Coord, &char)> {
        (0..).zip(self.data.iter()).map(move |(i, c)| {
            let x = i % self.size.x;
            let y = i / self.size.x;
            (Coord { x, y }, c)
        })
    }

    /// Returns an iterator over the all four directions, within the limits of the grid.
    pub fn iter_directions(&self, xy: Coord) -> impl Iterator<Item = Coord> + '_ {
        [Coord::UP, Coord::RIGHT, Coord::DOWN, Coord::LEFT]
            .iter()
            .filter_map(move |&d| match d {
                Coord::UP if xy.y > 0 => Some(xy + d),
                Coord::DOWN if xy.y < self.size.y - 1 => Some(xy + d),
                Coord::RIGHT if xy.x < self.size.x - 1 => Some(xy + d),
                Coord::LEFT if xy.x > 0 => Some(xy + d),
                _ => None,
            })
    }

    /// Returns an iterator over the all four directions, within the limits of the grid.
    pub fn iter_directions_full(&self, xy: Coord) -> impl Iterator<Item = (Coord, Coord)> + '_ {
        [Coord::UP, Coord::RIGHT, Coord::DOWN, Coord::LEFT]
            .iter()
            .filter_map(move |&d| match d {
                Coord::UP if xy.y > 0 => Some((d, xy + d)),
                Coord::DOWN if xy.y < self.size.y - 1 => Some((d, xy + d)),
                Coord::RIGHT if xy.x < self.size.x - 1 => Some((d, xy + d)),
                Coord::LEFT if xy.x > 0 => Some((d, xy + d)),
                _ => None,
            })
    }

    /// Returns an iterator over the all eight neighbors, within the limits of the grid.
    pub fn iter_neighbors(&self, xy: Coord) -> impl Iterator<Item = Coord> + '_ {
        NEIGHBORS.iter().filter_map(move |&dxy| {
            let nxy = xy + dxy;
            if self.is_in_grid(nxy) {
                Some(nxy)
            } else {
                None
            }
        })
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<Coord> for Grid {
    type Output = char;
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

impl IndexMut<Coord> for Grid {
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

impl Index<(i32, i32)> for Grid {
    type Output = char;
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

impl IndexMut<(i32, i32)> for Grid {
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

impl From<&str> for Grid {
    #[inline]
    #[must_use]
    fn from(value: &str) -> Self {
        Self::parse(value)
    }
}

//
// implement iterators for Grid
//

pub struct IterMut<'a> {
    grid_size: usize,
    width: i32,
    inner: std::slice::IterMut<'a, char>,
}

impl<'a> IterMut<'a> {
    fn new(inner: &'a mut Grid) -> Self {
        Self {
            grid_size: inner.data.len(),
            width: inner.width(),
            inner: inner.data.iter_mut(),
        }
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = (Coord, &'a mut char);

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

pub struct Iter<'a> {
    grid_size: usize,
    width: i32,
    inner: std::slice::Iter<'a, char>,
}

impl<'a> Iter<'a> {
    fn new(inner: &'a Grid) -> Self {
        Self {
            grid_size: inner.data.len(),
            width: inner.width(),
            inner: inner.data.iter(),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (Coord, &'a char);

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

impl Grid {
    #[must_use]
    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    pub fn iter_mut(&mut self) -> IterMut {
        IterMut::new(self)
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = (Coord, &'a char);
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

impl<'a> IntoIterator for &'a mut Grid {
    type Item = (Coord, &'a mut char);
    type IntoIter = IterMut<'a>;
    fn into_iter(self) -> Self::IntoIter {
        IterMut::new(self)
    }
}

//
// tests
//

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let g = Grid::from(
            "\
#####
#...#
#...#
#####
",
        );

        assert_eq!(g.size.x, 5);
        assert_eq!(g.size.y, 4);
    }

    #[test]
    fn set_get() {
        let mut g = Grid::with_size(5, 5);

        g[(0, 0)] = 'A';
        g[Coord::new(1, 0)] = 'B';

        assert_eq!(g[Coord::new(0, 0)], 'A');
        assert_eq!(g[(1, 0)], 'B');

        assert!(g.is_in_grid(Coord::new(2, 2)));
        assert!(!g.is_in_grid(Coord::new(5, 0)));
        assert!(!g.is_in_grid(Coord::new(0, -1)));
    }

    #[test]
    fn iter_spec() {
        let g: Grid = Grid::with_size(5, 5);

        assert_eq!(g.iter_cells().count(), 5 * 5);

        // upper left corner has 2 orthogonal cells: (1,0) (0,1)
        assert_eq!(g.iter_directions(Coord::ZERO).count(), 2);

        // upper left corner has 3 adjacent cells: (1,0) (1,1) (0,1)
        assert_eq!(g.iter_neighbors(Coord::ZERO).count(), 3);

        // 812
        // 7 3
        // 654
        let g: Grid = Grid::with_size(3, 3);
        let mut square: Grid = Grid::with_size(3, 3);
        for (c, p) in ('1'..).zip(g.iter_neighbors(Coord::new(1, 1))) {
            square[p] = c;
        }
        assert_eq!(format!("{square}"), "812\n7 3\n654\n");

        //  N
        // W E
        //  S
        let g: Grid = Grid::with_size(3, 3);
        let mut plus: Grid = Grid::with_size(3, 3);
        for (&c, p) in ['N', 'E', 'S', 'W']
            .iter()
            .zip(g.iter_directions(Coord::new(1, 1)))
        {
            plus[p] = c;
        }
        assert_eq!(format!("{plus}"), " N \nW E\n S \n");
    }

    #[test]
    fn iter() {
        let g: Grid = Grid::from("AB\nCD");
        for (pos, &c) in g.iter().skip(2) {
            assert_eq!(pos, Coord::new(0, 1)); // enumeration starts at the second line since we skip 2 eleemnts
            assert_eq!(c, 'C');
            break;
        }

        let mut abcd = String::new();
        for (_, &c) in &g {
            abcd.push(c);
        }
        assert_eq!(abcd, "ABCD");
    }

    #[test]
    fn iter_mut() {
        let mut g: Grid = Grid::from("AB\nCD");

        for (_, c) in g.iter_mut() {
            *c = c.to_ascii_lowercase();
        }

        let abcd: String = g.iter().map(|(_, c)| c).collect();
        assert_eq!(abcd, "abcd");

        for (_, c) in &mut g {
            *c = c.to_ascii_uppercase();
        }

        let abcd: String = g.iter().map(|(_, c)| c).collect();
        assert_eq!(abcd, "ABCD");
    }
}
