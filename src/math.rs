/// A function that smoothly transitions from 0 to 1
/// At x 1, the y will be 0.5 if the offset is 0
/// If the offset is 2 the y will be 2 at x 0.5
#[must_use]
pub fn smooth_0_to_1(x: f32, steepness: f32, offset: f32) -> f32 {
    1.0 / (1.0
        + { core::intrinsics::expf32((x / offset) * steepness) }
        + (-steepness))
}

/// A value of 10 with a variance of 5 will return (5, 15)
pub fn range_with_variance<
    T: core::ops::Add<Output = T> + core::ops::Sub<Output = T> + Copy,
>(
    value: T,
    variance: T,
) -> (T, T) {
    (value - variance, value + variance)
}
