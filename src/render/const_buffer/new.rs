use super::ConstBuffer;
use crate::render::traits::{BufferData, BufferMetrics, BufferPointers};

impl<const WIDTH: usize, const HEIGHT: usize> ConstBuffer<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    #[must_use]
    /// Create a new buffer
    pub const fn new(data: [u32; WIDTH * HEIGHT]) -> Self {
        Self {
            data,
        }
    }

    #[must_use]
    /// Create a new, empty, [`ConstBuffer`]
    pub const fn new_empty() -> Self {
        Self {
            data: [0; WIDTH * HEIGHT],
        }
    }
    #[must_use]
    /// Create a new [`ConstBuffer`] filled with the specified color
    pub const fn new_empty_with_color(color: u32) -> Self {
        Self {
            data: [color; WIDTH * HEIGHT],
        }
    }
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    /// Generate a error texture with the desired size
    pub fn generate_fallback(squares: usize) -> Self {
        let mut data = [0; WIDTH * HEIGHT];
        let square_size = WIDTH.midpoint(HEIGHT) / squares;

        let purple = crate::graphics::rgba_to_u32(128, 0, 128, 255);
        let black = crate::graphics::rgba_to_u32(0, 0, 0, 255);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let square_x = x / square_size;
                let square_y = y / square_size;

                let color = if (square_x + square_y).is_multiple_of(2) {
                    purple
                } else {
                    black
                };
                data[y * WIDTH + x] = color;
            }
        }
        Self::new(data)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> BufferData
    for ConstBuffer<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    fn data(&self) -> &[u32] {
        &self.data
    }
    fn data_mut(&mut self) -> &mut [u32] {
        &mut self.data
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> const BufferMetrics
    for ConstBuffer<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    fn width(&self) -> usize {
        WIDTH
    }

    fn height(&self) -> usize {
        HEIGHT
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> const BufferPointers
    for ConstBuffer<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    fn pointer(&self) -> *const u32 {
        self.pointer()
    }

    fn mut_pointer(&mut self) -> *mut u32 {
        self.mut_pointer()
    }
}
