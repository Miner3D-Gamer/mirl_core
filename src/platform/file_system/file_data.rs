use crate::graphics::{ColorManipulation, rgb_u8_to_u32, rgba_u8_to_u32};

// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[cfg(feature = "image")]
/// This struct hold the raw data of a file to be converted/used somewhere else
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "c_compatible", repr(C))]
pub struct FileData {
    /// Raw data
    pub raw_data: Vec<u8>,
    /// Path to get raw data
    pub expected_data_type: DataType,
}
impl FileData {
    #[allow(clippy::useless_format)]
    /// Creates a format that is just a little more pleasant to the eye
    #[must_use]
    pub fn to_printable(&self) -> String {
        match self.expected_data_type {
            DataType::Text => format!("Text: {:?}", self.to_string()),
            #[cfg(feature = "font_support")]
            DataType::Font => self.to_font().map_or_else(
                |_| "Not a font.".into(),
                |font| format!("Font: {font:?}"),
            ),
            #[cfg(feature = "image")]
            DataType::Image => format!("Bytes: {:?}", self.to_image()),
            DataType::Audio => format!("Audio: {:?}", "<Unsupported>"),
            DataType::ListOfText => {
                format!("List of text: {{Preview not available}}")
            }
            DataType::Color => self.to_color().map_or_else(
                || "Not a color.".into(),
                |color| {
                    format!(
                        "Color: {:?} | r{} g{} b{} a{}",
                        color,
                        color.red(),
                        color.green(),
                        color.blue(),
                        color.alpha()
                    )
                },
            ),
            _ => format!("Bytes: {:?}", self.as_bytes()),
        }
    }
}
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
/// What type the data is expected to be
///
/// If you don't know what type the data is, use [`Bytes`](Self::Bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "c_compatible", repr(C))]
pub enum DataType {
    #[default]
    /// Raw bytes/custom
    Bytes,
    /// Text/Strings
    Text,
    /// Font - Requires the `font_support` feature
    #[cfg(feature = "font_support")]
    Font,
    /// Image/Buffer - Requires the `image` feature
    #[cfg(feature = "image")]
    Image,
    /// Not supported
    Audio,
    /// As a list of strings/file paths
    ListOfText,
    /// As a color
    Color,
}
impl FileData {
    #[must_use]
    /// Constructor to load data from raw bytes
    pub const fn from_bytes(
        data: Vec<u8>,
        expected_data_type: DataType,
    ) -> Self {
        Self {
            raw_data: data,
            expected_data_type,
        }
    }
    #[must_use]
    /// Constructor to load data from raw bytes
    pub const fn from_string(data: String) -> Self {
        Self::from_bytes(data.into_bytes(), DataType::Text)
    }
}

impl FileData {
    /// Convert the raw bytes to a String (if valid UTF-8)
    ///
    /// # Errors
    /// If the data is not in utf8 format
    pub fn to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.raw_data.clone())
    }
    // #[must_use]
    // /// Convert the raw bytes to a Number (assumes data is in a binary format like little-endian)
    // pub fn as_number(&self) -> Result<i64, &'static str> {
    //     if self.raw_data.len() < 8 {
    //         return Err(
    //             "Not enough data",
    //         );
    //     }
    //     let number = i64::from_le_bytes(self.raw_data[0..8].try_into()?);
    //     Ok(number)
    // }
    #[cfg(feature = "font_support")]
    /// Convert the raw bytes to a [`fontdue::Font`] if possible
    ///
    /// # Errors
    /// When not a font it will error
    pub fn to_font(
        &self,
    ) -> Result<fontdue::Font, Box<dyn core::error::Error>> {
        let font = fontdue::Font::from_bytes(
            self.raw_data.clone(),
            fontdue::FontSettings::default(),
        )?;
        Ok(font)
    }
    /// Convert the raw bytes into an `image::DynamicImage` instance
    ///
    /// # Errors
    /// When unable to load the image from memory
    #[cfg(feature = "image")]
    pub fn to_image(
        &self,
    ) -> Result<image::DynamicImage, Box<dyn core::error::Error>> {
        // Decode the raw bytes as an image
        let img = image::load_from_memory(&self.raw_data)?;

        Ok(img)
    }
    #[must_use]
    /// Get raw bytes
    pub const fn as_bytes(&self) -> &Vec<u8> {
        &self.raw_data
    }

    /// Get the data as a color:
    /// One byte: Gray scale is assumed
    /// Two bytes: Gray scale with alpha assumed
    /// Three bytes: RGB assumed
    /// Four bytes: RGBA assumed
    #[must_use]
    pub fn to_color(&self) -> Option<u32> {
        Some(match self.raw_data.len() {
            1 => rgb_u8_to_u32(
                self.raw_data[0],
                self.raw_data[0],
                self.raw_data[0],
            ),
            2 => rgba_u8_to_u32(
                self.raw_data[0],
                self.raw_data[0],
                self.raw_data[0],
                self.raw_data[1],
            ),
            3 => rgb_u8_to_u32(
                self.raw_data[0],
                self.raw_data[1],
                self.raw_data[2],
            ),
            4 => rgba_u8_to_u32(
                self.raw_data[0],
                self.raw_data[1],
                self.raw_data[2],
                self.raw_data[3],
            ),
            _ => return None,
        })
    }
}
