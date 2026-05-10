
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
/// A struct to manage the pressed mouse keys + scroll
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct MouseManager {
    scroll_x: f32,
    scroll_y: f32,
    left_mouse_button: bool,
    right_mouse_button: bool,
    middle_mouse_button: bool,
    // Other mouse buttons not supported due to me not having a mouse that supports them
    extra1_button: bool,
    extra2_button: bool,
    // Never mind
    extra3_button: bool,
    extra4_button: bool,
}

impl Default for MouseManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MouseManager {
    /// Create a new `MouseManager` instance, keep in mind that it doesn't check if any keys are down/scroll itself
    #[must_use]
    pub const fn new() -> Self {
        Self {
            scroll_x: 0.0,
            scroll_y: 0.0,
            left_mouse_button: false,
            right_mouse_button: false,
            middle_mouse_button: false,
            extra1_button: false,
            extra2_button: false,
            extra3_button: false,
            extra4_button: false,
        }
    }
    /// Set the scroll
    pub const fn set_scroll(&mut self, xy: (f32, f32)) {
        self.scroll_x = xy.0;
        self.scroll_y = xy.1;
    }
    /// Add to the scroll
    pub fn add_scroll(&mut self, xy: (f32, f32)) {
        self.scroll_x += xy.0;
        self.scroll_y += xy.1;
    }
    /// Reset the scroll
    pub const fn reset_scroll(&mut self) {
        self.scroll_x = 0.0;
        self.scroll_y = 0.0;
    }
    #[must_use]
    /// Get the mouse wheel scroll
    pub const fn get_scroll(&self) -> (f32, f32) {
        (self.scroll_x, self.scroll_y)
    }
    #[must_use]
    /// Check if a mouse button is pressed
    pub const fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        map_button(button, self)
    }
    /// Set the state of a mouse button
    pub const fn set_mouse_button_state(
        &mut self,
        button: MouseButton,
        value: bool,
    ) {
        set_mouse_button(button, self, value);
    }
}

use crate::platform::mouse::MouseButton;
#[must_use]
/// Get the value [`MouseButton`] of [`MouseManager`]
pub const fn map_button(
    button: MouseButton,
    mouse_manager: &MouseManager,
) -> bool {
    match button {
        MouseButton::Left => mouse_manager.left_mouse_button,
        MouseButton::Right => mouse_manager.right_mouse_button,
        MouseButton::Middle => mouse_manager.middle_mouse_button,
        MouseButton::Extra1 => mouse_manager.extra1_button,
        MouseButton::Extra2 => mouse_manager.extra2_button,
        MouseButton::Extra3 => mouse_manager.extra3_button,
        MouseButton::Extra4 => mouse_manager.extra4_button,
        MouseButton::Unsupported => false,
    }
}
/// Set the value [`MouseButton`] of [`MouseManager`]
pub const fn set_mouse_button(
    button: MouseButton,
    mouse_manager: &mut MouseManager,
    value: bool,
) {
    match button {
        MouseButton::Left => mouse_manager.left_mouse_button = value,
        MouseButton::Right => mouse_manager.right_mouse_button = value,
        MouseButton::Middle => mouse_manager.middle_mouse_button = value,
        MouseButton::Extra1 => mouse_manager.extra1_button = value,
        MouseButton::Extra2 => mouse_manager.extra2_button = value,
        MouseButton::Extra3 => mouse_manager.extra3_button = value,
        MouseButton::Extra4 => mouse_manager.extra4_button = value,
        MouseButton::Unsupported => (),
    }
}
