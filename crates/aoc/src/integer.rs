// https://doc.rust-lang.org/book/appendix-02-operators.html

use std::ops::{
    Add, AddAssign, BitAnd, BitOr, Div, Mul, Neg, Rem, Shl, ShlAssign, Shr, ShrAssign, Sub,
    SubAssign,
};

pub trait Integer<T>:
    Copy
    + PartialEq
    + PartialOrd
    + Add<Output = T>
    + AddAssign<T>
    + Sub<Output = T>
    + SubAssign<T>
    + Mul<Output = T>
    + Div<Output = T>
    + Rem<Output = T>
    + BitAnd<Output = T>
    + BitOr<Output = T>
    + Shl<Output = T>
    + Shr<Output = T>
    + ShlAssign<T>
    + ShrAssign<T>
{
    const ZERO: T;
    const ONE: T;

    fn rem_euclid(self, rhs: T) -> T;
}

pub trait Signed<T>: Integer<T> + Neg<Output = T> {}

pub trait Unsigned<T>: Integer<T> {}

macro_rules! integer {
    ($($t:ty)*) => ($(
        impl Integer<$t> for $t {
            const ZERO: $t = 0;
            const ONE: $t = 1;

            fn rem_euclid(self, rhs: $t) -> $t {
                <$t>::rem_euclid(self, rhs) as $t
            }

    }
)*)
}

macro_rules! empty_trait {
    ($name:ident for $($t:ty)*) => ($(
        impl $name<$t> for $t {}
    )*)
}

integer!(i8 i16 i32 i64 i128 isize);
empty_trait!(Signed for i8 i16 i32 i64 i128 isize);

integer!(u8 u16 u32 u64 u128 usize);
empty_trait!(Unsigned for u8 u16 u32 u64 u128 usize);
