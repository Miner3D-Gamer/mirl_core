#![allow(clippy::inline_always)]
#![allow(clippy::cast_lossless)]

// #[allow(unused_imports)]
// use crate::prelude::*;

#[cfg(feature = "random")]
#[cfg(not(target_arch = "wasm32"))]
mod random;

#[cfg(feature = "random")]
#[cfg(not(target_arch = "wasm32"))]
pub use random::*;

/// Presets for common colors
pub mod colors;
// mod interpolation;
// pub use interpolation::*;
/// Convert an r b g format into u32 argb format
#[inline(always)]
#[must_use]
pub const fn rgb_u8_to_u32(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

/// Convert an r b g format into u32 argb format
#[inline(always)]
#[must_use]
pub const fn rgb_to_u32(r: u32, g: u32, b: u32) -> u32 {
    r << 16 | g << 8 | b
}
#[inline(always)]
#[must_use]
/// Convert r g b a in argb format
pub const fn rgba_u8_to_u32(r: u8, g: u8, b: u8, a: u8) -> u32 {
    (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32
}
#[inline(always)]
#[must_use]
/// Convert r g b a into u32 argb
pub const fn rgba_to_u32(r: u32, g: u32, b: u32, a: u32) -> u32 {
    (a) << 24 | (r) << 16 | (g) << 8 | b
}
#[inline(always)]
#[must_use]
/// Convert u32 argb to r g b
pub const fn u32_to_rgb_u8(color: u32) -> (u8, u8, u8) {
    let red = ((color >> 16) & 0xFF) as u8;
    let green = ((color >> 8) & 0xFF) as u8;
    let blue = (color & 0xFF) as u8;
    (red, green, blue)
}
#[inline(always)]
#[must_use]
/// Convert u32 argb to r g b
pub const fn u32_to_rgb(color: u32) -> (u32, u32, u32) {
    let red = (color >> 16) & 0xFF;
    let green = (color >> 8) & 0xFF;
    let blue = color & 0xFF;
    (red, green, blue)
}
#[inline(always)]
#[must_use]
/// Convert u32 argb to r g b a or u32 rgba to a g b r
pub const fn u32_to_rgba_u8(color: u32) -> (u8, u8, u8, u8) {
    let alpha = ((color >> 24) & 0xFF) as u8;
    let red = ((color >> 16) & 0xFF) as u8;
    let green = ((color >> 8) & 0xFF) as u8;
    let blue = (color & 0xFF) as u8;
    (red, green, blue, alpha)
}
#[inline(always)]
#[must_use]
/// Convert u32 argb to r g b a or u32 rgba to b g r a
pub const fn u32_to_bgra_u8(color: u32) -> (u8, u8, u8, u8) {
    let alpha = ((color >> 24) & 0xFF) as u8;
    let red = ((color >> 16) & 0xFF) as u8;
    let green = ((color >> 8) & 0xFF) as u8;
    let blue = (color & 0xFF) as u8;
    (blue, green, red, alpha)
}
#[inline(always)]
#[must_use]
/// Convert u32 argb to r g b a or u32 rgba to a g b r
pub const fn u32_to_rgba(color: u32) -> (u32, u32, u32, u32) {
    let alpha = (color >> 24) & 0xFF;
    let red = (color >> 16) & 0xFF;
    let green = (color >> 8) & 0xFF;
    let blue = color & 0xFF;
    (red, green, blue, alpha)
}
#[inline(always)]
#[must_use]
/// Convert u32 argb to a g b r or u32 rgba to r g b a
pub const fn u32_to_argb_u8(color: u32) -> (u8, u8, u8, u8) {
    let alpha = ((color >> 24) & 0xFF) as u8;
    let red = ((color >> 16) & 0xFF) as u8;
    let green = ((color >> 8) & 0xFF) as u8;
    let blue = (color & 0xFF) as u8;
    (alpha, red, green, blue)
}
#[inline(always)]
#[must_use]
/// Convert u32 argb to a g b r or u32 rgba to r g b a
pub const fn u32_to_argb(color: u32) -> (u32, u32, u32, u32) {
    let alpha = (color >> 24) & 0xFF;
    let red = (color >> 16) & 0xFF;
    let green = (color >> 8) & 0xFF;
    let blue = color & 0xFF;
    (alpha, red, green, blue)
}

#[inline(always)]
#[must_use]
/// Get the alpha of a u32 in argb style, get red rgba style
pub const fn get_alpha_of_u32_in_u8(color: u32) -> u8 {
    ((color >> 24) & 0xFF) as u8
}

#[inline(always)]
#[must_use]
/// Get the red of a u32 in argb style, get alpha rgba style
pub const fn get_red_of_u32_in_u8(color: u32) -> u8 {
    ((color >> 16) & 0xFF) as u8
}
#[inline(always)]
#[must_use]
/// Get the green of a u32
pub const fn get_green_of_u32_in_u8(color: u32) -> u8 {
    ((color >> 8) & 0xFF) as u8
}
#[inline(always)]
#[must_use]
/// Get the blue of a u32
pub const fn get_blue_of_u32_in_u8(color: u32) -> u8 {
    (color & 0xFF) as u8
}
//

#[inline(always)]
#[must_use]
/// Get the alpha of a u32 in argb style, get red rgba style
pub const fn get_alpha_of_u32(color: u32) -> u32 {
    (color >> 24) & 0xFF
}

#[inline(always)]
#[must_use]
/// Get the red of a u32 in argb style, get alpha rgba style
pub const fn get_red_of_u32(color: u32) -> u32 {
    (color >> 16) & 0xFF
}
#[inline(always)]
#[must_use]
/// Get the green of a u32
pub const fn get_green_of_u32(color: u32) -> u32 {
    (color >> 8) & 0xFF
}
#[inline(always)]
#[must_use]
/// Get the blue of a u32
pub const fn get_blue_of_u32(color: u32) -> u32 {
    color & 0xFF
}
/// Image support for mirl
#[cfg(feature = "image")]
#[cfg(feature = "std")]
pub mod imagery;
#[cfg(feature = "std")]
use std::collections::HashSet;

#[cfg(feature = "image")]
#[cfg(feature = "std")]
pub use imagery::*;

#[inline]
#[must_use]
#[allow(clippy::float_cmp)]
/// Get hue of rgb (hsl space)
pub fn get_hue_of_rgb(r: f32, g: f32, b: f32) -> f32 {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);

    if max == min {
        0.0
    } else if max == r {
        ((g - b) / core::f32::math::rem_euclid(max - min, 6.0)) * 60.0
    } else if max == g {
        ((b - r) / (max - min) + 2.0) * 60.0
    } else {
        ((r - g) / (max - min) + 4.0) * 60.0
    }
}

#[inline]
#[must_use]
#[allow(clippy::cast_precision_loss)]
/// Change the brightness of a hsl space
pub fn adjust_brightness_hsl_of_rgb(color: u32, change: f32) -> u32 {
    let alpha = (color >> 24) & 0xFF;
    let red = (color >> 16) & 0xFF;
    let green = (color >> 8) & 0xFF;
    let blue = color & 0xFF;

    let (h, s, l) = rgb_to_hsl(red as u8, green as u8, blue as u8);

    // Adjust lightness in HSL space (most perceptually accurate)
    let l_new = (l + change).clamp(0.0, 100.0);

    let (r_new, g_new, b_new) = hsl_to_rgb_u32(h, s, l_new);

    (alpha << 24) | (r_new << 16) | (g_new << 8) | b_new
}
#[inline]
#[must_use]
#[allow(clippy::cast_possible_truncation)]
/// Convert hsl space to rgb space
pub const fn hsl_to_rgb_f32(
    hue: f32,
    saturation: f32,
    lightness: f32,
) -> (f32, f32, f32) {
    let c = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let m = lightness - c / 2.0;

    let (r1, g1, b1) = match hue as i32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        300..=359 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    (r1 + m, g1 + m, b1 + m)
}

#[inline]
#[must_use]
#[allow(clippy::float_cmp)]
/// Convert rgb space to hsl space
pub const fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r_norm = r as f32 / 255.0;
    let g_norm = g as f32 / 255.0;
    let b_norm = b as f32 / 255.0;

    let max = r_norm.max(g_norm).max(b_norm);
    let min = r_norm.min(g_norm).min(b_norm);
    let delta = max - min;

    let lightness = f32::midpoint(max, min);

    let saturation = if delta < 0.0001 {
        0.0 // achromatic (gray)
    } else if lightness < 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };

    let hue = if delta < 0.0001 {
        0.0 // achromatic (gray)
    } else if max == r_norm {
        60.0 * (((g_norm - b_norm) / delta) % 6.0)
    } else if max == g_norm {
        60.0 * ((b_norm - r_norm) / delta + 2.0)
    } else {
        60.0 * ((r_norm - g_norm) / delta + 4.0)
    };

    (hue, saturation * 100.0, lightness * 100.0)
}

