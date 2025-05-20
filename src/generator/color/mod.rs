use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Manages color selection and blending for logo generation
pub struct ColorManager {
    palette: Vec<String>,
    rng: ChaCha8Rng,
}

impl ColorManager {
    pub fn new(palette: Vec<String>, seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(seed) => ChaCha8Rng::seed_from_u64(seed),
            None => ChaCha8Rng::from_entropy(),
        };

        Self { palette, rng }
    }

    pub fn default(seed: Option<u64>) -> Self {
        Self::new(
            vec![
                // Original hexagonal logo generator colors
                "#FFCC09".to_string(), // Yellow
                "#F68A21".to_string(), // Orange
                "#E42728".to_string(), // Red
                "#E81F6F".to_string(), // Magenta
                "#BD3D93".to_string(), // Pink
                "#71459B".to_string(), // Purple
                "#4D499C".to_string(), // Dark Blue
                "#3960A9".to_string(), // Medium Blue
                "#20B7E8".to_string(), // Light Blue
                "#46B78C".to_string(), // Teal
                "#49B650".to_string(), // Green
                "#78BF44".to_string(), // Light Green
                // Google colors for consistency with previous version
                "#4285F4".to_string(), // Google Blue
                "#EA4335".to_string(), // Google Red
                "#FBBC05".to_string(), // Google Yellow
                "#34A853".to_string(), // Google Green
            ],
            seed,
        )
    }

    // Public methods that are directly used in the application

    /// Get a random color from the palette
    pub fn get_random_color(&mut self) -> String {
        let idx = self.rng.gen_range(0..self.palette.len());
        self.palette[idx].clone()
    }

    /// Get a specific number of random colors
    pub fn get_random_colors(&mut self, count: usize) -> Vec<String> {
        let mut colors = Vec::with_capacity(count);
        for _ in 0..count {
            colors.push(self.get_random_color());
        }
        colors
    }
    
    /// Get a pair of colors with a blended color for overlapping regions
    /// Returns (color1, color2, blend)
    pub fn get_colors_with_blend(&mut self) -> (String, String, String) {
        // Get two distinct random colors
        let color1 = self.get_random_color();
        let mut color2 = self.get_random_color();
        while color2 == color1 {
            color2 = self.get_random_color();
        }
        
        // Create a blended color by averaging the RGB values
        let (r1, g1, b1) = Self::hex_to_rgb(&color1);
        let (r2, g2, b2) = Self::hex_to_rgb(&color2);
        
        let blend_r = (r1 as u16 + r2 as u16) / 2;
        let blend_g = (g1 as u16 + g2 as u16) / 2;
        let blend_b = (b1 as u16 + b2 as u16) / 2;
        
        let blend = Self::rgb_to_hex(blend_r as u8, blend_g as u8, blend_b as u8);
        
        (color1, color2, blend)
    }

    // Helper methods used only in tests
    #[cfg(test)]
    pub fn palette(&self) -> &[String] {
        &self.palette
    }

    pub fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
        let hex = hex.trim_start_matches('#');

        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);

        (r, g, b)
    }

    pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    /// Blend two colors together with a given opacity
    pub fn blend_colors(color1: &str, color2: &str, opacity: f32) -> String {
        let (r1, g1, b1) = Self::hex_to_rgb(color1);
        let (r2, g2, b2) = Self::hex_to_rgb(color2);

        let alpha = opacity.clamp(0.0, 1.0) as f64;

        let r = (r1 as f64 * (1.0 - alpha) + r2 as f64 * alpha).round() as u8;
        let g = (g1 as f64 * (1.0 - alpha) + g2 as f64 * alpha).round() as u8;
        let b = (b1 as f64 * (1.0 - alpha) + b2 as f64 * alpha).round() as u8;

        Self::rgb_to_hex(r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_conversion() {
        let hex = "#FF5500";
        let (r, g, b) = ColorManager::hex_to_rgb(hex);
        assert_eq!(r, 255);
        assert_eq!(g, 85);
        assert_eq!(b, 0);

        let hex2 = ColorManager::rgb_to_hex(r, g, b);
        assert_eq!(hex2.to_uppercase(), "#FF5500");
    }

    #[test]
    fn test_color_blending() {
        let color1 = "#FF0000"; // Red
        let color2 = "#0000FF"; // Blue

        // 50% blend should give purple
        let blended = ColorManager::blend_colors(color1, color2, 0.5);
        assert_eq!(blended.to_uppercase(), "#800080");

        // 0% opacity should give color1
        let blended = ColorManager::blend_colors(color1, color2, 0.0);
        assert_eq!(blended.to_uppercase(), "#FF0000");

        // 100% opacity should give color2
        let blended = ColorManager::blend_colors(color1, color2, 1.0);
        assert_eq!(blended.to_uppercase(), "#0000FF");
    }

    #[test]
    fn test_random_color_selection() {
        let mut manager = ColorManager::default(Some(42)); // Fixed seed for deterministic testing

        // Get a few random colors and ensure they're from the palette
        let color1 = manager.get_random_color();
        let color2 = manager.get_random_color();

        assert!(manager.palette().contains(&color1));
        assert!(manager.palette().contains(&color2));

        // Get multiple colors at once
        let colors = manager.get_random_colors(3);
        assert_eq!(colors.len(), 3);

        for color in colors {
            assert!(manager.palette().contains(&color));
        }
    }
}
