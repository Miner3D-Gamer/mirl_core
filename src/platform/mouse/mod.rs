use crate::{Buffer, ConstBuffer, render::traits::BufferMetricsHelper};

// #[cfg_attr(feature = "c_compatible", repr(C))]
// pub struct MousePos<T> {
//     pos: (T, T),
//     delta_pos: (T, T),
// }

// #[cfg_attr(feature = "c_compatible", repr(C))]
// pub struct MouseData<T> {
//     buttons: MouseState,
//     pos: MousePos<T>,
// }
// #[mirl_derive::derive_all]
// // #[cfg_attr(feature = "strum", derive(strum::EnumIter))]
// #[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
// // #[cfg_attr(feature = "wincode", derive(wincode::SchemaWrite, wincode::SchemaRead))]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "c_compatible", repr(C))]

// #[deprecated = "This struct is experimental"]
// #[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
// /// A raw representation of a cursor, the hotspot will never be outside the image bounds
// ///
// /// Manually setting the hotspot/image will shift all safety liabilities to you
// pub struct ConstRawCursor<const WIDTH: usize, const HEIGHT: usize>
// where
//     [(); WIDTH * HEIGHT]:,
// {
//     /// The visuals of the cursor
//     pub image: ConstBuffer<WIDTH, HEIGHT>,
//     /// The spot where the cursor clicks (shifts image)
//     pub hotspot: (u32, u32),
// }
// #[allow(deprecated)]
// impl<const WIDTH: usize, const HEIGHT: usize> ConstRawCursor<WIDTH, HEIGHT>
// where
//     [(); WIDTH * HEIGHT]:,
// {
//     #[must_use]
//     /// Create a new raw cursor, returning None if the hotspot is outside the image bounds
//     pub const fn new(
//         image: ConstBuffer<WIDTH, HEIGHT>,
//         hotspot: (u32, u32),
//     ) -> Option<Self> {
//         if hotspot.0 as usize >= WIDTH || hotspot.1 as usize >= HEIGHT {
//             None
//         } else {
//             Some(Self {
//                 image,
//                 hotspot,
//             })
//         }
//     }
// }
