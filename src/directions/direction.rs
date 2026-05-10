use super::RotateDirections;

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
/// N E S W
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
pub enum Directions {
    #[default]
    /// Up
    ///
    /// "It's North, what'd you expect?""
    North,
    /// Right
    ///
    /// "It's East, what'd you expect?""
    East,
    /// Down
    ///
    /// "It's South, what'd you expect?""
    South,
    /// Left
    ///
    /// "It's West, what'd you expect?""
    West,
}

impl const RotateDirections for Directions {
    fn rotate_clockwise_90(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
    fn rotate_counterclockwise_90(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }
}
