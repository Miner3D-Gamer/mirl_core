#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "wincode",
    derive(wincode::SchemaWrite, wincode::SchemaRead)
)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A buffer that utilized known metrics for more compile time optimizations
#[cfg_attr(feature = "c_compatible", repr(C))]
pub struct ConstBuffer<const WIDTH: usize, const HEIGHT: usize>
where
    [(); WIDTH * HEIGHT]:,
{
    /// Actual color data
    pub data: [u32; WIDTH * HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> ConstBuffer<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    /// The total size, to use, use; `Buffer::<WIDTH, HEIGHT>::TOTAL_SIZE`
    pub const TOTAL_SIZE: usize = WIDTH * HEIGHT;
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
}

impl<const WIDTH: usize, const HEIGHT: usize> core::ops::Deref
    for ConstBuffer<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
mod new;
mod set_pixel;