#[inline]
#[must_use]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
/// Convert hsl space to rgb space
pub fn hsl_to_rgb_u32(
    hue: f32,
    saturation: f32,
    lightness: f32,
) -> (u32, u32, u32) {
    let h_norm = hue / 360.0;
    let s_norm = saturation / 100.0;
    let l_norm = lightness / 100.0;

    if s_norm < 0.0001 {
        let gray = core::f32::math::round(l_norm * 255.0) as u32;
        return (gray, gray, gray);
    }

    let hue_to_rgb = |p: f32, q: f32, t: f32| -> f32 {
        let t_adj = if t < 0.0 {
            t + 1.0
        } else if t > 1.0 {
            t - 1.0
        } else {
            t
        };

        if t_adj < 1.0 / 6.0 {
            core::f32::math::mul_add((q - p) * 6.0, t_adj, p)
        } else if t_adj < 1.0 / 2.0 {
            q
        } else if t_adj < 2.0 / 3.0 {
            core::f32::math::mul_add((q - p) * (2.0 / 3.0 - t_adj), 6.0, p)
        } else {
            p
        }
    };

    let q = if l_norm < 0.5 {
        l_norm * (1.0 + s_norm)
    } else {
        core::f32::math::mul_add(l_norm, -s_norm, l_norm + s_norm)
    };
    let p = core::f32::math::mul_add(2.0f32, l_norm, -q);

    let red = hue_to_rgb(p, q, h_norm + 1.0 / 3.0);
    let green = hue_to_rgb(p, q, h_norm);
    let blue = hue_to_rgb(p, q, h_norm - 1.0 / 3.0);

    let r_8bit = core::f32::math::round(red * 255.0) as u32;
    let g_8bit = core::f32::math::round(green * 255.0) as u32;
    let b_8bit = core::f32::math::round(blue * 255.0) as u32;

    (r_8bit, g_8bit, b_8bit)
}

