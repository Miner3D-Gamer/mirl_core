// use crate::platform::file_system::{
//     file_data::DataType, file_system_traits::FileSystemTrait, FileData,
// };

// struct DefaultFont {
//     pub bytes: Option<Vec<u8>>,
//     pub path: Option<std::path::PathBuf>,
// }

// // #[cfg(feature = "font_kit")]
// fn get_default_os_font() -> Result<DefaultFont, Box<dyn core::error::Error>> {
//     let source = font_kit::source::SystemSource::new();

//     let handle = source.select_best_match(
//         &[font_kit::family_name::FamilyName::SansSerif],
//         &font_kit::properties::Properties::new(),
//     )?;

//     match handle {
//         font_kit::handle::Handle::Path {
//             path,
//             font_index: _,
//         } => Ok(DefaultFont {
//             bytes: None,
//             path: Some(path),
//         }),
//         font_kit::handle::Handle::Memory {
//             bytes,
//             font_index: _,
//         } => Ok(DefaultFont {
//             bytes: Some(bytes.as_ref().clone()),
//             path: None,
//         }),
//     }
// }

// /// Get the default font the os is using TODO: Turn this into a trait and maybe drop `font_kit` dependency?
// ///
// /// # Errors
// /// When no default font could be found
// pub fn get_default_font<T: FileSystemTrait>(
//     file_system: &T,
// ) -> Result<FileData, Box<dyn core::error::Error>> {
//     let thing = get_default_os_font()?;
//     if let Some(data) = thing.bytes {
//         Ok(FileData::from_bytes(data, DataType::Font))
//     } else if let Some(path) = thing.path {
//         let mut file_data = file_system
//             .get_file_contents(&path.as_os_str().to_string_lossy())?;
//         file_data.expected_data_type = DataType::Font;
//         Ok(file_data)
//     } else {
//         Err(Box::new(std::io::Error::from(std::io::ErrorKind::NotFound)))
//     }
// }
