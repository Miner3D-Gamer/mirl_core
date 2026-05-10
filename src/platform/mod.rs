/// Represents digital keys using `KeyCodes` of which there should be plenty enough to pretty all libraries that use their own `KeyCodes`
pub mod keycodes;
/// Mouse related items
pub mod mouse;

pub use keycodes::KeyCode;
pub use mouse::{
    ButtonState, CursorStyle, MouseButton, MouseButtonState, MouseSnapShot,
};
/// Mouse manager, check/set if and when mouse clicks happen
pub mod mouse_manager;
pub use mouse_manager::MouseManager;
/// Keyboard manager, check/set if and when keys are pressed
pub mod keyboard_manager;
pub use keyboard_manager::KeyManager;

/// Fs related stuff
pub mod file_system;
