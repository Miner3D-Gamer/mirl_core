#![allow(clippy::inline_always)]
use super::Buffer;
use crate::render::traits::BufferGetPixel;
// impl Buffer {
//     // /// Safely get the pixel color of the buffer at the specified x and y, returns the fallback input if the pixel is out of bounds
//     // #[inline(always)]
//     // #[must_use]
//     // pub const fn get_pixel_fallback(
//     //     &self,
//     //     xy: (usize, usize),
//     //     fallback: u32,
//     // ) -> u32 {
//     //     if xy.0 >= self.width || xy.1 >= self.height {
//     //         return fallback;
//     //     }
//     //     let index = xy.1 * self.width + xy.0;
//     //     unsafe { *self.data.as_ptr().add(index) }
//     // }
//     // /// Get the pixel color at a position in a buffer yet before that check if it is in the range of the buffer
//     // /// Instead of returning None if the result isn't in the buffer, it will return the specified fallback value
//     // #[inline(always)]
//     // #[must_use]
//     // #[allow(clippy::cast_sign_loss)]
//     // pub fn get_pixel_isize_fallback(
//     //     &self,
//     //     xy: (isize, isize),
//     //     fallback: u32,
//     // ) -> u32 {
//     //     if xy.0 < 0 || xy.1 < 0 {
//     //         return fallback;
//     //     }
//     //     let y = xy.1 as usize;
//     //     let x = xy.0 as usize;
//     //     if x >= self.width || y >= self.height {
//     //         return fallback;
//     //     }
//     //     let index = y * self.width + x;
//     //     self.data[index]
//     // }
// }

impl const BufferGetPixel for Buffer {
    /// Get the pixel color at a position in a buffer without checking if the pixel is on screen (which will crash the program if it isn't)
    /// The function for getting a pixel safely is [`get_pixel`](Buffer::get_pixel) or [`get_pixel_isize`](Buffer::get_pixel_isize)
    #[inline(always)]
    unsafe fn get_pixel_unchecked(&self, xy: (usize, usize)) -> u32 {
        let index = xy.1 * self.width + xy.0;
        unsafe { *self.data.as_ptr().add(index) }
    }
    /// Safely get the pixel color of the buffer at the specified x and y, returns 0 if the pixel is out of bounds
    /// For a custom return number use [`get_pixel_fallback`](Buffer::get_pixel_fallback)
    /// For getting the pixel without bounds checking use [`get_pixel_unchecked`](Buffer::get_pixel_unchecked)
    #[inline(always)]
    fn get_pixel(&self, xy: (usize, usize)) -> u32 {
        if xy.0 >= self.width || xy.1 >= self.height {
            return 0;
        }
        let index = xy.1 * self.width + xy.0;
        unsafe { *self.data.as_ptr().add(index) }
    }
    /// Safely get the pixel color of the buffer at the specified x and y, returns 0 if the pixel is out of bounds
    /// For a custom return number use [`get_pixel_fallback`](Buffer::get_pixel_fallback)
    /// For getting the pixel without bounds checking use [`get_pixel_unchecked`](Buffer::get_pixel_unchecked)
    #[inline(always)]
    fn get_pixel_option(&self, xy: (usize, usize)) -> Option<u32> {
        if xy.0 >= self.width || xy.1 >= self.height {
            return None;
        }
        let index = xy.1 * self.width + xy.0;
        Some(unsafe { *self.data.as_ptr().add(index) })
    }
}
