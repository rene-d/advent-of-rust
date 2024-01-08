use std::ops::{Index, IndexMut};

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
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
}

#[macro_export]
macro_rules! grid {
    () => {
        $crate::grid::Grid::new()
    };
}

pub struct Grid<T> {
    width: usize,
    height: usize,
    g: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    #[must_use]
    pub fn with_size(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            g: vec![T::default(); width * height],
        }
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

    #[inline]
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.g.iter().enumerate().map(move |(i, c)| {
            let x = i % self.width;
            let y = i / self.width;
            (x, y, c)
        })
    }

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
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let raw: Vec<_> = input.lines().map(str::as_bytes).collect();

        let width = raw[0].len();
        let height = raw.len();

        let mut g: Vec<u8> = Vec::with_capacity(width * height);

        raw.iter().for_each(|row| g.extend_from_slice(row));

        Grid { width, height, g }
    }
}

impl Grid<char> {
    #[must_use]
    pub fn parse(input: &str) -> Self {
        let raw: Vec<_> = input.lines().collect();

        let width = raw[0].len();
        let height = raw.len();

        let mut g: Vec<char> = Vec::with_capacity(width * height);

        raw.iter().for_each(|&row| g.extend(row.chars()));

        Grid { width, height, g }
    }
}

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
