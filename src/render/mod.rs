#[cfg(feature = "std")]
mod buffer;
#[cfg(feature = "std")]
pub use buffer::Buffer;

mod const_buffer;
pub use const_buffer::ConstBuffer;

use crate::render::traits::{BufferMetrics, BufferPointers};

/// Simple, essential, traits buffers must implement
pub mod traits;

mod buffer_compatibility;

/// Support for formats like .bmp or .cur
pub mod formats;

/// Draw a pixel color onto the buffer without checking if the pixel is on screen (which will crash the program if it isn't)
#[inline(always)]
#[allow(clippy::inline_always)]
#[track_caller]
pub const fn draw_pixel_unsafe(
    buffer: &mut (impl [const] BufferPointers + [const] BufferMetrics),
    xy: (usize, usize),
    color: u32,
) {
    unsafe {
        *buffer.mut_pointer().add(xy.1 * buffer.width() + xy.0) = color;
    }
}
/// Draw a pixel color onto the buffer by first checking if the pixel is on screen
#[inline(always)]
#[allow(clippy::inline_always)]
#[track_caller]
pub const fn draw_pixel_safe(
    buffer: &mut (impl [const] BufferPointers + [const] BufferMetrics),
    xy: (usize, usize),
    color: u32,
) {
    if xy.0 < buffer.width() && xy.1 < buffer.height() {
        unsafe {
            *buffer.mut_pointer().add(xy.1 * buffer.width() + xy.0) = color;
        }
    }
}
