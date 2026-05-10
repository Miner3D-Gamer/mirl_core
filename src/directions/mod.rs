/// Rotate a direction by a multiple of 90°
pub const trait RotateDirections: [const] RotateDirectionsHelper {
    #[must_use]
    /// Rotate the direction 90° clockwise
    fn rotate_clockwise_90(&self) -> Self;
    #[must_use]
    /// Rotate the direction 90° counterclockwise (or 270° clockwise)
    fn rotate_counterclockwise_90(&self) -> Self;
}
/// Rotate a direction by a multiple of 90°
pub const trait RotateDirectionsHelper: Sized {
    #[must_use]
    /// Rotate the direction 180°
    fn rotate_180(&self) -> Self;
    #[must_use]
    /// Rotate the direction 270° clockwise (or 90° counterclockwise)
    fn rotate_clockwise_270(&self) -> Self;
    #[must_use]
    /// Rotate the direction 270° counterclockwise (or 90° clockwise)
    fn rotate_counterclockwise_270(&self) -> Self;
}
impl<T: [const] RotateDirections + [const] core::marker::Destruct> const
    RotateDirectionsHelper for T
{
    /// Rotate the direction 180°
    fn rotate_180(&self) -> Self {
        self.rotate_clockwise_90().rotate_clockwise_90()
    }

    /// Rotate the direction 270° clockwise (or 90° counterclockwise)
    fn rotate_clockwise_270(&self) -> Self {
        self.rotate_counterclockwise_90()
    }
    /// Rotate the direction 270° counterclockwise (or 90° clockwise)
    fn rotate_counterclockwise_270(&self) -> Self {
        self.rotate_clockwise_90()
    }
}
/// Rotate a direction by 45° increments
pub const trait RotatePrecise: [const] RotatePreciseHelper {
    #[must_use]
    /// Rotate the direction 45° clockwise
    fn rotate_clockwise_45(&self) -> Self;
    #[must_use]
    /// Rotate the direction 45° counterclockwise
    fn rotate_counterclockwise_45(&self) -> Self;
}
/// Rotate a direction by 45° increments
pub const trait RotatePreciseHelper: [const] RotateDirections {
    #[must_use]
    /// Rotate the direction 135° clockwise (90° + 45°)
    fn rotate_clockwise_135(&self) -> Self;
    #[must_use]
    /// Rotate the direction 135° counterclockwise (90° + 45°)
    fn rotate_counterclockwise_135(&self) -> Self;
}
impl<T: [const] RotatePrecise + [const] core::marker::Destruct> const
    RotatePreciseHelper for T
{
    /// Rotate the direction 135° clockwise (90° + 45°)
    fn rotate_clockwise_135(&self) -> Self {
        self.rotate_clockwise_90().rotate_clockwise_45()
    }
    /// Rotate the direction 135° counterclockwise (90° + 45°)
    fn rotate_counterclockwise_135(&self) -> Self {
        self.rotate_counterclockwise_90().rotate_counterclockwise_45()
    }
}

mod all;
mod cardinal;
mod direction;
mod directions_with_none;
mod extended;
mod special;

pub use all::*;
pub use cardinal::*;
pub use direction::*;
pub use directions_with_none::*;
pub use extended::*;
pub use special::*;

/// Functions that are somewhat related to directions yet do not have direct relation to it
pub mod misc;

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[allow(clippy::struct_excessive_bools, missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
/// A boolean for each simple Direction
pub struct NormalDirections {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub top_left: bool,
    pub top_right: bool,
    pub bottom_left: bool,
    pub bottom_right: bool,
}
impl NormalDirections {
    #[must_use]
    #[allow(clippy::fn_params_excessive_bools)] // Really clippy? 4 booleans is excessive in your eyes?
    /// Create a simple directional boolean struct
    pub const fn new(top: bool, bottom: bool, left: bool, right: bool) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
            top_left: top && left,
            top_right: top && right,
            bottom_left: bottom && left,
            bottom_right: bottom && right,
        }
    }
    /// "Yes"
    #[must_use]
    pub const fn all_true() -> Self {
        Self {
            top: true,
            bottom: true,
            left: true,
            right: true,
            top_left: true,
            top_right: true,
            bottom_left: true,
            bottom_right: true,
        }
    }
    /// "No"
    #[must_use]
    pub const fn all_false() -> Self {
        Self {
            top: false,
            bottom: false,
            left: false,
            right: false,
            top_left: false,
            top_right: false,
            bottom_left: false,
            bottom_right: false,
        }
    }
}
