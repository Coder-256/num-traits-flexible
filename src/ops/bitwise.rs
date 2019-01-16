/// Bitwise operations (beyond std::ops)
pub trait Bitwise {
    /// Returns the number of ones in the binary representation of `self`.
    fn count_ones(self) -> u32;

    /// Returns the number of zeros in the binary representation of `self`.
    fn count_zeros(self) -> u32;

    /// Returns the number of leading zeros in the binary representation
    /// of `self`.
    fn leading_zeros(self) -> u32;

    /// Returns the number of trailing zeros in the binary representation
    /// of `self`.
    fn trailing_zeros(self) -> u32;

    /// Shifts the bits to the left by a specified amount amount, `n`, wrapping
    /// the truncated bits to the end of the resulting integer.
    fn rotate_left(self, n: u32) -> Self;

    /// Shifts the bits to the right by a specified amount amount, `n`, wrapping
    /// the truncated bits to the beginning of the resulting integer.
    fn rotate_right(self, n: u32) -> Self;

    /// Shifts the bits to the left by a specified amount amount, `n`, filling
    /// zeros in the least significant bits.
    ///
    /// This is bitwise equivalent to signed `Shl`.
    fn signed_shl(self, n: u32) -> Self;

    /// Shifts the bits to the right by a specified amount amount, `n`, copying
    /// the "sign bit" in the most significant bits even for unsigned types.
    ///
    /// This is bitwise equivalent to signed `Shr`.
    fn signed_shr(self, n: u32) -> Self;

    /// Shifts the bits to the left by a specified amount amount, `n`, filling
    /// zeros in the least significant bits.
    ///
    /// This is bitwise equivalent to unsigned `Shl`.
    fn unsigned_shl(self, n: u32) -> Self;

    /// Shifts the bits to the right by a specified amount amount, `n`, filling
    /// zeros in the most significant bits.
    ///
    /// This is bitwise equivalent to unsigned `Shr`.
    fn unsigned_shr(self, n: u32) -> Self;

    /// Reverses the byte order of the integer.
    fn swap_bytes(self) -> Self;

    /// Convert an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    fn from_be(x: Self) -> Self;

    /// Convert an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    fn from_le(x: Self) -> Self;

    /// Convert `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    fn to_be(self) -> Self;

    /// Convert `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    fn to_le(self) -> Self;

    /// Raises self to the power of `exp`, using exponentiation by squaring.
    fn pow(self, exp: u32) -> Self;
}