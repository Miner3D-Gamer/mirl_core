
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
// #[cfg_attr(feature = "wincode", derive(wincode::SchemaWrite, wincode::SchemaRead))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
/// Key code to be interpreted anywhere
#[allow(missing_docs)]
pub enum KeyCode {
    // Letters
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,

    // Numbers
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    KeyPad0,
    KeyPad1,
    KeyPad2,
    KeyPad3,
    KeyPad4,
    KeyPad5,
    KeyPad6,
    KeyPad7,
    KeyPad8,
    KeyPad9,

    // Function Keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    // Modifiers
    LeftShift,
    RightShift,
    LeftControl,
    RightControl,
    LeftAlt,
    RightAlt,
    LeftSuper,
    RightSuper,
    LeftHyper,
    RightHyper,
    AltControl,

    // Symbols / Punctuation
    Space,
    Enter,
    Escape,
    Backspace,
    Tab,
    BackTab,
    Comma,
    Period,
    Minus,
    Equal,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Quote,
    Tilde,
    Slash,
    Grave,
    Apostrophe,

    // Arrow keys
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,

    // Editing keys
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,

    // Lock keys
    CapsLock,
    NumLock,
    ScrollLock,

    // Keypad operations
    KeyPadDivide,
    KeyPadMultiply,
    KeyPadSubtract,
    KeyPadAdd,
    KeyPadDecimal,
    KeyPadEnter,

    // International & special characters
    KeyAUmlautÄ,
    KeyUUmlautÜ,
    KeyOUmlautÖ,
    KeySS,
    KeyACircumflexÂ,
    KeyUAcuteÚ,
    KeyOCircumflexÔ,
    KeyICircumflexÎ,
    KeyECircumflexÊ,
    KeyEthÐ,
    KeyOELigatureŒ,
    KeyAAcuteÁ,
    KeyYAcuteÝ,
    KeyIUmlautÏ,
    KeyNTildeÑ,
    KeyOGraveÒ,
    KeyUGraveÙ,
    KeyARingÅ,
    KeyAELigatureÆ,
    KeyOSlashØ,
    KeyIGraveÌ,
    KeyThornÞ,

    // Multimedia keys
    // TODO: when checking for MediaPlayPause, also check MediaPlay and MediaPause
    MediaPlayPause,
    MediaPlay,
    MediaPause,
    MediaStop,
    MediaNext,
    MediaPrev,
    VolumeUp,
    VolumeDown,
    Mute,
    MediaReverse,
    MediaFastForward,
    MediaRecord,

    // Browser/OS keys
    BrowserBack,
    BrowserForward,
    BrowserRefresh,
    BrowserHome,
    LaunchMail,
    LaunchApp1,
    LaunchApp2,

    // Platform-specific
    Menu,
    PrintScreen,
    Pause,
    Application,

    // Convenient
    AnyShift,
    AnyAlt,
    AnyControl,
    AnySuper,

    // what
    F25,
    KeyPadEqual,
    World1,
    World2,
    SpecialControl,
    #[default]
    Unknown,
}

macro_rules! define_keys {
    ( $( $ident:ident => $name:expr ),* $(,)? ) => {
        // compile_error!("Hi");
         impl KeyCode {
            /// Converts the requested key to a string representation of itself
            #[must_use]
            pub const fn to_string(&self) -> &'static str {
                match self {
                    $( Self::$ident => $name ),*
                }
            }
            /// Converts the requested key to a string representation of itself
            #[must_use]
            pub const fn str_to_keycode(string: &str) -> Self {
                match string {
                    $( $name => Self::$ident ),*,
                    _=>Self::Unknown
                }
            }
            #[must_use]
            #[cfg(feature = "std")]
            /// Converts self to `[Vec<Self>]`
            pub fn to_vec(&self) -> Vec<Self> {
                Vec::from([*self])
            }
        /// All available keys in string forming in a list
        pub const AVAILABLE_KEY_NAMES: &[&str] = &[
            $( $name ),*
        ];

        /// All available keys in [`KeyCode`] forming a list
        pub const AVAILABLE_KEYS: &[KeyCode] =&[
            $( KeyCode::$ident ),*];

        /// Get all available keys string form in a list
        #[must_use] pub const fn get_available_key_names() -> &'static [&'static str] {
            Self::AVAILABLE_KEY_NAMES
        }
        /// Get all available keys in [`KeyCode`] form in a list
        #[must_use] pub const fn get_available_keys() -> &'static [KeyCode] {
           Self::AVAILABLE_KEYS
        }
        }
        #[cfg(feature = "std")]
        impl StringToKeyCodes for String {
            fn to_keycodes(&self) -> Vec<KeyCode> {
                let mut list = Vec::new();
                for i in self.chars() {
                    list.push(i.to_keycode())
                }
                list
            }
            fn to_keycode(&self)->KeyCode{
                KeyCode::str_to_keycode(self)


            }
        }#[cfg(feature = "std")]
        impl StringToKeyCodes for char {
            fn to_keycodes(&self) -> Vec<KeyCode> {
                Vec::from([self.to_keycode()])
            }
            fn to_keycode(&self)->KeyCode{
                KeyCode::str_to_keycode(&self.to_string())
            }
        }

    };
}

