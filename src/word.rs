mod sealed {
    pub trait Sealed: Copy + Sized {}
    impl self::Sealed for u32 {}
    impl self::Sealed for u64 {}
}

/// An unsigned output word supported by [`Rng`](crate::Rng).
///
/// This trait is sealed and implemented only for `u32` and `u64`.
pub trait Word: self::sealed::Sealed {
    /// The signed integer type used for ranged draws ([`i32`] / [`i64`]).
    type Int: Copy + Sized;

    /// The float type produced from this word ([`f32`] for [`u32`], [`f64`] for [`u64`]).
    type Float: Copy + Sized;

    /// Converts this word to a floating-point value in `[0.0, 1.0)`.
    #[must_use]
    fn to_float(self) -> Self::Float;

    /// Maps this word to a signed integer in the inclusive range `[min, max]`.
    ///
    /// Uses multiply-shift reduction, which can introduce bias.
    /// The caller must ensure `min <= max`; this is not checked.
    #[must_use]
    fn randi(self, min: Self::Int, max: Self::Int) -> Self::Int;

    /// Maps this word to a floating-point value in `[min, max)` for finite
    /// bounds with `min < max`, subject to floating-point arithmetic limitations.
    /// Bounds are not validated.
    #[must_use]
    fn randf(self, min: Self::Float, max: Self::Float) -> Self::Float;
}

macro_rules! impl_word {
    ($($bits:tt),+ $(,)?) => {
        $(::pastey::paste! {
            impl self::Word for [<u $bits>] {
                type Float = [<f $bits>];
                type Int = [<i $bits>];

                #[inline(always)]
                fn to_float(self) -> Self::Float {
                    $crate::u2f_01!(Self::Float, $bits, self)
                }

                #[inline(always)]
                fn randi(self, min: Self::Int, max: Self::Int) -> Self::Int {
                    let range = {
                        (max as $crate::randi_wide!(i $bits) - min as $crate::randi_wide!(i $bits) + 1)
                        as $crate::randi_wide!(u $bits)
                    };
                    (((self as $crate::randi_wide!(u $bits) * range) >> $bits) as [<i $bits>]).wrapping_add(min)
                }

                #[inline(always)]
                fn randf(self, min: Self::Float, max: Self::Float) -> Self::Float {
                    let v = self.to_float() * (max - min) + min;
                    if v < max { v } else { max.next_down() }
                }
            }
        })+
    };
}

impl_word!(32, 64);