// /// Higher-level function that provides both perceptual models
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[cfg_attr(feature = "c_compatible", repr(C))] pub enum BrightnessModel {
//     /// Uses RGB with perceptual weights
//     LinearWeighted,
//     /// Uses HSL color space
//     HSL,
// }
// #[inline]
// #[must_use]
// #[allow(clippy::cast_sign_loss)]
// #[allow(clippy::cast_precision_loss)]
// #[allow(clippy::cast_possible_truncation)]
// #[allow(clippy::cast_possible_wrap)]
// /// Change the brightness of an rgba color
// pub fn adjust_brightness_based_on_human_eye(
//     color: u32,
//     x: f32,
//     model: BrightnessModel,
// ) -> u32 {
//     match model {
//         BrightnessModel::LinearWeighted => {
//             // Extract color components
//             let (alpha, red, green, blue): (u32, f32, f32, f32) =
//                 u32_to_argb(color).try_tuple_into();
//             // Apply perceptual weights to adjustment
//             let r_adj = x * 0.2126;
//             let g_adj = x * 0.7152;
//             let b_adj = x * 0.0722;

//             // Apply adjustments with clamping
//             let r_new = (red + r_adj).clamp(0.0, 255.0) as u32;
//             let g_new = (green + g_adj).clamp(0.0, 255.0) as u32;
//             let b_new = (blue + b_adj).clamp(0.0, 255.0) as u32;

//             // Recombine with alpha
//             (alpha << 24) | (r_new << 16) | (g_new << 8) | b_new
//         }
//         BrightnessModel::HSL => adjust_brightness_hsl_of_rgb(color, x),
//     }
// }

#[inline]
#[must_use]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
/// Shift the color (hue) of rgb
pub fn shift_color_rgb(
    red: u8,
    green: u8,
    blue: u8,
    hue_shift: f32,
) -> (u32, u32, u32) {
    let (hue, saturation, lightness) = rgb_to_hsl(red, green, blue);

    let new_h = (hue + hue_shift) % 360.0;

    let l_norm = lightness / 100.0;
    let s_norm = saturation / 100.0;

    let new_s = if l_norm > 0.5 {
        // Brighter colors - reduce saturation as lightness increases
        s_norm * core::f32::math::mul_add(l_norm - 0.5, -2.0, 1.0) * 100.0
    } else {
        // Darker colors - reduce saturation as lightness decreases
        s_norm * core::f32::math::mul_add(0.5 - l_norm, -2.0, 1.0) * 100.0
    };

    let new_l = if l_norm > 0.5 {
        // Brighter colors - darken slightly to maintain balance
        l_norm * 0.95 * 100.0
    } else {
        // Darker colors - lighten slightly, but don't go over 100
        (l_norm * 1.1).min(1.0) * 100.0
    };

    let new_s = new_s.clamp(0.0, 100.0);
    let new_l = new_l.clamp(0.0, 100.0);

    hsl_to_rgb_u32(new_h, new_s, new_l)
}

#[must_use]
/// Shift the hue of rgb, isn't there another function that does the exact same?
pub fn shift_hue_rgb(
    r: u8,
    g: u8,
    b: u8,
    hue_shift_degrees: f32,
) -> (u32, u32, u32) {
    // Convert to floating point RGB
    let mut hsv = rgb_to_hsl(r, g, b);

    // Shift hue
    hsv.0 = (hsv.0 + hue_shift_degrees) % 360.0;

    // Convert back to integer RGB
    let (r, g, b) = hsl_to_rgb_u32(hsv.0, hsv.1, hsv.2);
    (r, g, b)
}
#[must_use]
/// Shift the hue of a rgba u32
pub fn shift_hue_u32(color: u32, hue_shift: f32) -> u32 {
    let (r, g, b) = u32_to_rgb_u8(color);
    let (r, g, b) = shift_hue_rgb(r, g, b, hue_shift);
    rgb_to_u32(r, g, b)
}
#[inline]
#[must_use]
/// Shift the color of a rgba u32
pub fn shift_color_u32(color: u32, hue_shift: f32) -> u32 {
    let alpha = (color >> 24) & 0xFF;
    let red = (color >> 16) & 0xFF;
    let green = (color >> 8) & 0xFF;
    let blue = color & 0xFF;

    let (r_new, g_new, b_new) =
        shift_color_rgb(red as u8, green as u8, blue as u8, hue_shift);

    rgba_to_u32(r_new, g_new, b_new, alpha)
}

#[must_use]
#[inline]
#[allow(clippy::cast_sign_loss)]
/// Adjust the brightness of a rgba u32 color faster than with the function that does it with with human perception in mind
pub const fn adjust_brightness_fast(color: u32, x: i32) -> u32 {
    // Extract color components
    let t = u32_to_rgb(color);
    let (red, green, blue): (i32, i32, i32) =
        (t.0 as i32, t.1 as i32, t.2 as i32);

    // Calculate new values with clamping
    let r_new = (red + x).clamp(0, 255) as u32;
    let g_new = (green + x).clamp(0, 255) as u32;
    let b_new = (blue + x).clamp(0, 255) as u32;

    // Recombine into a single color value
    (r_new << 16) | (g_new << 8) | b_new
}
/// Desaturate the current color without caring that much about human vision
#[inline]
#[must_use]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub fn desaturate_fast(color: u32, amount: f32) -> u32 {
    // Extract color components
    let red = ((color >> 16) & 0xFF) as f32;
    let green = ((color >> 8) & 0xFF) as f32;
    let blue = (color & 0xFF) as f32;

    // Compute grayscale (luminance approximation)
    let gray = core::f32::math::mul_add(
        0.114f32,
        blue,
        core::f32::math::mul_add(0.299f32, red, 0.587 * green),
    );

    // Interpolate between color and gray based on amount (0.0 to 1.0)
    let r_new = core::f32::math::mul_add(red, 1.0 - amount, gray * amount)
        .clamp(0.0, 255.0) as u32;
    let g_new = core::f32::math::mul_add(green, 1.0 - amount, gray * amount)
        .clamp(0.0, 255.0) as u32;
    let b_new = core::f32::math::mul_add(blue, 1.0 - amount, gray * amount)
        .clamp(0.0, 255.0) as u32;

    // Recombine
    (r_new << 16) | (g_new << 8) | b_new
}
#[allow(clippy::missing_errors_doc)]
/// Rasterize an svg in the desired dimensions
#[cfg(feature = "svg")]
pub fn rasterize_svg(
    svg_data: &[u8],
    width: u32,
    height: u32,
) -> Result<resvg::tiny_skia::Pixmap, Option<resvg::usvg::Error>> {
    let opt = resvg::usvg::Options::default();
    //let fontdb = usvg::fontdb::Database::new();

    let tree = match resvg::usvg::Tree::from_data(svg_data, &opt) {
        Ok(value) => value,
        Err(error) => return Err(Some(error)),
    };

    // Create a pixmap with desired size (from SVG's size)
    let mut pixmap =
        resvg::tiny_skia::Pixmap::new(width, height).ok_or(None)?; // ok_or(None) is actually pretty funny. clippy! Time for you to add another rule!

    // Render the SVG
    resvg::render(
        &tree,
        resvg::usvg::Transform::default(),
        &mut pixmap.as_mut(),
    );

    Ok(pixmap)
}
#[cfg(feature = "svg")]
#[cfg(feature = "std")]
#[allow(clippy::unwrap_used, clippy::missing_panics_doc)]
/// To use this function, enable the "`svg`" feature
/// Converts a `resvg::tiny_skia::Pixmap` to a `mirl::Buffer`
#[must_use]
pub fn pixmap_to_buffer(pixmap: &resvg::tiny_skia::Pixmap) -> crate::Buffer {
    let mut data = Vec::new();
    for y in 0..pixmap.height() {
        for x in 0..pixmap.width() {
            let color = unsafe { pixmap.pixel(x, y).unwrap_unchecked() };
            data.push(rgba_u8_to_u32(
                color.red(),
                color.green(),
                color.blue(),
                color.alpha(),
            ));
        }
    }
    unsafe {
        crate::Buffer::new(
            (pixmap.width() as usize, pixmap.height() as usize),
            data,
        )
        .unwrap_unchecked()
    }
}

