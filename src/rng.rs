use crate::word::Word;

/// A random number generator that produces unsigned words of a fixed type.
///
/// Implementors provide [`nextu`](Self::nextu); the remaining methods convert
/// its output into floating-point values or bounded ranges.
///
/// This trait does not guarantee statistical quality or cryptographic security.
///
/// ## Example
/// ```
/// struct MyRng32(u32);
///
/// impl trng::Rng for MyRng32 {
///     type Word = u32;
///
///     fn nextu(&mut self) -> Self::Word {
///         self.0 = self.0.wrapping_add(1);
///         self.0
///     }
/// }
/// ```
pub trait Rng {
    /// The unsigned output type (`u32` or `u64`).
    type Word: crate::word::Word;

    /// Returns the next unsigned output word.
    #[must_use]
    fn nextu(&mut self) -> Self::Word;

    /// Returns a floating-point value in `[0.0, 1.0)`.
    #[must_use]
    #[inline]
    fn nextf(&mut self) -> <Self::Word as crate::word::Word>::Float {
        self.nextu().to_float()
    }

    /// Returns a signed integer in the inclusive range `[min, max]`.
    ///
    /// Uses multiply-shift range reduction, which can introduce bias when the
    /// range size does not divide the number of possible output words.
    ///
    /// The caller must ensure `min <= max`; this is not checked.
    #[must_use]
    #[inline]
    fn randi(
        &mut self,
        min: <Self::Word as crate::word::Word>::Int,
        max: <Self::Word as crate::word::Word>::Int,
    ) -> <Self::Word as crate::word::Word>::Int {
        self.nextu().randi(min, max)
    }

    /// Returns a floating-point value in `[min, max)` for finite bounds with
    /// `min < max`, subject to floating-point arithmetic limitations.
    ///
    /// This method does not validate its bounds.
    #[must_use]
    #[inline]
    fn randf(
        &mut self,
        min: <Self::Word as crate::word::Word>::Float,
        max: <Self::Word as crate::word::Word>::Float,
    ) -> <Self::Word as crate::word::Word>::Float {
        self.nextu().randf(min, max)
    }
}