#[cfg(feature = "std")]
/// Convert from a string to a `Vec<KeyCode>`
pub const trait StringToKeyCodes {
    /// Convert from a string to a `Vec<KeyCode>`
    fn to_keycodes(&self) -> Vec<KeyCode>;
    /// Converts a single text instance into the corresponding `KeyCode`
    fn to_keycode(&self) -> KeyCode;
}

impl KeyCode {
    #[must_use]
    #[allow(clippy::too_many_lines)]
    /// Convert a keycode to a String -> Letters/Numbers return a String while function keys return None
    pub const fn to_user_friendly_string(&self) -> Option<&'static str> {
        match self {
            Self::KeyA => Some("A"),
            Self::KeyB => Some("B"),
            Self::KeyC => Some("C"),
            Self::KeyD => Some("D"),
            Self::KeyE => Some("E"),
            Self::KeyF => Some("F"),
            Self::KeyG => Some("G"),
            Self::KeyH => Some("H"),
            Self::KeyI => Some("I"),
            Self::KeyJ => Some("J"),
            Self::KeyK => Some("K"),
            Self::KeyL => Some("L"),
            Self::KeyM => Some("M"),
            Self::KeyN => Some("N"),
            Self::KeyO => Some("O"),
            Self::KeyP => Some("P"),
            Self::KeyQ => Some("Q"),
            Self::KeyR => Some("R"),
            Self::KeyS => Some("S"),
            Self::KeyT => Some("T"),
            Self::KeyU => Some("U"),
            Self::KeyV => Some("V"),
            Self::KeyW => Some("W"),
            Self::KeyX => Some("X"),
            Self::KeyY => Some("Y"),
            Self::KeyZ => Some("Z"),
            Self::Num0 | Self::KeyPad0 => Some("0"),
            Self::Num1 | Self::KeyPad1 => Some("1"),
            Self::Num2 | Self::KeyPad2 => Some("2"),
            Self::Num3 | Self::KeyPad3 => Some("3"),
            Self::Num4 | Self::KeyPad4 => Some("4"),
            Self::Num5 | Self::KeyPad5 => Some("5"),
            Self::Num6 | Self::KeyPad6 => Some("6"),
            Self::Num7 | Self::KeyPad7 => Some("7"),
            Self::Num8 | Self::KeyPad8 => Some("8"),
            Self::Num9 | Self::KeyPad9 => Some("9"),
            Self::F1
            | Self::F2
            | Self::F3
            | Self::F4
            | Self::F5
            | Self::F6
            | Self::F7
            | Self::F8
            | Self::F9
            | Self::F10
            | Self::F11
            | Self::F12
            | Self::F13
            | Self::F14
            | Self::F15
            | Self::F16
            | Self::F17
            | Self::F18
            | Self::F19
            | Self::F20
            | Self::F21
            | Self::F22
            | Self::F23
            | Self::F24
            | Self::F25
            | Self::LeftShift
            | Self::RightShift
            | Self::LeftControl
            | Self::RightControl
            | Self::LeftAlt
            | Self::RightAlt
            | Self::LeftSuper
            | Self::RightSuper
            | Self::Escape
            | Self::Backspace
            | Self::UpArrow
            | Self::DownArrow
            | Self::LeftArrow
            | Self::RightArrow
            | Self::Insert
            | Self::Delete
            | Self::Home
            | Self::End
            | Self::PageUp
            | Self::PageDown
            | Self::CapsLock
            | Self::NumLock
            | Self::ScrollLock
            | Self::MediaPlayPause
            | Self::MediaStop
            | Self::MediaNext
            | Self::MediaPrev
            | Self::VolumeUp
            | Self::VolumeDown
            | Self::Mute
            | Self::BrowserBack
            | Self::BrowserForward
            | Self::BrowserRefresh
            | Self::BrowserHome
            | Self::LaunchMail
            | Self::LaunchApp1
            | Self::LaunchApp2
            | Self::Menu
            | Self::PrintScreen
            | Self::Pause
            | Self::Application
            | Self::World1
            | Self::World2
            | Self::AnyAlt
            | Self::AnySuper
            | Self::AnyControl
            | Self::AnyShift
            | Self::Unknown
            | Self::Enter
            | Self::KeyPadEnter
            | Self::BackTab
            | Self::Tab
            | Self::LeftHyper
            | Self::RightHyper
            | Self::AltControl
            | Self::MediaPlay
            | Self::MediaPause
            | Self::MediaReverse
            | Self::MediaFastForward
            | Self::MediaRecord
            | Self::SpecialControl => None,
            Self::Space => Some(" "),
            Self::Comma => Some(","),
            Self::Period | Self::KeyPadDecimal => Some("."),
            Self::Minus | Self::KeyPadSubtract => Some("-"),
            Self::Equal | Self::KeyPadEqual => Some("="),
            Self::LeftBracket => Some("["),
            Self::RightBracket => Some("]"),
            Self::Backslash => Some("\\"),
            Self::Semicolon => Some(";"),
            Self::Quote => Some("\""),
            Self::Tilde => Some("~"),
            Self::Slash | Self::KeyPadDivide => Some("/"),
            Self::Grave => Some("`"),
            Self::Apostrophe => Some("'"),
            Self::KeyPadMultiply => Some("*"),
            Self::KeyPadAdd => Some("+"),
            Self::KeyAUmlautÄ => Some("Ä"),
            Self::KeyUUmlautÜ => Some("Ü"),
            Self::KeyOUmlautÖ => Some("Ö"),
            Self::KeySS => Some("ß"),
            Self::KeyACircumflexÂ => Some("Â"),
            Self::KeyUAcuteÚ => Some("Ú"),
            Self::KeyOCircumflexÔ => Some("Ô"),
            Self::KeyICircumflexÎ => Some("Î"),
            Self::KeyECircumflexÊ => Some("Ê"),
            Self::KeyEthÐ => Some("Ð"),
            Self::KeyOELigatureŒ => Some("Œ"),
            Self::KeyAAcuteÁ => Some("Á"),
            Self::KeyYAcuteÝ => Some("Ý"),
            Self::KeyIUmlautÏ => Some("Ï"),
            Self::KeyNTildeÑ => Some("Ñ"),
            Self::KeyOGraveÒ => Some("Ò"),
            Self::KeyUGraveÙ => Some("Ù"),
            Self::KeyARingÅ => Some("Å"),
            Self::KeyAELigatureÆ => Some("Æ"),
            Self::KeyOSlashØ => Some("Ø"),
            Self::KeyIGraveÌ => Some("Ì"),
            Self::KeyThornÞ => Some("Þ"),
        }
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for KeyCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let value = self.to_string();
        write!(f, "{value}")
    }
}

