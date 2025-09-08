use std::{
    hash::{Hash, Hasher},
    ops::{Index, IndexMut},
};

/// A square grid of elements `T`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Square<T> {
    cells: Vec<T>,
    n: usize,
    dummy: T,
}

impl<T: Clone + Copy + Default> Square<T> {
    /// Create a new empty square.
    #[must_use]
    pub fn new(n: usize) -> Self {
        Self {
            cells: vec![T::default(); n * n],
            n,
            dummy: T::default(),
        }
    }

    /// Return the size of the square (not the number of cells).
    #[inline]
    #[must_use]
    pub const fn size(&self) -> usize {
        self.n
    }

    /// Clockwise rotation
    #[inline]
    pub fn rotate_inplace(&mut self) {
        let orig = self.clone();
        for x in 0..self.n {
            for y in 0..self.n {
                self[(x, y)] = orig[(y, self.n - 1 - x)];
            }
        }
    }

    /// Symmetry about the Y axis
    #[inline]
    pub fn flip_vertical_inplace(&mut self) {
        let n = self.n;
        for y in 0..n {
            for x in 0..(n / 2) {
                let tmp = self[(x, y)];
                self[(x, y)] = self[(n - 1 - x, y)];
                self[(n - 1 - x, y)] = tmp;
            }
        }
    }

    /// Symmetry about the X axis
    #[inline]
    pub fn flip_horizontal_inplace(&mut self) {
        let n = self.n;
        for x in 0..n {
            for y in 0..(n / 2) {
                let tmp = self[(x, y)];
                self[(x, y)] = self[(x, n - 1 - y)];
                self[(x, n - 1 - y)] = tmp;
            }
        }
    }

    /// Clockwise rotation
    #[inline]
    pub fn rotate(&self) -> Self {
        let mut rotated = Self::new(self.n);
        for x in 0..self.n {
            for y in 0..self.n {
                rotated[(x, y)] = self[(y, self.n - 1 - x)];
            }
        }
        rotated
    }

    /// Symmetry about the Y axis
    #[inline]
    pub fn flip_vertical(&self) -> Self {
        let mut flipped = Self::new(self.n);
        let n = self.n;
        for y in 0..n {
            for x in 0..n {
                flipped[(x, y)] = self[(n - 1 - x, y)];
            }
        }
        flipped
    }

    /// Symmetry about the X axis
    #[inline]
    pub fn flip_horizontal(&self) -> Self {
        let mut flipped = Self::new(self.n);
        let n = self.n;
        for y in 0..n {
            for x in 0..n {
                flipped[(x, y)] = self[(x, n - 1 - y)];
            }
        }
        flipped
    }

    /// Return the cells as a flattened slice `[row1, row2, ..., rowN]`
    /// with `rowI = col1, col2, ..., colN`.
    #[inline]
    pub fn values(&self) -> &[T] {
        &self.cells
    }

    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.cells.iter().enumerate().map(move |(i, c)| {
            let x = i % self.n;
            let y = i / self.n;
            ((x, y), c)
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> {
        self.cells.iter_mut().enumerate().map(|(i, c)| {
            let x = i % self.n;
            let y = i / self.n;
            ((x, y), c)
        })
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
        (0..self.n).map(|i| &self.cells[(i * self.n)..((i + 1) * self.n)])
    }

    /// Iterate over the 8 possible images of the square:
    /// 4 rotations of the original square and 4 rotations of the symmetric square.
    /// Return successively:
    /// - the original square
    /// - the square rotated clockwise
    /// - the square rotated clockwise x2
    /// - the square rotated clockwise x3
    /// - the original square vertically flipped (image in a vertical mirror)
    /// - the image rotated clockwise
    /// - the image rotated clockwise x2
    /// - the image rotated clockwise x3
    pub fn iter_pos(&self) -> impl Iterator<Item = Self> + '_ {
        let mut square = self.clone();
        (0..8).map(move |i| {
            match i {
                0 => {}
                1..=3 | 5..8 => square.rotate_inplace(),
                4 => {
                    square.rotate_inplace();
                    square.flip_vertical_inplace();
                }
                _ => panic!(),
            }
            square.clone()
        })
    }

    /// Extract a subsquare from a bigger one.
    #[inline]
    pub fn get_square(&self, offset_x: usize, offset_y: usize, n: usize) -> Self {
        let mut subsquare = Self::new(n);
        for y in 0..n {
            for x in 0..n {
                if offset_x + x < self.n && offset_y + y < self.n {
                    subsquare[(x, y)] = self[(offset_x + x, offset_y + y)];
                }
            }
        }
        subsquare
    }

    #[inline]
    pub fn put_square(&mut self, offset_x: usize, offset_y: usize, subsquare: &Self) {
        for y in 0..subsquare.n {
            for x in 0..subsquare.n {
                if offset_x + x < self.n && offset_y + y < self.n {
                    self[(offset_x + x, offset_y + y)] = subsquare[(x, y)];
                }
            }
        }
    }
}

impl<T: Hash> Hash for Square<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&*self.cells, state);
    }
}

