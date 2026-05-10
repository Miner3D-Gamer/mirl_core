use crate::render::traits::{
    BufferData, BufferMetrics, BufferPointers, SetBufferMetrics,
};

// #[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
// #[cfg_attr(feature = "wincode", derive(wincode::SchemaWrite, wincode::SchemaRead))]
// #[cfg_attr(feature = "serde", derive(serde::Serialize))]
// Rewrite to use copyable list instead of Vec<[u32]>?
/// A raw color buffer to be modified and read quickly
#[derive(PartialEq, Debug, Eq, Clone, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "c_compatible", repr(C))]
pub struct Buffer {
    /// Actual color data
    pub data: Vec<u32>,
    // /// Pointer to the color data
    // pub pointer: *mut u32,
    /// Width of the buffer
    pub width: usize,
    /// Height of the buffer
    pub height: usize,
    /// The total size -> width*height
    pub total_size: usize,
}

// Automatically convert the usage of Buffer to Buffer.data
impl core::ops::Deref for Buffer {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// unsafe impl core::marker::Send for Buffer {}
// unsafe impl core::marker::Sync for Buffer {}

impl Buffer {
    #[must_use]
    /// Get the pointer to self.data
    pub const fn pointer(&self) -> *const u32 {
        self.data.as_ptr()
    }
    #[must_use]
    /// Get the pointer to self.data mutably
    pub const fn mut_pointer(&mut self) -> *mut u32 {
        self.data.as_mut_ptr()
    }
    // /// Update internal pointer
    // pub const fn update_pointer(&mut self) {
    //     self.pointer = self.data.as_mut_ptr();
    // }
    /// Update the total size of the buffer
    pub const fn update_total_size(&mut self) {
        self.total_size = self.width * self.height;
    }
}
unsafe impl const SetBufferMetrics for Buffer {
    fn set_height(&mut self, height: usize) {
        self.height = height;
    }
    fn set_width(&mut self, width: usize) {
        self.width = width;
    }
    fn set_size(&mut self, size: (usize, usize)) {
        self.set_height(size.1);
        self.set_width(size.0);
    }
}

impl BufferData for Buffer {
    fn data(&self) -> &[u32] {
        &self.data
    }
    fn data_mut(&mut self) -> &mut [u32] {
        &mut self.data
    }
}

impl const BufferMetrics for Buffer {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl const BufferPointers for Buffer {
    fn pointer(&self) -> *const u32 {
        self.pointer()
    }

    fn mut_pointer(&mut self) -> *mut u32 {
        self.mut_pointer()
    }
}

mod get_pixel;
mod new;
