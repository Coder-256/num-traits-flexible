use super::super::ops::*;
use std::f32;
use std::f64;
use std::mem;
use std::num::{FpCategory, NonZeroU8};

macro_rules! float_impl {
    ($t:ident, $decode:ident, $signal:expr) => {
        impl Angles for $t {
            forward! {
                Self::to_degrees(self) -> Self;
                Self::to_radians(self) -> Self;
            }
        }

        impl Bounded for $t {
            constant! {
                min_value() -> $t::MIN;
                max_value() -> $t::MAX;
            }
        }

        impl Epsilon for $t {
            constant! {
                epsilon() -> $t::EPSILON;
            }
        }

        impl Exponential for $t {
            forward! {
                Self::sqrt(self) -> Self;
                Self::cbrt(self) -> Self;
                Self::exp(self) -> Self;
                Self::exp2(self) -> Self;
                Self::hypot(self, other: Self) -> Self;
                Self::exp_m1(self) -> Self;
            }
        }

        impl FloatCore for $t {
            constant! {
                nan() -> $t::NAN;
                infinity() -> $t::INFINITY;
                neg_infinity() -> $t::NEG_INFINITY;
                neg_zero() -> -0.0;
                min_positive_value() -> $t::MIN_POSITIVE;
            }

            forward! {
                Self::is_nan(self) -> bool;
                Self::is_infinite(self) -> bool;
                Self::is_finite(self) -> bool;
                Self::is_normal(self) -> bool;
                Self::classify(self) -> FpCategory;
                Self::floor(self) -> Self;
                Self::ceil(self) -> Self;
                Self::round(self) -> Self;
                Self::trunc(self) -> Self;
                Self::fract(self) -> Self;
                Self::recip(self) -> Self;
            }

            fn is_signaling(self) -> bool {
                (self.to_bits() & (1 << $signal)) == 0
            }
        }

        impl Hyperbolic for $t {
            forward! {
                Self::sinh(self) -> Self;
                Self::cosh(self) -> Self;
                Self::tanh(self) -> Self;
                Self::asinh(self) -> Self;
                Self::acosh(self) -> Self;
                Self::atanh(self) -> Self;
            }
        }

        impl Logarithmic for $t {
            forward! {
                Self::ln(self) -> Self;
                Self::log(self, base: Self) -> Self;
                Self::log2(self) -> Self;
                Self::log10(self) -> Self;
                Self::ln_1p(self) -> Self;
            }
        }

        impl MulAdd for $t {
            type Output = Self;
            forward! {
                Self::mul_add(self, a: Self, b: Self) -> Self;
            }
        }

        impl Normalize for $t {
            fn normalize(self) -> Self {
                self.signum()
            }
        }

        impl One for $t {
            fn one() -> Self {
                1.0
            }

            #[allow(clippy::float_cmp)]
            fn is_one(self) -> bool {
                self == 1.0
            }
        }

        impl PNorm for $t {
            type NormOutput = $t;

            fn pnorm(self, p: NonZeroU8) -> Self::NormOutput {
                if p.get() % 2 == 0 {
                    self.abs()
                } else {
                    self
                }
            }
        }

        impl Power<i32> for $t {
            fn pow(self, exp: i32) -> Self {
                self.powi(exp)
            }
        }

        impl Power<$t> for $t {
            fn pow(self, exp: Self) -> Self {
                self.powf(exp)
            }
        }

        impl Trigonometric for $t {
            forward! {
                Self::sin(self) -> Self;
                Self::cos(self) -> Self;
                Self::tan(self) -> Self;
                Self::asin(self) -> Self;
                Self::acos(self) -> Self;
                Self::atan(self) -> Self;
                Self::atan2(self, other: Self) -> Self;
                Self::sin_cos(self) -> (Self, Self);
            }
        }

        impl Zero for $t {
            fn zero() -> Self {
                0.0
            }

            fn is_zero(self) -> bool {
                self == 0.0
            }
        }
    };
}

impl FloatIntDecode<u32, i16> for f32 {
    fn integer_decode(self) -> (u32, i16, i8) {
        let bits: u32 = unsafe { mem::transmute(self) };
        let sign: i8 = if bits >> 31 == 0 { 1 } else { -1 };
        let mut exponent: i16 = ((bits >> 23) & 0xff) as i16;
        let mantissa = if exponent == 0 {
            (bits & 0x007f_ffff) << 1
        } else {
            (bits & 0x007f_ffff) | 0x0080_0000
        };
        // Exponent bias + mantissa shift
        exponent -= 127 + 23;
        (mantissa, exponent, sign)
    }
}

impl FloatIntDecode<u64, i16> for f64 {
    fn integer_decode(self) -> (u64, i16, i8) {
        let bits: u64 = unsafe { mem::transmute(self) };
        let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
        let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
        let mantissa = if exponent == 0 {
            (bits & 0x000f_ffff_ffff_ffff) << 1
        } else {
            (bits & 0x000f_ffff_ffff_ffff) | 0x0010_0000_0000_0000
        };
        // Exponent bias + mantissa shift
        exponent -= 1023 + 52;
        (mantissa, exponent, sign)
    }
}

float_impl!(f32, integer_decode_f32, 22);
float_impl!(f64, integer_decode_f64, 51);
