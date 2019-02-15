/// Hyperbolic functions.
pub trait Hyperbolic {
    /// Hyperbolic sine function.
    fn sinh(self) -> Self;

    /// Hyperbolic cosine function.
    fn cosh(self) -> Self;

    /// Hyperbolic tangent function.
    fn tanh(self) -> Self;

    /// Inverse hyperbolic sine function.
    fn asinh(self) -> Self;

    /// Inverse hyperbolic cosine function.
    fn acosh(self) -> Self;

    /// Inverse hyperbolic tangent function.
    fn atanh(self) -> Self;
}
