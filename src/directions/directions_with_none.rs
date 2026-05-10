use super::{Directions, RotateDirections, SpecialDirections};

/// N E S W + None
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
pub enum DirectionsWithNone {
    /// N E S W
    Base(Directions),
    /// NE SE SW NW
    Special(SpecialDirections),
}
impl core::default::Default for DirectionsWithNone {
    fn default() -> Self {
        Self::Special(SpecialDirections::default())
    }
}
impl const RotateDirections for DirectionsWithNone {
    fn rotate_clockwise_90(&self) -> Self {
        match self {
            Self::Base(direction) => {
                Self::Base(direction.rotate_clockwise_90())
            }
            Self::Special(SpecialDirections::None) => {
                Self::Special(SpecialDirections::None)
            }
        }
    }
    fn rotate_counterclockwise_90(&self) -> Self {
        match self {
            Self::Base(direction) => {
                Self::Base(direction.rotate_counterclockwise_90())
            }
            Self::Special(SpecialDirections::None) => {
                Self::Special(SpecialDirections::None)
            }
        }
    }
}