// /// A Buffer to be accessed without compression
// /// What is the difference between Buffer and Buffer? Buffer has more attributes ig :|
// #[derive(Debug, Clone, PartialEq, Eq)]
// #[cfg_attr(feature = "c_compatible", repr(C))]
// pub struct Buffer {
//     /// The Raw Data
//     pub data: Box<[u32]>,
//     /// The width of the image
//     pub width: usize,
//     /// The height of the image
//     pub height: usize,
// }

// impl Buffer {
//     /// Create a new Buffer with some data, if you want to create an empty image use Buffer::new_empty()
//     pub fn new(data: Vec<u32>, width: usize, height: usize) -> Self {
//         Self {
//             data: data.into_boxed_slice(),
//             width,
//             height,
//         }
//     }
//     /// Create a new, empty, Buffer instance. If you already have data to fill it with you can use Buffer::new()
//     pub fn new_empty(width: usize, height: usize) -> Self {
//         Self {
//             data: repeat_data(0, width * height).into(),
//             width: width,
//             height: height,
//         }
//     }
//     /// Gets the pixel color at the requested 3d coordinates
//     pub fn get_pixel(&self, pos: (usize, usize)) -> u32 {
//         return self.data[pos.1 * self.width + pos.0];
//     }
//     /// Generate a error texture with the desired size
//     pub fn generate_fallback(
//         width: usize,
//         height: usize,
//         square_size: usize,
//     ) -> Self {
//         let mut data = Vec::with_capacity(width * height);

