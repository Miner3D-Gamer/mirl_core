use crate::{Buffer, ConstBuffer};

impl ButtonState {
    #[must_use]
    /// Create a new button state -> Pressed, clicked, and released are calculated
    pub const fn new(current: bool, last: bool) -> Self {
        Self {
            down: current,
            clicked: current && !last,
            released: !current && last,
        }
    }
    /// Update the current state
    pub const fn update(&mut self, new: bool) {
        self.clicked = !self.down && new;
        self.released = self.down && !new;
        self.down = new;
    }
}

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
/// The current state of a mouse buttons and if they have just been pressed
#[allow(missing_docs, clippy::struct_excessive_bools)]
pub struct ButtonState {
    pub down: bool,
    pub clicked: bool,
    pub released: bool,
}

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
/// The current state of the mouse buttons and if they have just been pressed
pub struct MouseButtonState {
    pub left: ButtonState,
    pub middle: ButtonState,
    pub right: ButtonState,
}
impl MouseButtonState {
    #[must_use]
    /// Create a new button state using the current mouse state, assumes that the buttons have been released last frame
    pub const fn new_default(
        left_down: bool,
        middle_down: bool,
        right_down: bool,
    ) -> Self {
        Self {
            left: ButtonState::new(left_down, false),
            middle: ButtonState::new(middle_down, false),
            right: ButtonState::new(right_down, false),
        }
    }
    #[must_use]
    /// Create a new button state using the current mouse state, assumes that the buttons have been released last frame
    pub const fn new(
        left_down: bool,
        middle_down: bool,
        right_down: bool,
        previous_left_down: bool,
        previous_middle_down: bool,
        previous_right_down: bool,
    ) -> Self {
        Self {
            left: ButtonState::new(left_down, previous_left_down),
            middle: ButtonState::new(middle_down, previous_middle_down),
            right: ButtonState::new(right_down, previous_right_down),
        }
    }
    /// Updates the mouse button states
    pub const fn update(
        &mut self,
        left_down: bool,
        middle_down: bool,
        right_down: bool,
    ) {
        self.left.update(left_down);
        self.middle.update(middle_down);
        self.right.update(right_down);
    }
    /// Updates the mouse button states using the current mouse snapshot
    pub const fn update_with_snapshot(&mut self, snapshot: &MouseSnapShot) {
        self.update(
            snapshot.left_down,
            snapshot.middle_down,
            snapshot.right_down,
        );
    }
}

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, Copy, PartialEq, Default, PartialOrd)]
/// A data storage helper holding the current state of the mouse
pub struct MouseSnapShot {
    /// The current position of the mouse
    pub position: Option<(f32, f32)>,
    /// The current mouse scroll
    pub scroll: (f32, f32),
    /// If the left mouse button is down
    pub left_down: bool,
    /// If the right mouse button is down
    pub middle_down: bool,
    /// If the right mouse button is down
    pub right_down: bool,
}
impl MouseSnapShot {
    #[must_use]
    /// Convert the current snapshot into a proper
    pub const fn to_mouse_button_state(
        &self,
        previous_left: bool,
        previous_middle: bool,
        previous_right: bool,
    ) -> MouseButtonState {
        MouseButtonState::new(
            self.left_down,
            self.middle_down,
            self.right_down,
            previous_left,
            previous_middle,
            previous_right,
        )
    }
}
impl MouseSnapShot {
    #[must_use]
    /// Sets the current mouse position
    pub const fn set_position(mut self, position: Option<(f32, f32)>) -> Self {
        self.position = position;
        self
    }
    #[must_use]
    /// Sets the current mouse scroll
    pub const fn set_scroll(mut self, scroll: (f32, f32)) -> Self {
        self.scroll = scroll;
        self
    }
    #[must_use]
    /// Sets whether the left mouse button is down
    pub const fn set_left_down(mut self, left_down: bool) -> Self {
        self.left_down = left_down;
        self
    }
    #[must_use]
    /// Sets whether the middle mouse button is down
    pub const fn set_middle_down(mut self, middle_down: bool) -> Self {
        self.middle_down = middle_down;
        self
    }
    #[must_use]
    /// Sets whether the right mouse button is down
    pub const fn set_right_down(mut self, right_down: bool) -> Self {
        self.right_down = right_down;
        self
    }
}
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

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// Things that could go wrong while loading a cursor
pub enum LoadCursorError {
    /// Image data corrupt
    InvalidImageData(String),
    /// Unknown
    Misc(String),
    /// A tempfile could not be created
    UnableToCreateTempfile,
    /// A tempfile could not be removed
    UnableToDeleteTempfile,
    /// Os could not load the cursor even though the cursor data has been constructed
    OsError,
    /// When the hotspot is in an invalid position
    InvalidHotspot,
}


#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
// #[cfg_attr(feature = "wincode", derive(wincode::SchemaWrite, wincode::SchemaRead))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The resolution/size of the given cursor
///
/// If a given resolution is not supported, the next smaller is tried
pub enum CursorResolution {
    /// 16x16
    X16,
    /// 32x32
    X32,
    /// 64x64
    X64,
    /// 128x128
    X128,
    /// 256x256
    X256,
}


