mod native;
pub use crate::platform::*;
// pub use native::*;

// /// Why bother reading files if you can't process them? Let [`file_data::FileData`] fix that.
// mod file_data;
// pub use file_data::{GenericDataType, BinaryData};

// #[cfg(feature = "font_support")]
// #[cfg(not(target_arch = "wasm32"))]
// mod get_os_font;
// #[cfg(feature = "font_support")]
// #[cfg(not(target_arch = "wasm32"))]
// pub use get_os_font::get_default_font;

// /// The trait used by the file system implementations
// pub mod file_system_traits;
// #[cfg(not(target_arch = "wasm32"))]
// pub use native::NativeFileSystem as FileSystem;

// #[cfg(feature = "font_support")]
// #[cfg(not(target_arch = "wasm32"))]
// impl FileSystem {
//     #[must_use]
//     /// Get the default font the os uses
//     pub fn get_default_font(&self) -> Option<fontdue::Font> {
//         get_default_font(self).ok().and_then(|x| x.to_font().ok())
//     }
//     #[must_use]
//     /// Get the default font the os uses
//     pub fn get_default_font_file(&self) -> Option<FileData> {
//         get_default_font(self).ok()
//     }
// }
