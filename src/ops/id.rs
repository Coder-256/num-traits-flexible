/// Defines an additive identity element for `Self`.
pub trait Zero {
    /// Returns the additive identity element of `Self`, `0`.
    ///
    /// # Laws
    ///
    /// ```{.text}
    /// a + 0 = a       ∀ a ∈ Self
    /// 0 + a = a       ∀ a ∈ Self
    /// ```
    ///
    /// # Purity
    ///
    /// This function should return the same result at all times regardless of
    /// external mutable state, for example values stored in TLS or in
    /// `static mut`s.
    // This cannot be an associated constant, because of bignums.
    fn zero() -> Self;

    /// Returns `true` if `self` is equal to the additive identity.
    ///
    /// For performance reasons, it's best to implement this manually.
    fn is_zero(self) -> bool;
}

/// Defines a multiplicative identity element for `Self`.
pub trait One {
    /// Returns the multiplicative identity element of `Self`, `1`.
    ///
    /// # Laws
    ///
    /// ```{.text}
    /// a * 1 = a       ∀ a ∈ Self
    /// 1 * a = a       ∀ a ∈ Self
    /// ```
    ///
    /// # Purity
    ///
    /// This function should return the same result at all times regardless of
    /// external mutable state, for example values stored in TLS or in
    /// `static mut`s.
    // This cannot be an associated constant, because of bignums.
    fn one() -> Self;

    /// Returns `true` if `self` is equal to the multiplicative identity.
    ///
    /// For performance reasons, it's best to implement this manually.
    fn is_one(self) -> bool;
}
