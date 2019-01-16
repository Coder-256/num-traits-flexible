use std::ops::Neg;

/// The absolute value function.
pub trait AbsoluteValue<AbsOutput: ?Sized = Self>: Neg {
    /// Computes the absolute value.
    fn abs(self) -> AbsOutput;
}
