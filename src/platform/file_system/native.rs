// use std::io::Read;

// use crate::platform::file_system::{
//     file_data::DataType, file_system_traits::FileSystemTrait, FileData,
// };

// // #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// /// Implementation of `FileSystem` for Native use only
// /// This is supposed to make accessing files easier
// #[derive(Debug, Clone, PartialEq, Eq)]
// #[cfg_attr(feature = "c_compatible", repr(C))]
// pub struct NativeFileSystem {
//     exe_path: std::path::PathBuf,
//     src_path: Option<std::path::PathBuf>,
// }
// impl NativeFileSystem {
//     /// # Errors
//     /// When the current exe does not have a parent path, or the current exe does not exist. Both of which should not be possible under normal circumstances
//     pub fn new(//required_files: Vec<&'static str>,
//     ) -> Result<Self, Box<dyn core::error::Error>> {
//         let temp = std::env::current_exe()?;
//         let exe_path =
//             temp.parent().ok_or("No parent for current_exe")?.to_path_buf();

//         let src_path;
//         if let Some(build_folder) = exe_path.parent() {
//             if let Some(src_parent) = build_folder.parent() {
//                 if let Some(x) = src_parent.parent() {
//                     src_path = Some(x.join("src"));
//                 } else {
//                     src_path = None;
//                 }
//             } else {
//                 src_path = None;
//             }
//         } else {
//             src_path = None;
//         }

//         let file_system = Self {
//             exe_path,
//             src_path,
//         };

//         // for i in required_files {
//         //     assert!(
//         //         file_system.does_file_exist(i),
//         //         "Searched: {:?} but could not find {}",
//         //         file_system.get_searched_folders(),
//         //         i
//         //     );
//         // }

//         Ok(file_system)
//     }
// }

// fn get_file_contents<P: core::convert::AsRef<std::path::Path>>(
//     path: P,
// ) -> Result<FileData, Box<dyn core::error::Error>> {
//     let mut file = std::fs::File::open(path)?;
//     let mut buffer = Vec::new();
//     file.read_to_end(&mut buffer)?;
//     Ok(FileData::from_bytes(buffer, DataType::Bytes))
// }

// impl FileSystemTrait for NativeFileSystem {
//     fn get_file_contents(
//         &self,
//         path: &str,
//     ) -> Result<FileData, Box<dyn core::error::Error>> {
//         if let Ok(contents) = get_file_contents(path) {
//             return Ok(contents);
//         }
//         if let Ok(contents) = get_file_contents(self.exe_path.join(path)) {
//             return Ok(contents);
//         }
//         if let Ok(val) = std::env::current_dir() {
//             if let Ok(contents) = get_file_contents(val.join(path)) {
//                 return Ok(contents);
//             }
//         }
//         if let Ok(contents) = get_file_contents(
//             self.src_path.clone().unwrap_or_default().join(path),
//         ) {
//             return Ok(contents);
//         }
//         Err(Box::new(std::io::Error::new(
//             std::io::ErrorKind::NotFound,
//             "Requested file could not be found",
//         )))
//     }

//     fn write_to_file(
//         &self,
//         path: &str,
//         contents: &[u8],
//     ) -> std::io::Result<()> {
//         std::fs::write(self.exe_path.join(path), contents)?;
//         Ok(())
//     }
//     fn get_files_in_folder(&self, path: &str) -> Vec<String> {
//         let mut files = Vec::new();
//         files.extend(get_elements_in_folder::<_, false>(path));
//         if !files.is_empty() {
//             return files;
//         }
//         files.extend(get_elements_in_folder::<_, false>(
//             self.exe_path.join(path),
//         ));
//         if !files.is_empty() {
//             return files;
//         }
//         if let Ok(val) = std::env::current_dir() {
//             files.extend(get_elements_in_folder::<_, false>(val.join(path)));
//             if !files.is_empty() {
//                 return files;
//             }
//         }

//         files.extend(get_elements_in_folder::<_, false>(
//             self.src_path.clone().unwrap_or_default().join(path),
//         ));

//         files
//     }

//     fn get_folders_in_folder(&self, path: &str) -> Vec<String> {
//         let mut folders = Vec::new();
//         folders.extend(get_elements_in_folder::<_, true>(path));
//         if !folders.is_empty() {
//             return folders;
//         }
//         folders.extend(get_elements_in_folder::<_, true>(
//             self.exe_path.join(path),
//         ));
//         if !folders.is_empty() {
//             return folders;
//         }

//         if let Ok(val) = std::env::current_dir() {
//             folders.extend(get_elements_in_folder::<_, true>(val.join(path)));
//             if !folders.is_empty() {
//                 return folders;
//             }
//         }
//         folders.extend(get_elements_in_folder::<_, true>(
//             self.src_path.clone().unwrap_or_default().join(path),
//         ));

//         folders
//     }
//     fn join(&self, path1: &str, path2: &str) -> String {
//         std::path::Path::new(path1).join(path2).to_string_lossy().to_string()
//     }
//     fn does_file_exist(&self, path: &str) -> bool {
//         if std::fs::metadata(self.exe_path.join(path)).is_ok() {
//             return true;
//         }
//         if let Some(src_path) = &self.src_path {
//             return std::fs::metadata(src_path.join(path)).is_ok();
//         }
//         false
//     }
//     fn does_folder_exist(&self, path: &str) -> bool {
//         if std::fs::exists(self.exe_path.join(path)).is_ok() {
//             return true;
//         }
//         if let Some(src_path) = &self.src_path {
//             return std::fs::exists(src_path.join(path)).is_ok();
//         }
//         false
//     }
//     fn get_searched_folders(&self) -> Vec<String> {
//         let mut vec = Vec::new();

//         vec.push(self.exe_path.to_str().unwrap_or_default().to_string());
//         if let Some(value) = &self.src_path {
//             vec.push(value.to_str().unwrap_or_default().to_string());
//         }
//         vec
//     }
// }

// fn get_elements_in_folder<
//     P: core::convert::AsRef<std::path::Path>,
//     const FOLDERS: bool,
// >(
//     path: P,
// ) -> Vec<String> {
//     let mut folders = Vec::new();

//     if let Ok(entries) = std::fs::read_dir(path) {
//         for entry in entries.filter_map(Result::ok) {
//             let entry_path = entry.path();
//             if (entry_path.is_dir() && FOLDERS)
//                 || (!FOLDERS && entry_path.is_file())
//             {
//                 if let Some(folder_name) = entry_path.file_name() {
//                     folders.push(folder_name.to_string_lossy().to_string());
//                 }
//             }
//         }
//     }
//     folders
// }
