#![allow(clippy::inline_always)]
use super::ConstBuffer;
impl<const WIDTH: usize, const HEIGHT: usize> ConstBuffer<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    #[inline(always)]
    /// Set the pixel at the specified position, it'll check if the pixel is in bounds for you
    pub const fn set_pixel_safe(&mut self, xy: (usize, usize), color: u32) {
        if xy.0 < WIDTH && xy.1 < HEIGHT {
            unsafe {
                *self.data.as_mut_ptr().add(xy.1 * WIDTH + xy.0) = color;
            }
        }
    }
    #[inline(always)]
    /// Set the pixel at the specified position without checking if it is within the allowed memory
    pub const fn set_pixel_unchecked(
        &mut self,
        xy: (usize, usize),
        color: u32,
    ) {
        unsafe {
            *self.data.as_mut_ptr().add(xy.1 * WIDTH + xy.0) = color;
        }
    }
}
