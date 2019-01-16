use std::num::FpCategory;

pub trait FloatCore {
    /// Returns positive infinity.
    fn infinity() -> Self;

    /// Returns negative infinity.
    fn neg_infinity() -> Self;

    /// Returns NaN.
    fn nan() -> Self;

    /// Returns `-0.0`.
    fn neg_zero() -> Self;

    /// Returns the smallest positive, normalized value that this type can represent.
    fn min_positive_value() -> Self;

    /// Returns `true` if the number is NaN.
    fn is_nan(self) -> bool;

    /// Returns `true` if the number is infinite.
    fn is_infinite(self) -> bool;

    /// Returns `true` if the number is neither infinite or NaN.
    fn is_finite(self) -> bool;

    /// Returns `true` if the number is neither zero, infinite, subnormal or NaN.
    fn is_normal(self) -> bool;

    /// Returns the floating point category of the number. If only one property
    /// is going to be tested, it is generally faster to use the specific
    /// predicate instead.
    fn classify(self) -> FpCategory;

    /// Returns the largest integer less than or equal to a number.
    fn floor(self) -> Self;

    /// Returns the smallest integer greater than or equal to a number.
    fn ceil(self) -> Self;

    /// Returns the nearest integer to a number. Round half-way cases away from `0.0`.
    fn round(self) -> Self;

    /// Return the integer part of a number.
    fn trunc(self) -> Self;

    /// Returns the fractional part of a number.
    fn fract(self) -> Self;

    /// Returns the reciprocal (multiplicative inverse) of the number.
    fn recip(self) -> Self;

    /// Returns the mantissa, base 2 exponent, and sign as integers, respectively.
    /// The original number can be recovered by `sign * mantissa * 2 ^ exponent`.
    fn integer_decode(self) -> (u64, i16, i8);
}