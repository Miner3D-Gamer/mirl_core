use super::{draw_pixel_safe, draw_pixel_unsafe};

/// Get and set the raw data of the buffer
pub const trait BufferData {
    /// Get the stored data as a slice
    fn data(&self) -> &[u32];
    /// Get the stored data as a mutable slice
    fn data_mut(&mut self) -> &mut [u32];
}
/// Get data pointers for the given buffer
pub const trait BufferPointers {
    #[must_use]
    /// Get the pointer to the stored data
    fn pointer(&self) -> *const u32;
    #[must_use]
    /// Get the pointer to the stored data mutably
    fn mut_pointer(&mut self) -> *mut u32;
}
/// Set the data inside a buffer
pub const trait SetBufferData {
    /// Set the data inside a buffer
    fn set_data(&mut self, data: &[u32]);
}
impl<T: BufferData> SetBufferData for T {
    default fn set_data(&mut self, data: &[u32]) {
        self.data_mut().copy_from_slice(data);
    }
}
/// Get image size for the given buffer
///
/// # Safety
/// It is the callers responsibility to make sure the stored size and content size are the same
pub const unsafe trait SetBufferMetrics {
    /// Set the width of the stored image
    fn set_width(&mut self, width: usize);
    /// Set the height of the stored image
    fn set_height(&mut self, height: usize);
    /// Set the size of the buffer
    fn set_size(&mut self, size: (usize, usize));
}
/// Get image size for the given buffer
pub const trait BufferMetrics: [const] BufferMetricsHelper {
    /// Get the width of the stored image
    fn width(&self) -> usize;
    /// Get the height of the stored image
    fn height(&self) -> usize;
}
/// A helper trait for [`BufferMetrics`]
pub const trait BufferMetricsHelper {
    /// Get the current size of the buffer in a tuple
    fn get_size(&self) -> (usize, usize);
}
impl<T: [const] BufferMetrics> const BufferMetricsHelper for T {
    default fn get_size(&self) -> (usize, usize) {
        (self.width(), self.height())
    }
}

/// Get the pixel color at a position
pub const trait BufferGetPixel: BufferGetPixelHelper {
    /// Safely get the pixel color of the buffer at the specified x and y, returns 0 if the pixel is out of bounds
    fn get_pixel(&self, xy: (usize, usize)) -> u32;
    /// Safely get the pixel color of the buffer at the specified x and y, returns None if the pixel is out of bounds
    fn get_pixel_option(&self, xy: (usize, usize)) -> Option<u32>;
    /// Get the pixel color at a position in a buffer
    ///yy
    /// # Safety
    /// This function does not check if the pixel coordinates are in bounds
    unsafe fn get_pixel_unchecked(&self, xy: (usize, usize)) -> u32;
}
/// A helper trait with automatic isize support
pub const trait BufferGetPixelHelper {
    /// Get the pixel color at a position in a buffer yet before that check if it is in the range of the buffer
    fn get_pixel_isize(&self, xy: (isize, isize)) -> u32;
    /// Get the pixel color at a position in a buffer yet before that check if it is in the range of the buffer
    fn get_pixel_isize_option(&self, xy: (isize, isize)) -> Option<u32>;
}

impl<T: BufferGetPixel> BufferGetPixelHelper for T {
    /// Get the pixel color at a position in a buffer yet before that check if it is in the range of the buffer
    #[inline(always)]
    #[allow(clippy::cast_sign_loss)]
    fn get_pixel_isize(&self, xy: (isize, isize)) -> u32 {
        if xy.0 < 0 || xy.1 < 0 {
            return 0;
        }
        self.get_pixel((xy.0 as usize, xy.1 as usize))
    }
    /// Get the pixel color at a position in a buffer yet before that check if it is in the range of the buffer
    #[inline(always)]
    #[allow(clippy::cast_sign_loss)]
    fn get_pixel_isize_option(&self, xy: (isize, isize)) -> Option<u32> {
        if xy.0 < 0 || xy.1 < 0 {
            return None;
        }
        self.get_pixel_option((xy.0 as usize, xy.1 as usize))
    }
}
/// Set the pixel color at a position - Automatically implemented for structs that implement [`BufferPointers`] and [`BufferMetrics`]
pub const trait BufferSetPixel {
    /// Set the pixel at the specified position, it'll check if the pixel is in bounds for you
    fn set_pixel_safe(&mut self, xy: (usize, usize), color: u32);
    /// Set the pixel at the specified position without checking if it is within the allowed memory
    ///
    /// # Safety
    /// This function doesn't check if it's coordinates in in bounds and will write into unknown memory if not handled properly
    unsafe fn set_pixel_unchecked(&mut self, xy: (usize, usize), color: u32);
}

impl<T: BufferPointers + BufferMetrics> BufferSetPixel for T {
    /// Set the pixel at the specified position, it'll check if the pixel is in bounds for you
    fn set_pixel_safe(&mut self, xy: (usize, usize), color: u32) {
        crate::render::traits::draw_pixel_safe(self, xy, color);
    }
    /// Set the pixel at the specified position without checking if it is within the allowed memory
    unsafe fn set_pixel_unchecked(&mut self, xy: (usize, usize), color: u32) {
        crate::render::traits::draw_pixel_unsafe(self, xy, color);
    }
}

/// Turn the Buffer into a Vec of u8
pub const trait BufferToVec {
    #[must_use]
    /// Converts the [`Vec<u32>`] to [`Vec<8>`] by unpacking the u32 into argb style
    fn to_u8_argb(&self) -> Vec<u8>;
    #[must_use]
    /// Converts the internal [`Box<[u32]>`](Box<u32>) to [`Vec<8>`] by unpacking the u32 into rgba style
    fn to_u8_rgba(&self) -> Vec<u8>;
    #[must_use]
    /// Converts the internal [`Box<[u32]>`](Box<u32>) to [`Vec<8>`] by unpacking the u32 into rgba style
    fn to_u8_bgra(&self) -> Vec<u8>;
}
impl<S: BufferData> BufferToVec for S {
    default fn to_u8_argb(&self) -> Vec<u8> {
        let mut return_list = Vec::with_capacity(self.data().len() * 4);
        for i in self.data() {
            return_list
                .extend(<[u8; 4]>::from(crate::graphics::u32_to_argb_u8(*i)));
        }
        return_list
    }

    default fn to_u8_rgba(&self) -> Vec<u8> {
        let mut return_list = Vec::with_capacity(self.data().len() * 4);
        for i in self.data() {
            return_list
                .extend(<[u8; 4]>::from(crate::graphics::u32_to_rgba_u8(*i)));
        }
        return_list
    }
    default fn to_u8_bgra(&self) -> Vec<u8> {
        let mut return_list = Vec::with_capacity(self.data().len() * 4);
        for i in self.data() {
            return_list
                .extend(<[u8; 4]>::from(crate::graphics::u32_to_bgra_u8(*i)));
        }
        return_list
    }
}