define_keys!(
            // Letters
            KeyA => "A",
            KeyB => "B",
            KeyC => "C",
            KeyD => "D",
            KeyE => "E",
            KeyF => "F",
            KeyG => "G",
            KeyH => "H",
            KeyI => "I",
            KeyJ => "J",
            KeyK => "K",
            KeyL => "L",
            KeyM => "M",
            KeyN => "N",
            KeyO => "O",
            KeyP => "P",
            KeyQ => "Q",
            KeyR => "R",
            KeyS => "S",
            KeyT => "T",
            KeyU => "U",
            KeyV => "V",
            KeyW => "W",
            KeyX => "X",
            KeyY => "Y",
            KeyZ => "Z",

            // Numbers
            Num0 => "0",
            Num1 => "1",
            Num2 => "2",
            Num3 => "3",
            Num4 => "4",
            Num5 => "5",
            Num6 => "6",
            Num7 => "7",
            Num8 => "8",
            Num9 => "9",
            KeyPad0 => "KeyPad 0",
            KeyPad1 => "KeyPad 1",
            KeyPad2 => "KeyPad 2",
            KeyPad3 => "KeyPad 3",
            KeyPad4 => "KeyPad 4",
            KeyPad5 => "KeyPad 5",
            KeyPad6 => "KeyPad 6",
            KeyPad7 => "KeyPad 7",
            KeyPad8 => "KeyPad 8",
            KeyPad9 => "KeyPad 9",

            // Function Keys
            F1 => "F1",
            F2 => "F2",
            F3 => "F3",
            F4 => "F4",
            F5 => "F5",
            F6 => "F6",
            F7 => "F7",
            F8 => "F8",
            F9 => "F9",
            F10 => "F10",
            F11 => "F11",
            F12 => "F12",
            F13 => "F13",
            F14 => "F14",
            F15 => "F15",
            F16 => "F16",
            F17 => "F17",
            F18 => "F18",
            F19 => "F19",
            F20 => "F20",
            F21 => "F21",
            F22 => "F22",
            F23 => "F23",
            F24 => "F24",

            // Modifiers
            LeftShift => "Left Shift",
            RightShift => "Right Shift",
            LeftControl => "Left Control",
            RightControl => "Right Control",
            LeftAlt => "Left Alt",
            RightAlt => "Right Alt",
            LeftSuper => "Left Super",
            RightSuper => "Right Super",
    LeftHyper=>"Hyper Left",
    RightHyper=>"Hyper Right",
    AltControl=>"Alt Control",

            // Symbols / Punctuation
            Space => "Space",
            Enter => "Enter",
            Escape => "Escape",
            Backspace => "Backspace",
            Tab => "Tab",
            Comma => ",",
            Period => ".",
            Minus => "-",
            Equal => "=",
            LeftBracket => "[",
            RightBracket => "]",
            Backslash => "\\",
            Semicolon => ";",
            Quote => "\"",
            Tilde => "~",
            Slash => "/",
            Grave => "`",
            Apostrophe => "'",

            // Arrow keys
            UpArrow => "Up",
            DownArrow => "Down",
            LeftArrow => "Left",
            RightArrow => "Right",

            // Editing keys
            Insert => "Insert",
            Delete => "Delete",
            Home => "Home",
            End => "End",
            PageUp => "Page Up",
            PageDown => "Page Down",

            // Lock keys
            CapsLock => "Caps Lock",
            NumLock => "Num Lock",
            ScrollLock => "Scroll Lock",

            // Keypad operations
            KeyPadDivide => "KeyPad /",
            KeyPadMultiply => "KeyPad *",
            KeyPadSubtract => "KeyPad -",
            KeyPadAdd => "KeyPad +",
            KeyPadDecimal => "KeyPad .",
            KeyPadEnter => "KeyPad Enter",

            // International & special characters
            KeyAUmlautÄ => "Ä",
            KeyUUmlautÜ => "Ü",
            KeyOUmlautÖ => "Ö",
            KeySS => "ß",
            KeyACircumflexÂ => "Â",
            KeyUAcuteÚ => "Ú",
            KeyOCircumflexÔ => "Ô",
            KeyICircumflexÎ => "Î",
            KeyECircumflexÊ => "Ê",
            KeyEthÐ => "Ð",
            KeyOELigatureŒ => "Œ",
            KeyAAcuteÁ => "Á",
            KeyYAcuteÝ => "Ý",
            KeyIUmlautÏ => "Ï",
            KeyNTildeÑ => "Ñ",
            KeyOGraveÒ => "Ò",
            KeyUGraveÙ => "Ù",
            KeyARingÅ => "Å",
            KeyAELigatureÆ => "Æ",
            KeyOSlashØ => "Ø",
            KeyIGraveÌ => "Ì",
            KeyThornÞ => "Þ",

            // Multimedia keys
            MediaPlayPause => "Media Play/Pause",
            MediaStop => "Media Stop",
            MediaNext => "Media Next",
            MediaPrev => "Media Previous",
            VolumeUp => "Volume Up",
            VolumeDown => "Volume Down",
            Mute => "Mute",
            MediaPause=>"Media Pause",
            MediaPlay=>"Media Play",
            MediaReverse=>"Media Reverse",
            MediaFastForward=>"Media Fast forward",
            MediaRecord=>"Media Record",

            // Browser/OS keys
            BrowserBack => "Browser Back",
            BrowserForward => "Browser Forward",
            BrowserRefresh => "Browser Refresh",
            BrowserHome => "Browser Home",
            LaunchMail => "Launch Mail",
            LaunchApp1 => "Launch App 1",
            LaunchApp2 => "Launch App 2",

            // Platform-specific
            Menu => "Menu",
            PrintScreen => "Print Screen",
            Pause => "Pause",
            Application => "Application",

            AnyShift=>"Shift",
            AnyControl=>"Control",
            AnyAlt=>"Alt",
            AnySuper=>"Super",

            // what
            F25 => "F25",
            KeyPadEqual => "KeyPad =",
            World1 => "World 1",
            World2 => "World 2",
            BackTab=>"BackTab",
            SpecialControl=>"Special Control",
            Unknown => "Unknown",);

#[must_use]
#[cfg(feature = "std")]
/// Convert a list of keycodes into a String and return the ones that weren't convertible
pub fn keycodes_to_str(keycodes: &Vec<KeyCode>) -> (String, Vec<KeyCode>) {
    let mut functions = Vec::new();
    let mut output = String::new();
    for key_code in keycodes {
        let value = key_code.to_user_friendly_string();
        if let Some(key) = value {
            for l in key.chars() {
                output.push(l);
            }
        } else {
            functions.push(*key_code);
        }
    }
    (output, functions)
}
