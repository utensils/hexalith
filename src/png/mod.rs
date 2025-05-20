use crate::generator::Generator;
use crate::svg;
use crate::Result;
use resvg::tiny_skia;
use resvg::usvg::{self, TreeParsing};
use std::fs;
use std::path::Path;

/// Converts an SVG string to PNG data
pub fn convert_svg_to_png(svg_data: &str, width: u32, height: u32) -> Result<Vec<u8>> {
    // Parse the SVG string
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_str(svg_data, &opt)?;

    // Create a Skia surface to render on
    let pixmap_size = tiny_skia::IntSize::from_wh(width, height).ok_or("Invalid dimensions")?;
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .ok_or("Failed to create Pixmap")?;

    // Set up the renderer and render the SVG
    let render_tree = resvg::Tree::from_usvg(&tree);
    render_tree.render(tiny_skia::Transform::default(), &mut pixmap.as_mut());

    Ok(pixmap.encode_png()?)
}

/// Generates a PNG from a logo generator
pub fn generate_png(generator: &Generator, width: u32, height: u32) -> Result<Vec<u8>> {
    // First generate the SVG
    let svg_data = svg::generate_svg(generator, width, height)?;

    // Then convert it to PNG
    convert_svg_to_png(&svg_data, width, height)
}

/// Saves PNG data to a file
pub fn save_png<P: AsRef<Path>>(png_data: &[u8], path: P) -> Result<()> {
    fs::write(path, png_data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::Generator;

    #[test]
    fn test_png_generation() {
        // Create a generator with a simple configuration
        let mut generator = Generator::new(4, 2, 0.8, Some(42));

        // Generate the logo
        generator.generate().unwrap();

        // Generate PNG
        let png_data = generate_png(&generator, 200, 200).unwrap();

        // Basic checks
        assert!(!png_data.is_empty());
        assert_eq!(&png_data[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]); // PNG magic number
    }
}