//         let purple = rgb_to_u32(128, 0, 128);
//         let black = rgb_to_u32(0, 0, 0);

//         for y in 0..height {
//             for x in 0..width {
//                 let square_x = x / square_size;
//                 let square_y = y / square_size;

//                 let color = if (square_x + square_y) % 2 == 0 {
//                     purple
//                 } else {
//                     black
//                 };

//                 data.push(color);
//             }
//         }

//         Self::new(data, width, height)
//     }
//     /// Optimizes the image by removing empty space around the image
//     pub fn remove_margins(&mut self) {
//         // Remove all margins in one pass to avoid multiple data copies
//         let (top_trim, bottom_trim, left_trim, right_trim) =
//             self.calculate_trims();

//         if top_trim > 0 || bottom_trim > 0 || left_trim > 0 || right_trim > 0 {
//             self.apply_trim(top_trim, bottom_trim, left_trim, right_trim);
//         }
//     }
//     /// Calculates the empty space around the image
//     pub fn calculate_trims(&self) -> (usize, usize, usize, usize) {
//         let mut top_trim = 0;
//         let mut bottom_trim = 0;
//         let mut left_trim = 0;
//         let mut right_trim = 0;

//         // Calculate top trim
//         for row in 0..self.height {
//             if self.is_row_transparent(row) {
//                 top_trim += 1;
//             } else {
//                 break;
//             }
//         }

//         // Calculate bottom trim
//         for row in (0..self.height).rev() {
//             if self.is_row_transparent(row) {
//                 bottom_trim += 1;
//             } else {
//                 break;
//             }
//         }

//         // Calculate left trim
//         for col in 0..self.width {
//             if self.is_col_transparent(col) {
//                 left_trim += 1;
//             } else {
//                 break;
//             }
//         }

//         // Calculate right trim
//         for col in (0..self.width).rev() {
//             if self.is_col_transparent(col) {
//                 right_trim += 1;
//             } else {
//                 break;
//             }
//         }

//         (top_trim, bottom_trim, left_trim, right_trim)
//     }
//     /// Checks if the requested row only has fully transparent pixels
//     pub fn is_row_transparent(&self, row: usize) -> bool {
//         let start = row * self.width;
//         let end = start + self.width;
//         self.data[start..end]
//             .iter()
//             .all(|&pixel| get_u32_alpha_of_u32(pixel) == 0)
//     }
//     /// Checks if the requested column only has fully transparent pixels
//     pub fn is_col_transparent(&self, col: usize) -> bool {
//         (0..self.height).all(|row| {
//             get_u32_alpha_of_u32(self.data[row * self.width + col]) == 0
//         })
//     }
//     /// Trims the image by the given restrictions
//     pub fn apply_trim(
//         &mut self,
//         top: usize,
//         bottom: usize,
//         left: usize,
//         right: usize,
//     ) {
//         let new_width = self.width - left - right;
//         let new_height = self.height - top - bottom;
//         let mut new_data = Vec::with_capacity(new_width * new_height);

//         for row in top..(self.height - bottom) {
//             let row_start = row * self.width + left;
//             let row_end = row_start + new_width;
//             new_data.extend_from_slice(&self.data[row_start..row_end]);
//         }

//         self.data = new_data.into();
//         self.width = new_width;
//         self.height = new_height;
//     }
// }

// impl From<Buffer> for Buffer {
//     fn from(p: Buffer) -> Self {
//         Buffer {
//             data: p.buffer,
//             width: p.width,
//             height: p.height,
//         }
//     }
// }
// impl From<Buffer> for Buffer {
//     fn from(p: Buffer) -> Self {
//         let mut buffer = Buffer::new(p.width, p.height);
//         buffer.buffer = p.data;
//         return buffer;
//     }
// }
// mod pixel;
// pub use pixel::*;

/// Convert u32 argb to hex
#[inline(always)]
#[must_use]
#[cfg(feature = "std")]
pub fn u32_to_hex_without_alpha(color: u32) -> String {
    let (r, g, b) = u32_to_rgb_u8(color);
    format!("{r:02x}{g:02x}{b:02x}")
}
/// Convert u32 argb to hex
#[inline(always)]
#[must_use]
#[cfg(feature = "std")]
pub fn u32_to_hex(color: u32) -> String {
    let (r, g, b, a) = u32_to_rgba_u8(color);
    format!("{r:02x}{g:02x}{b:02x}{a:02x}")
}