#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
/// Supported (and unsupported) mouse buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MouseButton {
    /// ✨ The left mouse button ✨
    Left,
    /// ✨ The right mouse button ✨
    Right,
    /// ✨ The button between the left and right mouse buttons ✨
    Middle,
    /// An extra niche button some mice have
    Extra1,
    /// Another extra niche button some mice have
    Extra2,
    /// A freakish amalgamation of human invention
    Extra3,
    /// No one should be allowed this much power.
    Extra4,
    /// You can't expect to be able to expect everything ¯\_(ツ)_/¯
    Unsupported,
}
#[cfg(feature = "std")]
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
/// A raw representation of a cursor, the hotspot will never be outside the image bounds
///
/// Manually setting the hotspot/image will shift all safety liabilities to you
pub struct RawCursor {
    /// The visuals of the cursor
    pub image: Buffer,
    /// The spot where the cursor clicks (shifts image)
    pub hotspot: (u32, u32),
}
impl RawCursor {
    #[must_use]
    /// Create a new raw cursor, returning None if the hotspot is outside the image bounds
    pub fn new(image: Buffer, hotspot: (u32, u32)) -> Option<Self> {
        if hotspot.0 as usize >= image.width
            || hotspot.1 as usize >= image.height
        {
            None
        } else {
            Some(Self {
                image,
                hotspot,
            })
        }
    }
}
#[deprecated = "This struct is experimental"]
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
/// A raw representation of a cursor, the hotspot will never be outside the image bounds
///
/// Manually setting the hotspot/image will shift all safety liabilities to you
pub struct ConstRawCursor<const WIDTH: usize, const HEIGHT: usize>
where
    [(); WIDTH * HEIGHT]:,
{
    /// The visuals of the cursor
    pub image: ConstBuffer<WIDTH, HEIGHT>,
    /// The spot where the cursor clicks (shifts image)
    pub hotspot: (u32, u32),
}
#[allow(deprecated)]
impl<const WIDTH: usize, const HEIGHT: usize> ConstRawCursor<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    #[must_use]
    /// Create a new raw cursor, returning None if the hotspot is outside the image bounds
    pub const fn new(
        image: ConstBuffer<WIDTH, HEIGHT>,
        hotspot: (u32, u32),
    ) -> Option<Self> {
        if hotspot.0 as usize >= WIDTH || hotspot.1 as usize >= HEIGHT {
            None
        } else {
            Some(Self {
                image,
                hotspot,
            })
        }
    }
}


#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
/// A cursor style, what else to say?
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CursorStyle {
    /// Default Pointer
    Default,
    /// Open hand
    HandOpen,
    /// Closed hand
    HandClosed,
    /// Default cursor with an extra arrow (e.g. clickable text)
    Alias,
    /// Resize vertically + Resize horizontally
    AllScroll,
    /// Arrow pointing to the bottom left ⬋
    ArrowBottomLeft,
    /// Arrow pointing to the bottom right ⬊
    ArrowBottomRight,
    /// Arrow down with a _ at the end
    SideBottom,
    /// A plus shape
    Cell,
    /// Default cursor rotated to be vertical
    CenteredPointer,
    /// Horizontal resizing
    ResizeHorizontally,
    /// Eyedropper
    ColorPicker,
    /// Default cursor with ≡ attached
    ContextMenu,
    /// Default cursor with a plus
    Copy,
    /// Cross
    Crosshair,
    /// Closed hand with an 🚫 attached
    HandClosedNoDrop,
    /// Arrow pointing down
    ArrowDown,
    /// Tip of an ink pen
    Draft,
    /// Small pointers in all directions like this: ◄ ►
    Fleur,
    /// Question mark
    Help,
    /// Arrow pointing left
    ArrowLeft,
    /// Arrow left with a stopper |←
    SideLeft,
    /// Default cursor with a 🚫 attached
    NoDrop,
    /// "🚫"
    NotAllowed,
    /// A Pencil
    Pencil,
    /// Skull
    Pirate,
    /// Hand with pointing index finger
    Pointer,
    /// Arrow pointing right
    ArrowRight,
    /// Mirrored version of normal cursor
    MirroredPointer,
    /// Arrow pointing right with a stopper →|
    SideRight,
    /// Resize top right to bottom left
    ResizeNESW,
    /// Resize top left to bottom right
    ResizeNWSE,
    /// Resize horizontally
    SizeHor,
    /// Resize vertically
    ResizeVertically,
    /// I Beam
    Text,
    /// Arrow pointing up top left
    ArrowTopLeft,
    /// Arrow pointing up top right
    ArrowTopRight,
    /// Arrow pointing up with an _ on top
    SideTop,
    /// Arrow pointing up
    ArrowUp,
    /// I Beam rotated 90°
    VerticalText,
    /// Magnifying glass with plus
    ZoomIn,
    /// Magnifying glass with minus
    ZoomOut,
}
