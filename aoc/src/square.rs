use std::{
    hash::{Hash, Hasher},
    ops::{Index, IndexMut},
};

#[derive(Clone, PartialEq, Eq)]
pub struct Square<T> {
    cells: Vec<T>,
    n: usize,
    dummy: T,
}

impl<T: Clone + Copy + Default> Square<T> {
    #[must_use]
    pub fn new(n: usize) -> Self {
        Self {
            cells: vec![T::default(); n * n],
            n,
            dummy: T::default(),
        }
    }

    #[inline]
    #[must_use]
    pub const fn size(&self) -> usize {
        self.n
    }

    #[inline]
    pub fn rotate_inplace(&mut self) {
        let orig = self.clone();
        for x in 0..self.n {
            for y in 0..self.n {
                self[(y, x)] = orig[(self.n - 1 - x, y)];
            }
        }
    }

    #[inline]
    pub fn flip_vertical_inplace(&mut self) {
        let n = self.n;
        for y in 0..n {
            for x in 0..(n / 2) {
                let tmp = self[(y, x)];
                self[(y, x)] = self[(y, n - 1 - x)];
                self[(y, n - 1 - x)] = tmp;
            }
        }
    }

    #[inline]
    pub fn rotate(&self) -> Self {
        let mut rotated = Self::new(self.n);
        for x in 0..self.n {
            for y in 0..self.n {
                rotated[(y, x)] = self[(self.n - 1 - x, y)];
            }
        }
        rotated
    }

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

    #[inline]
    pub fn values(&self) -> &[T] {
        &self.cells
    }

    /// Iterate over the 8 possible images of the square:
    /// 4 rotations of the original square and 4 rotations of the symmetric square.
    pub fn iter_pos(&self) -> impl Iterator<Item = Self> + '_ {
        let mut square = self.clone();
        (0..9).map(move |i| {
            match i {
                0..=3 | 5..=8 => square.rotate_inplace(),
                4 => square.flip_vertical_inplace(),
                _ => panic!(),
            };
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
    pub fn parse(pattern: &str) -> Self {
        let rows: Vec<_> = pattern.split('/').collect();

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

    #[test]
    fn square_iter() {
        let square = Square::<u8>::parse("##./.../...");
        print!("{square}");
        for s in square.iter_pos() {
            println!("next:");
            println!("{s}");
        }
    }
    #[test]
    fn square_flip() {
        let mut square = Square::<u8>::parse(".#./..#/###");
        println!("{square}");
        square = square.flip_vertical();
        println!("{square}");
        square.flip_vertical_inplace();
        println!("{square}");
    }
}