/// Convert hex into u32 argb
///
/// # Errors
/// If the hex is not valid - The function expects for there to not be a # before the hex values
#[inline(always)]
pub fn hex_to_u32(hex: &str) -> Result<u32, core::num::ParseIntError> {
    let alpha = u8::from_str_radix(&hex[0..2], 16)?;
    let red = u8::from_str_radix(&hex[2..4], 16)?;
    let green = u8::from_str_radix(&hex[4..6], 16)?;
    let blue = u8::from_str_radix(&hex[6..8], 16)?;
    Ok(rgba_u8_to_u32(red, green, blue, alpha))
}

/// Convert hex into u32 rgba
///
/// # Errors
/// If the hex is not valid - The function expects for there to not be a # before the hex values
#[inline(always)]
#[cfg(feature = "std")]
pub fn hex_to_u32_rgba(hex: &str) -> Result<u32, core::num::ParseIntError> {
    let red = u8::from_str_radix(&hex[0..2], 16)?;
    let green = u8::from_str_radix(&hex[2..4], 16)?;
    let blue = u8::from_str_radix(&hex[4..6], 16)?;
    let alpha = u8::from_str_radix(&hex[6..8], 16)?;
    Ok(argb_to_rgba(rgba_u8_to_u32(red, green, blue, alpha)))
}

/// Convert hex into u32 rgb
///
/// # Errors
/// If the hex is not valid - The function expects for there to not be a # before the hex values
#[inline(always)]
#[cfg(feature = "std")]
pub fn hex_to_u32_rgb(hex: &str) -> Result<u32, core::num::ParseIntError> {
    let red = u8::from_str_radix(&hex[2..4], 16)?;
    let green = u8::from_str_radix(&hex[4..6], 16)?;
    let blue = u8::from_str_radix(&hex[6..8], 16)?;
    Ok(rgb_u8_to_u32(red, green, blue))
}
#[must_use]
/// Converts rgb into hex
#[inline(always)]
#[cfg(feature = "std")]
pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("{r:02x}{g:02x}{b:02x}")
}
/// Converts argb to rgba color
#[must_use]
pub const fn argb_to_rgba(color: u32) -> u32 {
    let (a, r, g, b) = u32_to_argb_u8(color);
    rgba_u8_to_u32(a, g, b, r)
}
/// Converts argb to abgr color
#[must_use]
pub const fn switch_red_and_blue(color: u32) -> u32 {
    let (a, r, g, b) = u32_to_argb_u8(color);
    rgba_u8_to_u32(b, g, r, a)
}
/// Converts argb to abgr color
#[must_use]
pub const fn switch_alpha_and_blue(color: u32) -> u32 {
    let (a, r, g, b) = u32_to_argb_u8(color);
    rgba_u8_to_u32(r, g, a, b)
}
#[must_use]
/// Converts a list of argb to rgba and vice versa
#[inline(always)]
#[cfg(feature = "std")]
pub fn switch_alpha_and_blue_list(input: &[u32]) -> Vec<u32> {
    input.iter().map(|&argb| switch_alpha_and_blue(argb)).collect()
}
#[must_use]
/// Converts a list of argb to rgba and vice versa
#[inline(always)]
#[cfg(feature = "std")]
pub fn switch_red_and_blue_list(input: &[u32]) -> Vec<u32> {
    input.iter().map(|&argb| switch_red_and_blue(argb)).collect()
}
#[must_use]
/// Converts a list of argb to rgba and vice versa
#[inline(always)]
#[cfg(feature = "std")]
pub fn argb_list_to_rgba_list(input: &[u32]) -> Vec<u32> {
    input.iter().map(|&argb| argb_to_rgba(argb)).collect()
}
#[must_use]
#[cfg(feature = "std")]
/// Converts a list of rgba to argb and vice versa
pub fn rgba_list_to_argb_list(input: &[u32]) -> Vec<u32> {
    argb_list_to_rgba_list(input)
}

#[must_use]
#[inline]
/// Get the next color in line
pub const fn advance_color(red: u8, green: u8, blue: u8) -> (u8, u8, u8) {
    if blue == 255 {
        if green == 255 {
            if red == 255 {
                return (0, 0, 0);
            }
            return (red + 1, green, blue);
        }
        return (red, green + 1, blue);
    }
    (red, green, blue + 1)
}
#[must_use]
#[cfg(feature = "std")]
/// This is quite expensive
pub fn get_unused_color(
    buffer: &[u8],
    current_color: (u8, u8, u8),
) -> (u8, u8, u8) {
    let mut current_color = current_color;
    let mut unique_colors = HashSet::new();
    for i in buffer.chunks_exact(4) {
        if i[0] != 0 {
            unique_colors.insert((i[1], i[2], i[3]));
        }
    }
    while unique_colors.contains(&current_color) {
        current_color =
            advance_color(current_color.0, current_color.1, current_color.2);
    }
    current_color
}

