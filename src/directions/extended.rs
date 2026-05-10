use super::RotateDirections;

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
/// NE SE SW NW
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[allow(missing_docs)]
pub enum ExtendedDirections {
    #[default]
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}
impl const RotateDirections for ExtendedDirections {
    fn rotate_clockwise_90(&self) -> Self {
        match self {
            Self::NorthEast => Self::SouthEast,
            Self::SouthEast => Self::SouthWest,
            Self::SouthWest => Self::NorthWest,
            Self::NorthWest => Self::NorthEast,
        }
    }
    fn rotate_counterclockwise_90(&self) -> Self {
        match self {
            Self::NorthEast => Self::NorthWest,
            Self::SouthEast => Self::NorthEast,
            Self::SouthWest => Self::SouthEast,
            Self::NorthWest => Self::SouthWest,
        }
    }
}
