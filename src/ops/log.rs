/// Logarithmic functions.
pub trait Logarithmic {
    /// Returns the natural logarithm of the number.
    ///
    /// # Panics
    ///
    /// If `self <= 0` and this type does not support a NaN representation, this function should panic.
    fn ln(self) -> Self;

    /// Returns the logarithm of the number with respect to an arbitrary base.
    ///
    /// # Panics
    ///
    /// If `self <= 0` and this type does not support a NaN representation, this function should panic.
    fn log(self, base: Self) -> Self;

    /// Returns the base 2 logarithm of the number.
    ///
    /// # Panics
    ///
    /// If `self <= 0` and this type does not support a NaN representation, this function should panic.
    fn log2(self) -> Self;

    /// Returns the base 10 logarithm of the number.
    ///
    /// # Panics
    ///
    /// If `self <= 0` and this type does not support a NaN representation, this function should panic.
    fn log10(self) -> Self;

    /// Returns `ln(1+n)` (natural logarithm) more accurately than if
    /// the operations were performed separately.
    ///
    /// # Panics
    ///
    /// If this type does not support a NaN representation, this function should panic
    /// if `self-1 <= 0`.
    fn ln_1p(self) -> Self;
}