// #[must_use]
// #[allow(clippy::cast_sign_loss)]
// #[allow(clippy::cast_precision_loss)]
// #[allow(clippy::cast_possible_truncation)]
// #[allow(clippy::cast_possible_wrap)]
// /// Interpolate a u32 rgba based on 4 other u32 rgba
// pub fn bilinear_interpolate_f32(
//     top_left: u32,
//     top_right: u32,
//     bottom_left: u32,
//     bottom_right: u32,
//     progress_x: f32,
//     progress_y: f32,
// ) -> u32 {
//     let (r1, g1, b1, a1) = u32_to_rgba(top_left).into_value();
//     let (r2, g2, b2, a2) = u32_to_rgba(top_right).into_value();
//     let (r3, g3, b3, a3) = u32_to_rgba(bottom_left).into_value();
//     let (r4, g4, b4, a4) = u32_to_rgba(bottom_right).into_value();

//     let interpolate_channel = |c1: f32, c2: f32, c3: f32, c4: f32| -> u8 {
//         let top = core::f32::math::mul_add(c1, 1.0 - progress_x, c2 * progress_x);
//         let bottom =
//             core::f32::math::mul_add(c3, 1.0 - progress_x, c4 * progress_x);
//         let result = core::f32::math::mul_add(
//             top,
//             1.0 - progress_y,
//             bottom * progress_y,
//         );
//         core::f32::math::round(result).clamp(0.0, 255.0) as u8
//     };

//     let red = interpolate_channel(r1, r2, r3, r4);
//     let green = interpolate_channel(g1, g2, g3, g4);
//     let blue = interpolate_channel(b1, b2, b3, b4);
//     let alpha = interpolate_channel(a1, a2, a3, a4);

//     rgba_u8_to_u32(red, green, blue, alpha)
// }

#[must_use]
/// Interpolate between 2 numbers linearly
/// progress should be from 0 to 255
pub const fn interpolate_color_rgb_u32(
    from: u32,
    to: u32,
    progress: u32,
) -> u32 {
    // Convert progress to fixed-point (8.8 format)
    //let progress_fixed = (progress * 256.0) as u32;
    let inv_progress = 256 - progress;

    let r1 = (from >> 24) & 0xFF;
    let g1 = (from >> 16) & 0xFF;
    let b1 = (from >> 8) & 0xFF;
    let r2 = (to >> 24) & 0xFF;
    let g2 = (to >> 16) & 0xFF;
    let b2 = (to >> 8) & 0xFF;

    let r = (r1 * inv_progress + r2 * progress) >> 8;
    let g = (g1 * inv_progress + g2 * progress) >> 8;
    let b = (b1 * inv_progress + b2 * progress) >> 8;

    (r << 24) | (g << 16) | (b << 8) | 0xFF
}
/// Inverts the rgb channels of the given color
#[must_use]
pub const fn invert_color(color: u32) -> u32 {
    let (r, g, b, a) = u32_to_rgba_u8(color);
    rgba_u8_to_u32(255 - r, 255 - g, 255 - b, a)
}
// /// Generates a random color (+random alpha)
// ///
// /// # Errors
// /// See [`getrandom::Error`]
// #[inline]
// pub fn generate_random_color() -> Result<u32, getrandom::Error> {
//     getrandom::u32()
// }
// /// Generates a random color (with alpha of 0)
// ///
// /// # Errors
// /// See [`getrandom::Error`]
// #[inline]
// pub fn generate_random_color_stable_alpha(
//     alpha: u32,
// ) -> Result<u32, getrandom::Error> {
//     let color = getrandom::u32()?;
//     Ok(set_alpha(color, alpha))
// }

/// A trait for changing or retrieving a single channel of a color
pub const trait ColorManipulation {
    #[must_use]
    /// Set the alpha channel
    fn with_alpha(&self, alpha: Self) -> Self;
    #[must_use]
    /// Set the red channel
    fn with_red(&self, red: Self) -> Self;
    #[must_use]
    /// Set the green channel
    fn with_green(&self, green: Self) -> Self;
    #[must_use]
    /// Set the blue channel
    fn with_blue(&self, blue: Self) -> Self;
    #[must_use]
    /// Get the alpha channel
    fn alpha(&self) -> Self;
    #[must_use]
    /// Get the red channel
    fn red(&self) -> Self;
    #[must_use]
    /// Get the green channel
    fn green(&self) -> Self;
    #[must_use]
    /// Get the blue channel
    fn blue(&self) -> Self;
    #[must_use]
    /// Add X to the blue channel
    fn add_blue(&self, other: Self) -> Self;
    #[must_use]
    /// Add X to the red channel
    fn add_red(&self, other: Self) -> Self;
    #[must_use]
    /// Add X to the green channel
    fn add_green(&self, other: Self) -> Self;
    #[must_use]
    /// Add X to the alpha channel
    fn add_alpha(&self, other: Self) -> Self;
    #[must_use]
    /// Subtract X from the blue channel
    fn sub_blue(&self, other: Self) -> Self;
    #[must_use]
    /// Subtract X from the red channel
    fn sub_red(&self, other: Self) -> Self;
    #[must_use]
    /// Subtract X from the green channel
    fn sub_green(&self, other: Self) -> Self;
    #[must_use]
    /// Subtract X from the alpha channel
    fn sub_alpha(&self, other: Self) -> Self;
    #[must_use]
    /// Add to all color channels - Alpha excluded
    fn add_all_color_channels(&self, other: Self) -> Self;
    #[must_use]
    /// Sub from all color channels - Alpha excluded
    fn sub_all_color_channels(&self, other: Self) -> Self;
    #[must_use]
    /// Invert all color channels
    fn invert_color(&self) -> Self;
    #[must_use]
    /// Invert the red channel
    fn invert_red(&self) -> Self;
    #[must_use]
    /// Invert the blue channel
    fn invert_blue(&self) -> Self;
    #[must_use]
    /// Invert the green channel
    fn invert_green(&self) -> Self;
    #[must_use]
    /// Invert alpha channel
    fn invert_alpha(&self) -> Self;
}

