/// Exponential functions.
pub trait Exponential {
    /// Raise a number to an integer power.
    ///
    /// Using this function is generally faster than using `powf`
    fn powi(self, n: i32) -> Self;

    /// Take the square root of a number.
    ///
    /// Returns NaN if `self` is a negative floating-point number.  
    ///
    /// # Panics
    ///
    /// If the implementing type doesn't support NaN, this method should panic if `self < 0`.
    fn sqrt(self) -> Self;

    /// Take the cubic root of a number.
    fn cbrt(self) -> Self;

    /// Returns `e^(self)`, (the exponential function).
    fn exp(self) -> Self;

    /// Returns `2^(self)`.
    fn exp2(self) -> Self;

    /// Calculate the length of the hypotenuse of a right-angle triangle given
    /// legs of length `x` and `y`.
    fn hypot(self, other: Self) -> Self;

    /// Returns `e^(self) - 1` in a way that is accurate even if the
    /// number is close to zero.
    fn exp_m1(self) -> Self;
}

pub trait ExponentialFloat {
    /// Raise a number to a floating point power.
    fn powf(self, n: Self) -> Self;
}
