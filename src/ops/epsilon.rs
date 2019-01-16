/// Returns epsilon, a small positive value.
///
/// # Panics
///
/// The default implementation will panic if `f32::EPSILON` cannot
/// be cast to `Self`.
pub trait Epsilon {
    fn epsilon() -> Self;
}
