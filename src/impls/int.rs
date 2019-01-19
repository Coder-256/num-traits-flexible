use super::super::ops::*;

macro_rules! signed {
    ($t:ty) => {
        impl AbsoluteValue for $t {
            fn abs(self) -> $t {
                self.abs()
            }
        }

        impl Signed for $t {
            forward! {
                Self::signum(self) -> Self;
            }

            fn is_sign_positive(self) -> bool {
                self.is_positive()
            }

            fn is_sign_negative(self) -> bool {
                self.is_negative()
            }
        }
    };
}

macro_rules! int_shared {
    ($t:ty, $s:ty, $u:ty) => {
        impl Bitwise for $t {
            forward! {
                Self::count_ones(self) -> u32;
                Self::count_zeros(self) -> u32;
                Self::leading_zeros(self) -> u32;
                Self::trailing_zeros(self) -> u32;
                Self::rotate_left(self, n: u32) -> Self;
                Self::rotate_right(self, n: u32) -> Self;
                Self::swap_bytes(self) -> Self;
                Self::from_be(x: Self) -> Self;
                Self::from_le(x: Self) -> Self;
                Self::to_be(self) -> Self;
                Self::to_le(self) -> Self;
                Self::pow(self, exp: u32) -> Self;
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
        }

        impl Bounded for $t {
            forward! {
                Self::min_value() -> Self;
                Self::max_value() -> Self;
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

macro_rules! int_impl {
    ($s:ty, $u:ty) => {
        signed!($s);
        int_shared!($s, $s, $u);
        int_shared!($u, $s, $u);
    };
}

int_impl!(isize, usize);
int_impl!(i8, u8);
int_impl!(i16, u16);
int_impl!(i32, u32);
int_impl!(i64, u64);
int_impl!(i128, u128);