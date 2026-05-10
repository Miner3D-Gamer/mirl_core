use super::{rgb_u8_to_u32, u32_to_rgba_u8};


#[mirl_derive::derive_all]
/// A pixel made to be read FAST
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pixel {
    /// Red channel
    pub r: u8,
    /// Green Channel
    pub g: u8,
    /// Blue channel,
    pub b: u8,
    /// Alpha Channel
    pub a: u8,
    /// Color as u32
    pub color: u32,
}

impl Pixel {
    /// Create a new Pixel instance using rgb values
    #[must_use]
    pub const fn new_rgb(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r,
            g,
            b,
            a,
            color: rgb_u8_to_u32(r, g, b),
        }
    }
    /// Create a new Pixel instance using a u32 rgba
    #[must_use]
    pub const fn new_32(color: u32) -> Self {
        let (r, g, b, a) = u32_to_rgba_u8(color);
        Self {
            r,
            g,
            b,
            a,
            color,
        }
    }
}
impl From<Pixel> for u32 {
    fn from(p: Pixel) -> Self {
        p.color
    }
}
