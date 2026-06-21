/// Represents digital keys using `KeyCodes` of which there should be plenty enough to pretty all libraries that use their own `KeyCodes`
pub mod keycodes;
// /// Mouse related items
// pub mod mouse;

pub use keycodes::KeyCode;

/// Keyboard manager, check/set if and when keys are pressed
pub mod keyboard_manager;
pub use keyboard_manager::KeyboardState;

/// Fs related stuff
pub mod file_system;
