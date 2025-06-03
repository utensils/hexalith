use crate::Result;
use arboard::{Clipboard, ImageData};
use std::borrow::Cow;

/// Copy PNG image data to clipboard
pub fn copy_png_to_clipboard(png_data: &[u8]) -> Result<()> {
    // Decode PNG to get dimensions and raw pixel data
    let decoder = png::Decoder::new(png_data);
    let mut reader = decoder.read_info()?;

    // Allocate buffer for image data
    let mut buf = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut buf)?;

    let info = reader.info();
    let width = info.width as usize;
    let height = info.height as usize;

    // Convert to RGBA if needed
    let rgba_data = match info.color_type {
        png::ColorType::Rgba => buf,
        png::ColorType::Rgb => {
            // Convert RGB to RGBA
            let mut rgba = Vec::with_capacity(width * height * 4);
            for chunk in buf.chunks(3) {
                rgba.extend_from_slice(chunk);
                rgba.push(255); // Full opacity
            }
            rgba
        }
        _ => {
            return Err("Unsupported PNG color type for clipboard".into());
        }
    };

    // Create clipboard instance
    let mut clipboard = Clipboard::new()?;

    // Create image data for clipboard
    let img_data = ImageData {
        width,
        height,
        bytes: Cow::from(rgba_data),
    };

    // Set image to clipboard
    clipboard.set_image(img_data)?;

    Ok(())
}

/// Copy SVG text to clipboard
pub fn copy_svg_to_clipboard(svg_data: &str) -> Result<()> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(svg_data)?;
    Ok(())
}
