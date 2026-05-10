#![allow(clippy::unnested_or_patterns)]
#![allow(clippy::cast_ptr_alignment)]

/// A 2d position specialized for lines and columns
pub mod position;

/// A copy of [`core::str::Utf8Error`]
#[derive(Copy, Eq, PartialEq, Clone, Debug)]
pub struct Utf8Error {
    /// Up until where the string is valid
    pub valid_up_to: usize,
    /// Idk
    pub error_len: Option<u8>,
}
impl core::fmt::Display for Utf8Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(error_len) = self.error_len {
            write!(
                f,
                "invalid utf-8 sequence of {} bytes from index {}",
                error_len, self.valid_up_to
            )
        } else {
            write!(
                f,
                "incomplete utf-8 byte sequence from index {}",
                self.valid_up_to
            )
        }
    }
}

impl core::error::Error for Utf8Error {}

use core::intrinsics::const_eval_select;

/// A macro to make it easier to invoke `const_eval_select`. Use as follows:
/// ```rust,ignore (just a macro example)
/// const_eval_select!(
///     @capture { arg1: i32 = some_expr, arg2: T = other_expr } -> U:
///     if const #[attributes_for_const_arm] {
///         // Compile-time code goes here.
///     } else #[attributes_for_runtime_arm] {
///         // Run-time code goes here.
///     }
/// )
/// ```
/// The `@capture` block declares which surrounding variables / expressions can be
/// used inside the `if const`.
/// Note that the two arms of this `if` really each become their own function, which is why the
/// macro supports setting attributes for those functions. Both functions are marked as `#[inline]`.
///
/// See [`const_eval_select()`] for the rules and requirements around that intrinsic.
pub macro const_eval_select {
    (
        @capture$([$($binders:tt)*])? { $($arg:ident : $ty:ty = $val:expr),* $(,)? } $( -> $ret:ty )? :
        if const
            $(#[$compiletime_attr:meta])* $compiletime:block
        else
            $(#[$runtime_attr:meta])* $runtime:block
    ) => {{
        #[inline]
        $(#[$runtime_attr])*
        fn runtime$(<$($binders)*>)?($($arg: $ty),*) $( -> $ret )? {
            $runtime
        }

        #[inline]
        $(#[$compiletime_attr])*
        const fn compiletime$(<$($binders)*>)?($($arg: $ty),*) $( -> $ret )? {
            // Don't warn if one of the arguments is unused.
            $(let _ = $arg;)*

            $compiletime
        }

        const_eval_select(($($val,)*), compiletime, runtime)
    }},
    // We support leaving away the `val` expressions for *all* arguments
    // (but not for *some* arguments, that's too tricky).
    (
        @capture$([$($binders:tt)*])? { $($arg:ident : $ty:ty),* $(,)? } $( -> $ret:ty )? :
        if const
            $(#[$compiletime_attr:meta])* $compiletime:block
        else
            $(#[$runtime_attr:meta])* $runtime:block
    ) => {
        $crate::text::const_eval_select!(
            @capture$([$($binders)*])? { $($arg : $ty = $arg),* } $(-> $ret)? :
            if const
                $(#[$compiletime_attr])* $compiletime
            else
                $(#[$runtime_attr])* $runtime
        )
    },
}
const NONASCII_MASK: usize = usize::from_ne_bytes([0x80; size_of::<usize>()]);

/// Returns `true` if any byte in the word `x` is nonascii (>= 128).
#[inline]
const fn contains_nonascii(x: usize) -> bool {
    (x & NONASCII_MASK) != 0
}
use core::str::utf8_char_width;

#[allow(clippy::missing_errors_doc)]
/// Walks through `v` checking that it's a valid UTF-8 sequence,
/// returning `Ok(())` in that case, or, if it is invalid, `Err(err)`.
#[inline(always)]
#[rustc_allow_const_fn_unstable(const_eval_select)] // fallback impl has same behavior
pub const fn is_utf8_or_range(v: &[u8]) -> Result<(), Utf8Error> {
    const USIZE_BYTES: usize = size_of::<usize>();
    let mut index = 0;
    let len = v.len();

    let ascii_block_size = 2 * USIZE_BYTES;
    let blocks_end = if len >= ascii_block_size {
        len - ascii_block_size + 1
    } else {
        0
    };
    // Below, we safely fall back to a slower codepath if the offset is `usize::MAX`,
    // so the end-to-end behavior is the same at compiletime and runtime.
    let align = const_eval_select!(
        @capture { v: &[u8] } -> usize:
        if const {
            usize::MAX
        } else {
            v.as_ptr().align_offset(USIZE_BYTES)
        }
    );

    while index < len {
        let old_offset = index;
        macro_rules! err {
            ($error_len: expr) => {
                return Err(Utf8Error {
                    valid_up_to: old_offset,
                    error_len: $error_len,
                })
            };
        }

        macro_rules! next {
            () => {{
                index += 1;
                // we needed data, but there was none: error!
                if index >= len {
                    err!(None)
                }
                v[index]
            }};
        }

        let first = v[index];
        if first >= 128 {
            let w = utf8_char_width(first);
            // 2-byte encoding is for codepoints  \u{0080} to  \u{07ff}
            //        first  C2 80        last DF BF
            // 3-byte encoding is for codepoints  \u{0800} to  \u{ffff}
            //        first  E0 A0 80     last EF BF BF
            //   excluding surrogates codepoints  \u{d800} to  \u{dfff}
            //               ED A0 80 to       ED BF BF
            // 4-byte encoding is for codepoints \u{10000} to \u{10ffff}
            //        first  F0 90 80 80  last F4 8F BF BF
            //
            // Use the UTF-8 syntax from the RFC
            //
            // https://tools.ietf.org/html/rfc3629
            // UTF8-1      = %x00-7F
            // UTF8-2      = %xC2-DF UTF8-tail
            // UTF8-3      = %xE0 %xA0-BF UTF8-tail / %xE1-EC 2( UTF8-tail ) /
            //               %xED %x80-9F UTF8-tail / %xEE-EF 2( UTF8-tail )
            // UTF8-4      = %xF0 %x90-BF 2( UTF8-tail ) / %xF1-F3 3( UTF8-tail ) /
            //               %xF4 %x80-8F 2( UTF8-tail )
            match w {
                2 => {
                    if next!() as i8 >= -64 {
                        err!(Some(1))
                    }
                }
                3 => {
                    match (first, next!()) {
                        (0xE0, 0xA0..=0xBF)
                        | (0xE1..=0xEC, 0x80..=0xBF)
                        | (0xED, 0x80..=0x9F)
                        | (0xEE..=0xEF, 0x80..=0xBF) => {}
                        _ => err!(Some(1)),
                    }
                    if next!() as i8 >= -64 {
                        err!(Some(2))
                    }
                }
                4 => {
                    match (first, next!()) {
                        (0xF0, 0x90..=0xBF)
                        | (0xF1..=0xF3, 0x80..=0xBF)
                        | (0xF4, 0x80..=0x8F) => {}
                        _ => err!(Some(1)),
                    }
                    if next!() as i8 >= -64 {
                        err!(Some(2))
                    }
                    if next!() as i8 >= -64 {
                        err!(Some(3))
                    }
                }
                _ => err!(Some(1)),
            }
            index += 1;
        } else {
            // Ascii case, try to skip forward quickly.
            // When the pointer is aligned, read 2 words of data per iteration
            // until we find a word containing a non-ascii byte.
            if align != usize::MAX
                && align.wrapping_sub(index).is_multiple_of(USIZE_BYTES)
            {
                let ptr = v.as_ptr();
                while index < blocks_end {
                    // SAFETY: since `align - index` and `ascii_block_size` are
                    // multiples of `USIZE_BYTES`, `block = ptr.add(index)` is
                    // always aligned with a `usize` so it's safe to dereference
                    // both `block` and `block.add(1)`.
                    unsafe {
                        let block = ptr.add(index).cast::<usize>();
                        // break if there is a nonascii byte
                        let zu = contains_nonascii(*block);
                        let zv = contains_nonascii(*block.add(1));
                        if zu || zv {
                            break;
                        }
                    }
                    index += ascii_block_size;
                }
                // step from the point where the wordwise loop stopped
                while index < len && v[index] < 128 {
                    index += 1;
                }
            } else {
                index += 1;
            }
        }
    }

    Ok(())
}

#[allow(clippy::missing_errors_doc)]
/// Walks through `v` checking that it's a valid UTF-8 sequence,
/// returning `true` in that case, or, if it is invalid, `false`.
#[inline(always)]
#[rustc_allow_const_fn_unstable(const_eval_select)] // fallback impl has same behavior
pub const fn is_utf8(v: &[u8]) -> bool {
    const USIZE_BYTES: usize = size_of::<usize>();
    let mut index = 0;
    let len = v.len();

    let ascii_block_size = 2 * USIZE_BYTES;
    let blocks_end = if len >= ascii_block_size {
        len - ascii_block_size + 1
    } else {
        0
    };
    // Below, we safely fall back to a slower codepath if the offset is `usize::MAX`,
    // so the end-to-end behavior is the same at compiletime and runtime.
    let align = const_eval_select!(
        @capture { v: &[u8] } -> usize:
        if const {
            usize::MAX
        } else {
            v.as_ptr().align_offset(USIZE_BYTES)
        }
    );

    while index < len {
        macro_rules! next {
            () => {{
                index += 1;
                // we needed data, but there was none: error!
                if index >= len {
                    return false;
                }
                v[index]
            }};
        }

        let first = v[index];
        if first >= 128 {
            let w = utf8_char_width(first);
            // 2-byte encoding is for codepoints  \u{0080} to  \u{07ff}
            //        first  C2 80        last DF BF
            // 3-byte encoding is for codepoints  \u{0800} to  \u{ffff}
            //        first  E0 A0 80     last EF BF BF
            //   excluding surrogates codepoints  \u{d800} to  \u{dfff}
            //               ED A0 80 to       ED BF BF
            // 4-byte encoding is for codepoints \u{10000} to \u{10ffff}
            //        first  F0 90 80 80  last F4 8F BF BF
            //
            // Use the UTF-8 syntax from the RFC
            //
            // https://tools.ietf.org/html/rfc3629
            // UTF8-1      = %x00-7F
            // UTF8-2      = %xC2-DF UTF8-tail
            // UTF8-3      = %xE0 %xA0-BF UTF8-tail / %xE1-EC 2( UTF8-tail ) /
            //               %xED %x80-9F UTF8-tail / %xEE-EF 2( UTF8-tail )
            // UTF8-4      = %xF0 %x90-BF 2( UTF8-tail ) / %xF1-F3 3( UTF8-tail ) /
            //               %xF4 %x80-8F 2( UTF8-tail )
            match w {
                2 => {
                    if next!() as i8 >= -64 {
                        return false;
                    }
                }
                3 => {
                    match (first, next!()) {
                        (0xE0, 0xA0..=0xBF)
                        | (0xE1..=0xEC, 0x80..=0xBF)
                        | (0xED, 0x80..=0x9F)
                        | (0xEE..=0xEF, 0x80..=0xBF) => {}
                        _ => return false,
                    }
                    if next!() as i8 >= -64 {
                        return false;
                    }
                }
                4 => {
                    match (first, next!()) {
                        (0xF0, 0x90..=0xBF)
                        | (0xF1..=0xF3, 0x80..=0xBF)
                        | (0xF4, 0x80..=0x8F) => {}
                        _ => return false,
                    }
                    if next!() as i8 >= -64 {
                        return false;
                    }
                    if next!() as i8 >= -64 {
                        return false;
                    }
                }
                _ => return false,
            }
            index += 1;
        } else {
            // Ascii case, try to skip forward quickly.
            // When the pointer is aligned, read 2 words of data per iteration
            // until we find a word containing a non-ascii byte.
            if align != usize::MAX
                && align.wrapping_sub(index).is_multiple_of(USIZE_BYTES)
            {
                let ptr = v.as_ptr();
                while index < blocks_end {
                    // SAFETY: since `align - index` and `ascii_block_size` are
                    // multiples of `USIZE_BYTES`, `block = ptr.add(index)` is
                    // always aligned with a `usize` so it's safe to dereference
                    // both `block` and `block.add(1)`.
                    unsafe {
                        let block = ptr.add(index).cast::<usize>();
                        // break if there is a nonascii byte
                        let zu = contains_nonascii(*block);
                        let zv = contains_nonascii(*block.add(1));
                        if zu || zv {
                            break;
                        }
                    }
                    index += ascii_block_size;
                }
                // step from the point where the wordwise loop stopped
                while index < len && v[index] < 128 {
                    index += 1;
                }
            } else {
                index += 1;
            }
        }
    }

    true
}
#[must_use]
/// Given a list of keys and values, replace all keys with their respective value
pub fn multi_replace(input: &str, replaces: &[(String, String)]) -> String {
    let mut found = Vec::new();

    for (idx, (key, _value)) in replaces.iter().enumerate() {
        for index in non_overlapping_matches(input, key) {
            found.push((index, idx));
        }
    }
    found.sort_by_key(|x| x.0);
    found.dedup_by(|a, b| {
        let a_start = a.0;
        let a_end = a.0 + replaces[a.1].0.len();
        let b_start = b.0;
        let b_end = b.0 + replaces[b.1].0.len();

        // If they overlap, keep the first one (earlier in iteration order)
        a_end > b_start && a_start < b_end
    });

    let mut offset = 0;
    let mut output = String::new();
    // let names: Vec<(usize, &String)> =
    //     found.iter().map(|x| (x.0, &replaces[x.1].1)).collect();
    //  println!("#> {:#?}", names);

    for f in found {
        // Safety: It passed a bunch of tests so it probably safe
        let before = unsafe { input.get_unchecked(offset..f.0) };
        output.push_str(before);
        offset += before.len();
        //  println!("# '{}'", before);
        // println!(">'{input}':{offset} ({}) for '{output}'", f.0);
        let entry = &replaces[f.1];
        let key = &entry.0;
        let val = &entry.1;
        output.push_str(val);
        offset += key.len();
        // println!(">'{input}':{offset} ({}) for '{output}'", f.0);
    }
    output.push_str(&input[offset..]);

    // println!(">>'{input}':{offset}/{} for '{output}'", input.len());
    output
}
#[must_use]
/// Given a list of keys and values, replace all keys with their respective value
///
/// # Safety
/// Keys must not be overlapping
pub unsafe fn multi_replace_non_overlapping(
    input: &str,
    replaces: &[(String, String)],
) -> String {
    let mut found = Vec::new();

    for (idx, (key, _value)) in replaces.iter().enumerate() {
        found.extend(input.match_indices(key).map(|(index, _)| (index, idx)));
    }
    found.sort_by_key(|x| x.0);
    // let names: Vec<(usize, &String)> =
    //     found.iter().map(|x| (x.0, &replaces[x.1].1)).collect();
    //  println!("#> {:#?}", names);

    let mut offset = 0;
    let mut output = String::new();
    // let names: Vec<(usize, &String)> =
    //     found.iter().map(|x| (x.0, &replaces[x.1].1)).collect();
    // println!("##> {:#?}", names);

    for f in found {
        let before = unsafe { input.get_unchecked(offset..f.0) };
        output.push_str(before);
        offset += before.len();
        // println!("# '{}'", before);
        // println!(">'{input}':{offset} ({}) for '{output}'", f.0);
        let entry = &replaces[f.1];
        let key = &entry.0;
        let val = &entry.1;
        output.push_str(val);
        offset += key.len();
        // println!(">'{input}':{offset} ({}) for '{output}'", f.0);
    }
    output.push_str(&input[offset..]);

    // println!(">>'{input}':{offset}/{} for '{output}'", input.len());
    output
}
#[must_use]
/// Get all non overlapping instances of a substring
pub fn non_overlapping_matches(haystack: &str, needle: &str) -> Vec<usize> {
    if needle.is_empty() {
        return Vec::new();
    }
    let mut matches = Vec::new();
    let mut pos = 0;
    while let Some(index) = haystack[pos..].find(needle) {
        let actual_index = pos + index;
        matches.push(actual_index);
        pos = actual_index + needle.len();
    }
    matches
}
// #[must_use]
// pub fn find_all(string: &str, to_find: &str) -> Vec<usize> {
//     string.match_indices(to_find).map(|(index, _)| index).collect()
// }
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_replacement_creates_new_match() {
//         // After replacing "a" with "b", does it create a new "b" that matches another rule?
//         let input = "a";
//         let replaces = vec![
//             ("a".to_string(), "b".to_string()),
//             ("b".to_string(), "c".to_string()),
//         ];
//         // Tests if replacements cascade or are applied once
//         let result = multi_replace(input, &replaces);
//         assert!(result == "b" || result == "c"); // Depends on your implementation
//     }

//     #[test]
//     fn test_nested_pattern_containment() {
//         // Pattern fully contains another pattern
//         let input = "aaa";
//         let replaces = vec![
//             ("aaa".to_string(), "X".to_string()),
//             ("aa".to_string(), "Y".to_string()),
//         ];
//         assert_eq!(multi_replace(input, &replaces), "X");
//     }

//     #[test]
//     fn test_single_character_replacement() {
//         let input = "a b c d";
//         let replaces = vec![("a".to_string(), "A".to_string())];
//         assert_eq!(multi_replace(input, &replaces), "A b c d");
//     }

//     #[test]
//     fn test_replacement_to_substring_of_input() {
//         let input = "abcdef";
//         let replaces = vec![("cde".to_string(), "cd".to_string())];
//         assert_eq!(multi_replace(input, &replaces), "abcdf");
//     }

//     #[test]
//     fn test_case_sensitivity() {
//         let input = "Hello hello HELLO";
//         let replaces = vec![("hello".to_string(), "HI".to_string())];
//         assert_eq!(multi_replace(input, &replaces), "Hello HI HELLO");
//     }

//     #[test]
//     fn test_whitespace_only_patterns() {
//         let input = "a  b   c";
//         let replaces = vec![("  ".to_string(), "_".to_string())];
//         assert_eq!(multi_replace(input, &replaces), "a_b_ c");
//     }

//     #[test]
//     fn test_pattern_at_boundaries() {
//         let input = "foobarfoo";
//         let replaces = vec![("foo".to_string(), "X".to_string())];
//         assert_eq!(multi_replace(input, &replaces), "XbarX");
//     }

//     #[test]
//     fn test_special_regex_characters() {
//         // If not using regex, these are just literal chars
//         let input = "a.b*c+d";
//         let replaces = vec![(".".to_string(), "-".to_string())];
//         assert_eq!(multi_replace(input, &replaces), "a-b*c+d");
//     }

//     #[test]
//     fn test_very_long_replacement() {
//         let input = "x";
//         let replaces = vec![("x".to_string(), "a".repeat(1000))];
//         let result = multi_replace(input, &replaces);
//         assert_eq!(result.len(), 1000);
//     }

//     #[test]
//     fn test_many_small_replacements() {
//         let input = "abcdefghij";
//         let replaces = vec![
//             ("a".to_string(), "A".to_string()),
//             ("b".to_string(), "B".to_string()),
//             ("c".to_string(), "C".to_string()),
//             ("d".to_string(), "D".to_string()),
//             ("e".to_string(), "E".to_string()),
//         ];
//         assert_eq!(multi_replace(input, &replaces), "ABCDEfghij");
//     }

//     #[test]
//     fn test_duplicate_patterns_in_list() {
//         // Same pattern appears twice in replaces
//         let input = "foo";
//         let replaces = vec![
//             ("foo".to_string(), "bar".to_string()),
//             ("foo".to_string(), "baz".to_string()),
//         ];
//         let result = multi_replace(input, &replaces);
//         // Tests which one wins or if both apply
//         assert!(result == "bar" || result == "baz");
//     }

//     #[test]
//     fn test_replacement_same_as_original() {
//         let input = "hello";
//         let replaces = vec![("hello".to_string(), "hello".to_string())];
//         assert_eq!(multi_replace(input, &replaces), "hello");
//     }

//     #[test]
//     fn test_partial_overlap_no_greedy() {
//         let input = "abab";
//         let replaces = vec![
//             ("ab".to_string(), "X".to_string()),
//             ("ba".to_string(), "Y".to_string()),
//         ];
//         // Tests how overlaps are handled (first-found vs greedy)
//         assert_eq!(multi_replace(input, &replaces), "XX");
//     }

//     #[test]
//     fn test_numeric_patterns() {
//         let input = "123 456 789";
//         let replaces = vec![
//             ("123".to_string(), "ONE".to_string()),
//             ("456".to_string(), "TWO".to_string()),
//         ];
//         assert_eq!(multi_replace(input, &replaces), "ONE TWO 789");
//     }

//     #[test]
//     fn test_newline_and_special_whitespace() {
//         let input = "hello\nworld\ttest";
//         let replaces = vec![("\n".to_string(), " ".to_string())];
//         assert_eq!(multi_replace(input, &replaces), "hello world\ttest");
//     }

//     #[test]
//     fn test_empty_pattern_key() {
//         // Edge case: what happens with empty string as pattern?
//         let input = "hello";
//         let replaces = vec![(String::new(), "X".to_string())];
//         let result = multi_replace(input, &replaces);
//         // Might be undefined behavior - document expected behavior
//         assert!(!result.is_empty());
//     }
// }
