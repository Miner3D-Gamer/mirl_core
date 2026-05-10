use image::{GenericImage, GenericImageView};

use crate::{
    graphics::{rgba_u8_to_u32, u32_to_rgba_u8},
    prelude::Buffer,
};
/// Convert a [u32] argb format into an [`image::Rgba<u8>`]
#[must_use]
pub const fn u32_to_image_rgba(color: u32) -> image::Rgba<u8> {
    let (r, g, b, a) = u32_to_rgba_u8(color);
    image::Rgba([r, g, b, 255 - a])
}
/// Convert a [`image::Rgba<u8>`] into an [u32] argb
#[must_use]
pub const fn image_rgba_to_u32(rgba: image::Rgba<u8>) -> u32 {
    let [r, g, b, a] = rgba.0;
    rgba_u8_to_u32(r, g, b, a)
}
#[must_use]
/// Convert basic RGB into [`image::Rgba<u8>`]
pub const fn rgb_to_image_rgba(r: u8, g: u8, b: u8) -> image::Rgba<u8> {
    image::Rgba([r, g, b, 255])
}
#[must_use]
/// Create a new, empty, [`image::DynamicImage`]
pub fn create_empty_image(width: u32, height: u32) -> image::DynamicImage {
    image::DynamicImage::new_rgba8(width, height)
}

// pub fn vec_to_img(
//     image: Vec<u32>,
//     width: u32,
//     height: u32,
// ) -> image::DynamicImage {
//     let mut img = create_empty_image(width, height);
//     for y in 0..height {
//         for x in 0..width {
//             let color = image[(y * width + x) as usize];
//             img.put_pixel(x as u32, y as u32, u32_to_image_rgba(color));
//         }
//     }
//     img
// }
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
// #[deprecated = "This function is terribly slow. Please turn the dynamic image into a Buffer before drawing"]
/// Draw one dynamic image into another
pub fn draw_texture_into_image(
    image: &mut image::DynamicImage,
    texture_width: u16,
    texture_height: u16,
    texture_x: u16,
    texture_y: u16,
    texture: &image::DynamicImage,
) {
    if core::intrinsics::unlikely(texture.width() == 0 || texture.height() == 0)
    {
        return;
    }

    let scaling_x = (texture_width as f32) / (texture.width() as f32);
    let scaling_y = (texture_height as f32) / (texture.height() as f32);

    for x in texture_x..texture_x + texture_width {
        let texture_uv_x = ((x - texture_x) as f32 / scaling_x) as u32;

        for y in texture_y..texture_y + texture_height {
            let texture_uv_y = ((y - texture_y) as f32 / scaling_y) as u32;

            if texture_uv_x < texture.width() && texture_uv_y < texture.height()
            {
                let pixel = texture.get_pixel(texture_uv_x, texture_uv_y);
                image.put_pixel(x as u32, y as u32, pixel);
            }
        }
    }
}
#[must_use]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
/// Set the image size and fill the new space with nothing
pub fn set_image_size(
    image: &image::DynamicImage,
    width: u32,
    height: u32,
) -> image::DynamicImage {
    //not resize, added pixels by new size should be empty
    let mut img = create_empty_image(width, height);
    draw_texture_into_image(
        &mut img,
        image.width() as u16,
        image.height() as u16,
        0,
        0,
        image,
    );
    img
}
// TODO: Add this to mirl_extensions
// #[must_use]
// #[allow(clippy::unwrap_used, clippy::missing_panics_doc)]
// #[cfg(feature = "svg")]
// /// Converts between [`resvg::tiny_skia::Pixmap`] and [`image::DynamicImage`]
// pub fn pixmap_to_dynamic_image(
//     ras: &resvg::tiny_skia::Pixmap,
// ) -> Option<image::DynamicImage> {
//     let mut img = image::DynamicImage::new(
//         ras.width(),
//         ras.height(),
//         image::ColorType::Rgba8,
//     );
//     for x in 0..ras.width() {
//         for y in 0..ras.height() {
//             use crate::extensions::*;

//             let color = unsafe { ras.pixel(x, y).unwrap_unchecked() }; // If this crashes I will be glad to fix it, until then shut it
//             let (r, g, b) = crate::graphics::shift_hue_rgb(
//                 color.red(),
//                 color.green(),
//                 color.blue(),
//                 50.0,
//             )
//             .try_tuple_into()?;
//             let pixel = image::Rgba([r, g, b, color.alpha()]);
//             img.put_pixel(x, y, pixel);
//         }
//     }
//     Some(img)
// }
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[must_use]
/// Converts between [`image::DynamicImage`] and [Buffer]
pub fn dynamic_image_to_buffer(image: &image::DynamicImage) -> Buffer {
    let width = image.width() as usize;
    let height = image.height() as usize;

    let mut img = Buffer::new_empty((width, height));
    for y in 0..height {
        for x in 0..width {
            let color = image.get_pixel(x as u32, y as u32);
            img.data[y * img.width + x] = image_rgba_to_u32(color);
        }
    }
    img
}
#[must_use]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
/// Converts between [Buffer] and [`image::DynamicImage`]
pub fn buffer_to_dynamic_image(buffer: &Buffer) -> image::DynamicImage {
    let mut img = create_empty_image(buffer.width as u32, buffer.height as u32);
    for y in 0..buffer.height {
        for x in 0..buffer.width {
            let color = buffer.data[y * buffer.width + x];
            img.put_pixel(x as u32, y as u32, u32_to_image_rgba(color));
        }
    }
    img
}

// Make .into() work
impl From<Buffer> for image::DynamicImage {
    fn from(bush: Buffer) -> Self {
        buffer_to_dynamic_image(&bush)
    }
}
impl From<image::DynamicImage> for Buffer {
    fn from(bush: image::DynamicImage) -> Self {
        dynamic_image_to_buffer(&bush)
    }
}

// Automatically convert Buffer to DynamicImage
// impl Deref for Buffer {
//     type Target = image::DynamicImage;

//     fn deref(&self) -> &Self::Target {
//         buffer_to_dynamic_image(&self)
//     }
// }
// use crate::graphics::pixel::Pixel;

// impl From<Pixel> for image::Rgba<u8> {
//     fn from(p: Pixel) -> Self {
//         rgb_to_image_rgba(p.r, p.g, p.b)
//     }
// }