/// Get the element (x,y) of a square.
impl<T> Index<(usize, usize)> for Square<T> {
    type Output = T;
    #[inline]
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        if x < self.n && y < self.n {
            &self.cells[self.n * y + x]
        } else {
            &self.dummy
        }
    }
}

/// Set the element (x,y) of a square.
impl<T> IndexMut<(usize, usize)> for Square<T> {
    #[inline]
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        if x < self.n && y < self.n {
            &mut self.cells[self.n * y + x]
        } else {
            &mut self.dummy
        }
    }
}

// impl<T: Display> std::fmt::Display for Square<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for y in 0..self.n {
//             for x in 0..self.n {
//                 let idx = self.n * y + x;
//                 write!(f, "{}", self.cells[idx])?;
//             }
//             writeln!(f)?;
//         }
//         Ok(())
//     }
// }

impl Square<u8> {
    pub fn parse(pattern: &str, sep: char) -> Self {
        let rows: Vec<_> = pattern.split(sep).collect();

        let n = rows.len();
        if n == rows.iter().map(|s| s.len()).min().unwrap() {
            let mut square = Self::new(n);
            for (y, row) in rows.iter().enumerate() {
                for (x, c) in row.chars().enumerate() {
                    square[(x, y)] = (c as u32).to_ne_bytes()[0];
                }
            }
            square
        } else {
            Self::new(0)
        }
    }
}

impl std::fmt::Display for Square<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.n {
            for x in 0..self.n {
                let idx = self.n * y + x;
                write!(f, "{}", char::from_u32(u32::from(self.cells[idx])).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const R: &str = "\
###### |\
#     #|\
#     #|\
###### |\
#   #  |\
#    # |\
#     #";

    #[test]
    fn square_iter() {
        let square = Square::<u8>::parse(R, '|');

        println!("orig:");
        print!("{square}");

        for (i, s) in square.iter_pos().enumerate() {
            println!("iter: {i}");
            println!("{s}");
        }
    }

    #[test]
    fn square_flip_vertical() {
        let mut square = Square::<u8>::parse(R, '|');

        println!("orig:");
        println!("{square}");

        square = square.flip_vertical();
        println!("flip_vertical");
        println!("{square}");

        square.flip_vertical_inplace();
        println!("flip_vertical_inplace");
        println!("{square}");
    }

    #[test]
    fn square_flip_horizontal() {
        let mut square = Square::<u8>::parse(R, '|');

        println!("orig:");
        println!("{square}");

        square = square.flip_horizontal();
        println!("flip_horizontal");
        println!("{square}");

        square.flip_horizontal_inplace();
        println!("flip_horizontal_inplace");
        println!("{square}");
    }

    #[test]
    fn square_rotate() {
        let mut square = Square::<u8>::parse(R, '|');

        println!("orig:");
        println!("{square}");

        square = square.rotate();
        println!("rotate");
        println!("{square}");

        square.rotate_inplace();
        println!("rotate");
        println!("{square}");
    }
}
