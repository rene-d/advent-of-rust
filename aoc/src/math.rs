use crate::integer::{Integer, Signed, Unsigned};

pub trait IntegerMathOps<T: Integer<T>> {
    /// Greatest common divisor.
    fn gcd(self, b: T) -> T;

    /// Least common multiple.
    fn lcm(self, b: T) -> T;

    /// Calculates `(n^x) % p`.
    fn mod_exp(self, x: T, p: T) -> T;
}

pub trait SignedMathOps<T: Signed<T>> {
    /// Extended Euclidean algorithm.
    fn egcd(self, b: T) -> (T, T, T);

    /// Modular multiplicate inverse.
    fn mod_inv(self, m: T) -> Option<T>;
}

pub trait UnsignedMathOps<T: Unsigned<T>> {
    /// Integer square root (using binary search).
    fn sqrt(self) -> T;
}

impl<T: Integer<T>> IntegerMathOps<T> for T {
    #[inline]
    fn gcd(self, mut b: T) -> T {
        let mut a = self;

        while b != T::ZERO {
            (a, b) = (b, a.rem_euclid(b));
        }
        a
    }

    #[inline]
    fn lcm(self, b: T) -> T {
        self * (b / self.gcd(b))
    }

    fn mod_exp(self: T, mut x: T, p: T) -> T {
        let mut n = self;
        let mut ans = T::ONE;
        if x <= T::ZERO {
            return T::ONE;
        }
        loop {
            if x == T::ONE {
                return (ans * n) % p;
            }
            if x & T::ONE == T::ZERO {
                n = (n * n) % p;
                x >>= T::ONE;
            } else {
                ans = (ans * n) % p;
                x -= T::ONE;
            }
        }
    }
}

impl<T: Signed<T>> SignedMathOps<T> for T {
    fn egcd(self, b: T) -> (T, T, T) {
        let a = self;

        let (mut old_r, mut r) = (a, b);
        let (mut old_s, mut s) = (T::ONE, T::ZERO);

        while r != T::ZERO {
            let quotient = old_r / r;
            (old_r, r) = (r, old_r - quotient * r);
            (old_s, s) = (s, old_s - quotient * s);
        }

        let bezout_t = if b == T::ZERO {
            T::ZERO
        } else {
            (old_r - old_s * a) / b
        };

        (old_r, old_s, bezout_t)
    }

    fn mod_inv(self, n: T) -> Option<T> {
        let x = self;
        let (g, x, _) = x.egcd(n);
        if g == T::ONE {
            Some(x.rem_euclid(n))
        } else {
            None
        }
    }
}

impl<T: Unsigned<T>> UnsignedMathOps<T> for T {
    fn sqrt(self) -> T {
        let mut a = T::ZERO;
        let mut m;
        let mut b = self + T::ONE;

        while a < b - T::ONE {
            m = (a + b) >> T::ONE;

            if m * m <= self {
                a = m;
            } else {
                b = m;
            }
        }

        a
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn both() {
        // coprimes
        let a: i32 = 3 * 5 * 7;
        let b: i32 = 2 * 11 * 13;
        assert_eq!(a.gcd(b), 1);

        // gcd(8,12) = 4
        assert_eq!(8_u64.gcd(12), 4);
        assert_eq!(8_i64.gcd(12), 4);
        assert_eq!(8_usize.gcd(12), 4);

        // 29^830 (mod 20253) = 14587
        assert_eq!(29_u128.mod_exp(830, 20253), 14587);
    }

    #[test]
    fn signed() {
        // 14*1 + (-21)*1 = 7, 7=gcd(14, 21)
        assert_eq!(14_i32.egcd(-21), (-7, 1, 1));

        // 18*30 - 7*77 = 1, 18 and 77 are coprime
        assert_eq!(18_i32.egcd(77), (1, 30, -7));

        // 3 * 3 = 1 (mod 4)
        assert_eq!(3_i32.mod_inv(4), Some(3));

        // 32863850 * 213 = 1 (mod 1000000007)
        assert_eq!(213.mod_inv(1000000007_i128), Some(32863850));
    }

    #[test]
    fn unsigned() {
        assert_eq!(99u32.sqrt(), 9);
        assert_eq!(100u32.sqrt(), 10);
        assert_eq!(120u32.sqrt(), 10);
        assert_eq!(121u32.sqrt(), 11);

        // from day25 of 2020
        assert_eq!(20201227_u64.sqrt(), 4494);
    }
}
