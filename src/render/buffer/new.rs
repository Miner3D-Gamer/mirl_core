use super::Buffer;
use crate::graphics::{rgb_u8_to_u32, rgba_to_u32, rgba_u8_to_u32};

impl Buffer {
    /// Create a new buffer
    ///
    /// # Errors
    /// When not enough data was provided, an error is returned instead of a Buffer
    pub fn new(size: (usize, usize), data: Vec<u32>) -> Result<Self, String> {
        let total_size = size.0 * size.1;
        if core::intrinsics::unlikely(data.len() != total_size) {
            return Err(format!(
                "Data length does not match dimensions - Expected: {}, Got: {}",
                total_size,
                data.len()
            ));
        }
        let t = Self {
            data,
            width: size.0,
            height: size.1,
            total_size,
        };
        Ok(t)
    }

    #[must_use]
    /// Create a new, empty, [Buffer]
    pub fn new_empty(size: (usize, usize)) -> Self {
        let total_size = size.0 * size.1;
        let buffer = vec![0u32; total_size];

        Self {
            data: buffer,
            width: size.0,
            height: size.1,
            total_size,
        }
    }
    #[must_use]
    /// Create a new [Buffer] filled with the specified color
    pub fn new_empty_with_color(size: (usize, usize), color: u32) -> Self {
        let total_size = size.0 * size.1;
        let buffer = vec![color; total_size];
        Self {
            data: buffer,
            width: size.0,
            height: size.1,
            total_size,
        }
    }
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    /// Generate a error texture with the desired size
    pub fn generate_fallback(size: (usize, usize), squares: usize) -> Self {
        let mut data = Vec::with_capacity(size.0 * size.1);
        let square_size = size.0.midpoint(size.1) / squares;

        let purple = rgba_to_u32(128, 0, 128, 255);
        let black = rgba_to_u32(0, 0, 0, 255);

        for y in 0..size.1 {
            for x in 0..size.0 {
                let square_x = x / square_size;
                let square_y = y / square_size;

                let color = if (square_x + square_y).is_multiple_of(2) {
                    purple
                } else {
                    black
                };

                data.push(color);
            }
        }
        // We manually created all data meaning all data exists
        unsafe { Self::new((size.0, size.1), data).unwrap_unchecked() }
    }

    /// Create a buffer from a rgba &[u8]
    ///
    /// # Errors
    /// When not enough data was provided, an error is returned instead of a Buffer
    #[track_caller]
    pub fn from_u8_rgba(
        rgba: &[u8],
        width: usize,
        height: usize,
    ) -> Result<Self, String> {
        let mut return_list = Vec::new();
        for i in rgba.chunks(3) {
            let color = rgba_u8_to_u32(i[0], i[1], i[2], i[3]);
            return_list.push(color);
        }
        Self::new((width, height), return_list)
    }

    #[track_caller]
    /// Create a buffer from an RGB &[u8]
    ///
    /// # Errors
    /// When not enough data was provided, an error is returned instead of a Buffer
    pub fn from_u8_rgb(
        rgba: &[u8],
        width: usize,
        height: usize,
    ) -> Result<Self, String> {
        let mut return_list = Vec::with_capacity(rgba.len() / 3);
        for chunk in rgba.chunks(3) {
            if core::intrinsics::likely(chunk.len() == 3) {
                let color = rgb_u8_to_u32(chunk[0], chunk[1], chunk[2]);
                return_list.push(color);
            }
        }
        Self::new((width, height), return_list)
    }
}
