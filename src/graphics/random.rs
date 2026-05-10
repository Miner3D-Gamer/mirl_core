use crate::math::range_with_variance;

#[must_use]
/// Get a random color within the given color range
pub fn generate_random_color(range: ((u32, u32, u32), (u32, u32, u32))) -> u32 {
    let red = rand::random_range(range.0.0..range.1.0);
    let green = rand::random_range(range.0.1..range.1.1);
    let blue = rand::random_range(range.0.2..range.1.2);
    crate::graphics::rgba_to_u32(red, green, blue, 255)
}

/// Replace a color in the buffer with a random color within the given color range
pub fn replace_color_with_random_color(
    buffer: &mut crate::prelude::Buffer,
    color: u32,
    range: ((u32, u32, u32), (u32, u32, u32)),
) {
    for i in &mut buffer.data {
        if *i == color {
            *i = generate_random_color(range);
        }
    }
}
#[must_use]
/// Create a range of colors based on a variance
pub fn color_range_with_variance(
    color: (u32, u32, u32),
    variance: (u32, u32, u32),
) -> ((u32, u32, u32), (u32, u32, u32)) {
    crate::graphics::reorder_color_range((
        range_with_variance(color.0, variance.0),
        range_with_variance(color.1, variance.1),
        range_with_variance(color.2, variance.2),
    ))
}
