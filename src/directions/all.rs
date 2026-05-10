use super::{
    Directions, ExtendedDirections, RotateDirections, RotatePrecise,
    SpecialDirections,
};

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
// overly complex generic constant consider moving this anonymous constant into a `const` function this operation may be supported in the future
/// N E S W + NE SE SW NW + None
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AllDirections {
    /// N E S W
    Base(Directions),
    /// NE SE SW NW
    Extended(ExtendedDirections),
    /// No direction
    Special(SpecialDirections),
}
impl core::default::Default for AllDirections {
    fn default() -> Self {
        Self::Special(SpecialDirections::default())
    }
}
impl const RotateDirections for AllDirections {
    fn rotate_clockwise_90(&self) -> Self {
        match self {
            Self::Base(direction) => {
                Self::Base(direction.rotate_clockwise_90())
            }
            Self::Extended(direction) => {
                Self::Extended(direction.rotate_clockwise_90())
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
            Self::Extended(direction) => {
                Self::Extended(direction.rotate_counterclockwise_90())
            }
            Self::Special(SpecialDirections::None) => {
                Self::Special(SpecialDirections::None)
            }
        }
    }
}
impl const RotatePrecise for AllDirections {
    fn rotate_clockwise_45(&self) -> Self {
        match self {
            Self::Base(direction) => Self::Extended(match direction {
                Directions::North => ExtendedDirections::NorthEast,
                Directions::East => ExtendedDirections::SouthEast,
                Directions::South => ExtendedDirections::SouthWest,
                Directions::West => ExtendedDirections::NorthWest,
            }),
            Self::Extended(direction) => Self::Base(match direction {
                ExtendedDirections::NorthEast => Directions::East,
                ExtendedDirections::SouthEast => Directions::South,
                ExtendedDirections::SouthWest => Directions::West,
                ExtendedDirections::NorthWest => Directions::North,
            }),
            Self::Special(SpecialDirections::None) => {
                Self::Special(SpecialDirections::None)
            }
        }
    }

    fn rotate_counterclockwise_45(&self) -> Self {
        match self {
            Self::Base(direction) => Self::Extended(match direction {
                Directions::North => ExtendedDirections::NorthWest,
                Directions::East => ExtendedDirections::NorthEast,
                Directions::South => ExtendedDirections::SouthEast,
                Directions::West => ExtendedDirections::SouthWest,
            }),
            Self::Extended(direction) => Self::Base(match direction {
                ExtendedDirections::NorthEast => Directions::North,
                ExtendedDirections::SouthEast => Directions::East,
                ExtendedDirections::SouthWest => Directions::South,
                ExtendedDirections::NorthWest => Directions::West,
            }),
            Self::Special(SpecialDirections::None) => {
                Self::Special(SpecialDirections::None)
            }
        }
    }
}