impl const ColorManipulation for u32 {
    #[inline(always)]
    fn with_alpha(&self, alpha: Self) -> Self {
        rgba_to_u32(
            get_red_of_u32(*self),
            get_green_of_u32(*self),
            get_blue_of_u32(*self),
            alpha,
        )
    }

    #[inline(always)]
    fn with_red(&self, red: Self) -> Self {
        rgba_to_u32(
            red,
            get_green_of_u32(*self),
            get_blue_of_u32(*self),
            get_alpha_of_u32(*self),
        )
    }

    #[inline(always)]
    fn with_green(&self, green: Self) -> Self {
        rgba_to_u32(
            get_red_of_u32(*self),
            green,
            get_blue_of_u32(*self),
            get_alpha_of_u32(*self),
        )
    }

    #[inline(always)]
    fn with_blue(&self, blue: Self) -> Self {
        rgba_to_u32(
            get_red_of_u32(*self),
            get_green_of_u32(*self),
            blue,
            get_alpha_of_u32(*self),
        )
    }

    #[inline(always)]
    fn alpha(&self) -> Self {
        get_alpha_of_u32(*self)
    }

    #[inline(always)]
    fn red(&self) -> Self {
        get_red_of_u32(*self)
    }

    #[inline(always)]
    fn green(&self) -> Self {
        get_green_of_u32(*self)
    }

    #[inline(always)]
    fn blue(&self) -> Self {
        get_blue_of_u32(*self)
    }
    #[inline(always)]
    fn add_alpha(&self, other: Self) -> Self {
        self.with_alpha(self.alpha() + other)
    }
    #[inline(always)]
    fn add_red(&self, other: Self) -> Self {
        self.with_red(self.red() + other)
    }
    #[inline(always)]
    fn add_green(&self, other: Self) -> Self {
        self.with_green(self.green() + other)
    }
    #[inline(always)]
    fn add_blue(&self, other: Self) -> Self {
        self.with_blue(self.blue() + other)
    }
    #[inline(always)]
    fn sub_alpha(&self, other: Self) -> Self {
        self.with_alpha(self.alpha() - other)
    }
    #[inline(always)]
    fn sub_red(&self, other: Self) -> Self {
        self.with_red(self.red() - other)
    }
    #[inline(always)]
    fn sub_green(&self, other: Self) -> Self {
        self.with_green(self.green() - other)
    }
    #[inline(always)]
    fn sub_blue(&self, other: Self) -> Self {
        self.with_blue(self.blue() - other)
    }

    fn add_all_color_channels(&self, other: Self) -> Self {
        self.add_blue(other).add_red(other).add_green(other)
    }
    fn sub_all_color_channels(&self, other: Self) -> Self {
        self.sub_blue(other).sub_red(other).sub_green(other)
    }
    fn invert_alpha(&self) -> Self {
        self.with_alpha(invert_channel(self.alpha()))
    }
    fn invert_red(&self) -> Self {
        self.with_red(invert_channel(self.red()))
    }
    fn invert_blue(&self) -> Self {
        self.with_blue(invert_channel(self.blue()))
    }
    fn invert_green(&self) -> Self {
        self.with_green(invert_channel(self.green()))
    }
    fn invert_color(&self) -> Self {
        rgba_to_u32(
            invert_channel(self.red()),
            invert_channel(self.green()),
            invert_channel(self.blue()),
            self.alpha(),
        )
    }
}
#[must_use]
/// Invert a color channel
pub const fn invert_channel(color: u32) -> u32 {
    255 - color
}

/// Reorder color from ((r1, r2), (g1, g2), (b1, b2)) to ((r1, g1, b1), (r2, g2, b2))
#[must_use]
pub const fn reorder_color_range(
    color_range: ((u32, u32), (u32, u32), (u32, u32)),
) -> ((u32, u32, u32), (u32, u32, u32)) {
    (
        (color_range.0.0, color_range.1.0, color_range.2.0),
        (color_range.0.1, color_range.1.1, color_range.2.1),
    )
}
