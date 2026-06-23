/// A value of 10 with a variance of 5 will return (5, 15)
pub fn range_with_variance<T: core::ops::Add<Output = T> + core::ops::Sub<Output = T> + Copy>(
    value: T,
    variance: T,
) -> (T, T) {
    (value - variance, value + variance)
}
