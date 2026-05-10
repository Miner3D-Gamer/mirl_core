#![allow(clippy::modulo_one)]

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
/// None
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
pub enum SpecialDirections {
    #[allow(missing_docs)]
    #[default]
    None,
}
