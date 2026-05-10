#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[allow(clippy::derived_hash_with_manual_eq)] // It is manually derived for the const tag.
#[derive(Debug, Clone, Copy, Hash)]
/// A 2d position in a text context
pub struct TextPosition {
    /// Vertical
    pub line: usize,
    /// Horizontal
    pub column: usize,
}
impl const Eq for TextPosition {}
impl const PartialEq for TextPosition {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.column == other.column
    }
}

#[cfg(feature = "std")]
impl const Ord for TextPosition {
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self < other {
            self
        } else {
            other
        }
    }
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self > other {
            self
        } else {
            other
        }
    }
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        self.min(min).max(max)
    }
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if self.line > other.line {
            core::cmp::Ordering::Greater
        } else if self.line < other.line {
            core::cmp::Ordering::Less
        } else if self.column > other.column {
            core::cmp::Ordering::Greater
        } else if self.column < other.column {
            core::cmp::Ordering::Less
        } else {
            core::cmp::Ordering::Equal
        }
    }
}

#[cfg(feature = "std")]
impl const PartialOrd for TextPosition {
    fn ge(&self, other: &Self) -> bool {
        self.gt(other) || self == other
    }
    fn gt(&self, other: &Self) -> bool {
        match self.line.cmp(&other.line) {
            core::cmp::Ordering::Greater => true,
            core::cmp::Ordering::Equal => self.column > other.column,
            core::cmp::Ordering::Less => false,
        }
    }

    fn lt(&self, other: &Self) -> bool {
        match self.line.cmp(&other.line) {
            core::cmp::Ordering::Greater => false,
            core::cmp::Ordering::Equal => self.column < other.column,
            core::cmp::Ordering::Less => true,
        }
    }
    fn le(&self, other: &Self) -> bool {
        self.lt(other) || self == other
    }
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TextPosition {
    #[allow(missing_docs)]
    #[must_use]
    pub const fn new(line: usize, column: usize) -> Self {
        Self {
            line,
            column,
        }
    }
    /// Move column by one
    pub const fn advance_char(&mut self) {
        self.column += 1;
    }
    /// Move column by one
    pub const fn advance_char_by(&mut self, by: usize) {
        self.column += by;
    }
    /// Move line by one and reset column
    pub const fn advance_line(&mut self) {
        self.line += 1;
        self.column = 0;
    }
    /// Move line by one and reset column
    pub const fn advance_line_by(&mut self, by: usize) {
        self.line += by;
        self.column = 0;
    }
    #[must_use]
    /// Get the line and offset from the given offset
    pub fn from_offset(offset: usize, text: &str) -> Self {
        let (line, col) = line_and_column_from_offset(offset, text);

        Self::new(line, col)
    }
    #[must_use]
    /// Get the text offset from the stored line and column
    pub fn offset_from_pos(&self, text: &str) -> Option<usize> {
        offset_from_line_and_column(self.line, self.column, text)
    }
    #[must_use]
    /// Get the character at the stored pos of the given text
    pub fn get_char_at_pos(&self, text: &str) -> Option<char> {
        text.chars().nth(offset_from_line_and_column(
            self.line,
            self.column,
            text,
        )?)
    }
}

impl Default for TextPosition {
    fn default() -> Self {
        Self {
            line: usize::MAX,
            column: usize::MAX,
        }
    }
}

// #[cfg_attr(feature = "std", derive(PartialOrd, Ord))]
// /// A text position range
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// #[cfg_attr(feature = "c_compatible", repr(C))]
// pub struct PositionRange {
//     /// The start of the range
//     pub start: TextPosition,
//     /// The end of the range
//     pub end: TextPosition,
// }

// impl PositionRange {
//     /// Create a new range based on start and end points
//     #[must_use]
//     pub const fn new(start: TextPosition, end: TextPosition) -> Self {
//         Self {
//             start,
//             end,
//         }
//     }
// }
#[must_use]
/// Get the line and offset from the given offset
pub fn line_and_column_from_offset(
    offset: usize,
    text: &str,
) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;

    for (i, ch) in text.char_indices() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    (line, col)
}

#[must_use]
/// Get the offset from the given line and column
pub fn offset_from_line_and_column(
    line: usize,
    col: usize,
    text: &str,
) -> Option<usize> {
    let mut current_line = 1;
    let mut current_col = 1;

    for (i, ch) in text.char_indices() {
        if current_line == line && current_col == col {
            return Some(i);
        }

        if ch == '\n' {
            current_line += 1;
            current_col = 1;
        } else {
            current_col += 1;
        }
    }

    if current_line == line && current_col == col {
        return Some(text.len());
    }

    None
}
