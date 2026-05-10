/// Convert the corner type from [`mirl::math::collision::rectangle::Rectangle::get_edge_position`](crate::math::collision::rectangle::Rectangle::get_edge_position) into the appropriate cursor style
#[must_use]
pub const fn corner_type_to_cursor_style(
    corner: u8,
) -> Option<crate::platform::CursorStyle> {
    match corner {
        0 | 4 => Some(crate::platform::CursorStyle::ResizeNWSE),
        1 | 5 => Some(crate::platform::CursorStyle::ResizeVertically),
        2 | 6 => Some(crate::platform::CursorStyle::ResizeNESW),
        3 | 7 => Some(crate::platform::CursorStyle::ResizeHorizontally),
        _ => None,
    }
}

// /// Get the relative direction from the target to the current [`Rectangle`](crate::math::collision::Rectangle)
// #[must_use]
// pub fn direction_rect_to_rect<
//     T: Copy
//         + mirl_extensions::ConstOne
//         + core::ops::Add<Output = T>
//         + core::ops::Div<Output = T>
//         + ConstNumbers128
//         + core::cmp::PartialOrd
//         + core::ops::Sub<Output = T>
//         + core::ops::Mul<Output = T>
//         + mirl_extensions::ConstZero
//         + mirl_extensions::Abs,
//     const CS: bool,
// >(
//     current: &crate::math::geometry::Pos2D<
//         T,
//         crate::math::geometry::d2::rectangle::Rectangle<T, CS>,
//     >,
//     target: &crate::math::geometry::Pos2D<
//         T,
//         crate::math::geometry::d2::rectangle::Rectangle<T, CS>,
//     >,
// ) -> crate::directions::Directions {
//     direction_point_to_point::<T, CS>(
//         current.center(),
//         target.center(),
//         target.get_ratio(),
//     )
// }
