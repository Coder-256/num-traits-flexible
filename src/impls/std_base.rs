use super::super::ops::*;

macro_rules! signed {
    ($t:ty) => {
        impl AbsoluteValue for $t {
            fn abs(self) -> $t {
                self.abs()
            }
        }

        impl Signed for $t {
            fn signum(self) -> $t {
                match self {
                    n if n > 0 => 1,
                    0 => 0,
                    _ => -1,
                }
            }

            fn is_positive(self) -> bool {
                self > 0
            }

            fn is_negative(self) -> bool {
                self < 0
            }
        }
    };
}

macro_rules! shared_base {
    ($t:ty, $s:ty, $u:ty) => {
        impl Bitwise for $t {
            fn count_ones(self) -> u32 {
                Self::count_ones(self)
            }
            fn count_zeros(self) -> u32 {
                Self::count_zeros(self)
            }
            fn leading_zeros(self) -> u32 {
                Self::leading_zeros(self)
            }
            fn trailing_zeros(self) -> u32 {
                Self::trailing_zeros(self)
            }
            fn rotate_left(self, n: u32) -> Self {
                Self::rotate_left(self, n)
            }
            fn rotate_right(self, n: u32) -> Self {
                Self::rotate_right(self, n)
            }
            fn signed_shl(self, n: u32) -> Self {
                ((self as $s) << n) as $t
            }
            fn signed_shr(self, n: u32) -> Self {
                ((self as $s) >> n) as $t
            }
            fn unsigned_shl(self, n: u32) -> Self {
                ((self as $u) << n) as $t
            }
            fn unsigned_shr(self, n: u32) -> Self {
                ((self as $u) >> n) as $t
            }
            fn swap_bytes(self) -> Self {
                Self::swap_bytes(self)
            }
            fn from_be(x: Self) -> Self {
                Self::from_be(x)
            }
            fn from_le(x: Self) -> Self {
                Self::from_le(x)
            }
            fn to_be(self) -> Self {
                Self::to_be(self)
            }
            fn to_le(self) -> Self {
                Self::to_le(self)
            }
            fn pow(self, exp: u32) -> Self {
                Self::pow(self, exp)
            }
        }

        impl Bounded for $t {
            fn min_value() -> Self {
                Self::min_value()
            }
            fn max_value() -> Self {
                Self::max_value()
            }
        }

        // TODO
        // impl Exponential for $t {
        //     fn powi(self, n: i32) -> Self {
        //         self.pow(n as u32)
        //     }
        //     fn sqrt(self) -> Self {
        //         f64::from(self).sqrt() as Self
        //     }
        //     fn exp(self) -> Self {
        //         self.exp()
        //     }
        //     fn exp2(self) -> Self {
        //         self.exp2()
        //     }
        //     fn hypot(self, other: Self) -> Self {
        //         self.hypot(other)
        //     }
        //     fn exp_m1(self) -> Self {
        //         self.exp_m1()
        //     }
        // }
        
        impl Zero for $t {
            fn zero() -> Self {
                0
            }

            fn is_zero(self) -> bool {
                self == 0
            }
        }

        impl One for $t {
            fn one() -> Self {
                1
            }

            fn is_one(self) -> bool {
                self == 1
            }
        }

        impl MulAdd for $t {
            type Output = Self;
            fn mul_add(self, a: Self, b: Self) -> Self {
                (self * a) + b
            }
        }

        impl MulAddAssign for $t {
            fn mul_add_assign(&mut self, a: Self, b: Self) {
                *self = (*self * a) + b
            }
        }

        // TODO: Logarithmic
    };
}

macro_rules! base_impl {
    ($s:ty, $u:ty) => {
        signed!($s);
        shared_base!($s, $s, $u);
        shared_base!($u, $s, $u);
    };
}

base_impl!(isize, usize);
base_impl!(i8, u8);
base_impl!(i16, u16);
base_impl!(i32, u32);
base_impl!(i64, u64);
base_impl!(i128, u128);