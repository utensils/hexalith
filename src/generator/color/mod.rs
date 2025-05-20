use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Manages color selection and blending for logo generation
pub struct ColorManager {
    palette: Vec<String>,
    rng: ChaCha8Rng,
}

/// Available color themes for logo generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Mesos,    // Original Mesos style colors
    Google,   // Google brand colors
    Blues,    // Blue color theme
    Greens,   // Green color theme
    Reds,     // Red color theme
    Purples,  // Purple color theme
    Rainbow,  // All colors of the rainbow
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Mesos => write!(f, "mesos"),
            Theme::Google => write!(f, "google"),
            Theme::Blues => write!(f, "blues"),
            Theme::Greens => write!(f, "greens"),
            Theme::Reds => write!(f, "reds"),
            Theme::Purples => write!(f, "purples"),
            Theme::Rainbow => write!(f, "rainbow"),
        }
    }
}

impl From<&str> for Theme {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "mesos" => Theme::Mesos,
            "google" => Theme::Google,
            "blues" => Theme::Blues,
            "greens" => Theme::Greens,
            "reds" => Theme::Reds,
            "purples" => Theme::Purples,
            "rainbow" => Theme::Rainbow,
            _ => Theme::Mesos, // Default to Mesos theme if unknown
        }
    }
}

impl ColorManager {
    pub fn new(palette: Vec<String>, seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(seed) => ChaCha8Rng::seed_from_u64(seed),
            None => ChaCha8Rng::from_entropy(),
        };

        Self { palette, rng }
    }

    /// Get a list of available theme names
    pub fn available_themes() -> Vec<String> {
        vec![
            "mesos".to_string(),
            "google".to_string(), 
            "blues".to_string(),
            "greens".to_string(),
            "reds".to_string(),
            "purples".to_string(),
            "rainbow".to_string(),
        ]
    }

    /// Initialize with a specified theme
    pub fn with_theme(theme: Theme, seed: Option<u64>) -> Self {
        match theme {
            Theme::Mesos => Self::mesos_theme(seed),
            Theme::Google => Self::google_theme(seed),
            Theme::Blues => Self::blues_theme(seed),
            Theme::Greens => Self::greens_theme(seed),
            Theme::Reds => Self::reds_theme(seed),
            Theme::Purples => Self::purples_theme(seed),
            Theme::Rainbow => Self::rainbow_theme(seed),
        }
    }

    /// Create a ColorManager with the specified theme by name
    pub fn with_theme_name(theme_name: &str, seed: Option<u64>) -> Self {
        let theme = Theme::from(theme_name);
        Self::with_theme(theme, seed)
    }

    /// Initialize with the default (Mesos) theme
    pub fn default(seed: Option<u64>) -> Self {
        Self::mesos_theme(seed)
    }

    /// Mesos theme - original colors from the Mesos logo generator
    pub fn mesos_theme(seed: Option<u64>) -> Self {
        Self::new(
            vec![
                // Original hexagonal logo generator colors from the JS implementation
                "#FFCC09".to_string(), // Yellow [255, 204, 9]
                "#F68A21".to_string(), // Orange [246, 138, 33]
                "#E42728".to_string(), // Red [228, 39, 43]
                "#E81F6F".to_string(), // Magenta [232, 31, 111]
                "#BD3D93".to_string(), // Pink [189, 61, 147]
                "#71459B".to_string(), // Purple [113, 69, 155]
                "#4D499C".to_string(), // Dark Blue [77, 73, 156]
                "#3960A9".to_string(), // Medium Blue [57, 96, 169]
                "#20B7E8".to_string(), // Light Blue [32, 183, 232]
                "#46B78C".to_string(), // Teal [70, 183, 140]
                "#49B650".to_string(), // Green [73, 182, 80]
                "#78BF44".to_string(), // Light Green [120, 191, 68]
                // Additional colors for variety
                "#B3675E".to_string(), // Rust
                "#3EAF51".to_string(), // Forest Green
                "#5A4FCF".to_string(), // Royal Purple
            ],
            seed,
        )
    }
    
    /// Google theme - based on Google's brand colors
    pub fn google_theme(seed: Option<u64>) -> Self {
        Self::new(
            vec![
                "#4285F4".to_string(), // Google Blue
                "#EA4335".to_string(), // Google Red
                "#FBBC05".to_string(), // Google Yellow
                "#34A853".to_string(), // Google Green
                "#1A73E8".to_string(), // Google Blue (alternate)
                "#D93025".to_string(), // Google Red (darker)
                "#F9AB00".to_string(), // Google Yellow (darker)
                "#1E8E3E".to_string(), // Google Green (darker)
                "#174EA6".to_string(), // Google Blue (darkest)
                "#A50E0E".to_string(), // Google Red (darkest)
                "#E37400".to_string(), // Google Yellow (darkest)
                "#0D652D".to_string(), // Google Green (darkest)
                "#5BB974".to_string(), // Light Green
                "#81C995".to_string(), // Very Light Green
                "#8AB4F8".to_string(), // Light Blue
            ],
            seed,
        )
    }
    
    /// Blues theme - various shades of blue
    pub fn blues_theme(seed: Option<u64>) -> Self {
        Self::new(
            vec![
                "#0D47A1".to_string(), // Deep Blue
                "#1565C0".to_string(), // Dark Blue
                "#1976D2".to_string(), // Medium Blue
                "#1E88E5".to_string(), // Blue
                "#2196F3".to_string(), // Light Blue
                "#42A5F5".to_string(), // Lighter Blue
                "#64B5F6".to_string(), // Very Light Blue
                "#90CAF9".to_string(), // Pale Blue
                "#BBDEFB".to_string(), // Extremely Light Blue
                "#2962FF".to_string(), // Accent Blue
                "#0277BD".to_string(), // Light Blue (darker)
                "#01579B".to_string(), // Light Blue (darkest)
                "#039BE5".to_string(), // Light Blue (medium)
                "#03A9F4".to_string(), // Light Blue (lighter)
                "#29B6F6".to_string(), // Light Blue (lightest)
            ],
            seed,
        )
    }
    
    /// Greens theme - various shades of green
    pub fn greens_theme(seed: Option<u64>) -> Self {
        Self::new(
            vec![
                "#1B5E20".to_string(), // Deep Green
                "#2E7D32".to_string(), // Dark Green
                "#388E3C".to_string(), // Medium Green
                "#43A047".to_string(), // Green
                "#4CAF50".to_string(), // Light Green
                "#66BB6A".to_string(), // Lighter Green
                "#81C784".to_string(), // Very Light Green
                "#A5D6A7".to_string(), // Pale Green
                "#C8E6C9".to_string(), // Extremely Light Green
                "#00C853".to_string(), // Accent Green
                "#00695C".to_string(), // Teal Green
                "#00796B".to_string(), // Medium Teal
                "#00897B".to_string(), // Light Teal
                "#009688".to_string(), // Teal
                "#26A69A".to_string(), // Light Teal Green
            ],
            seed,
        )
    }
    
    /// Reds theme - various shades of red and orange
    pub fn reds_theme(seed: Option<u64>) -> Self {
        Self::new(
            vec![
                "#B71C1C".to_string(), // Deep Red
                "#C62828".to_string(), // Dark Red
                "#D32F2F".to_string(), // Medium Red
                "#E53935".to_string(), // Red
                "#F44336".to_string(), // Light Red
                "#EF5350".to_string(), // Lighter Red
                "#E57373".to_string(), // Very Light Red
                "#EF9A9A".to_string(), // Pale Red
                "#FFCDD2".to_string(), // Extremely Light Red
                "#DD2C00".to_string(), // Accent Red
                "#BF360C".to_string(), // Deep Orange
                "#E64A19".to_string(), // Dark Orange
                "#F4511E".to_string(), // Medium Orange
                "#FF5722".to_string(), // Orange
                "#FF7043".to_string(), // Light Orange
            ],
            seed,
        )
    }
    
    /// Purples theme - various shades of purple and pink
    pub fn purples_theme(seed: Option<u64>) -> Self {
        Self::new(
            vec![
                "#4A148C".to_string(), // Deep Purple
                "#6A1B9A".to_string(), // Dark Purple
                "#7B1FA2".to_string(), // Medium Purple
                "#8E24AA".to_string(), // Purple
                "#9C27B0".to_string(), // Light Purple
                "#AB47BC".to_string(), // Lighter Purple
                "#BA68C8".to_string(), // Very Light Purple
                "#CE93D8".to_string(), // Pale Purple
                "#E1BEE7".to_string(), // Extremely Light Purple
                "#880E4F".to_string(), // Deep Pink
                "#AD1457".to_string(), // Dark Pink
                "#C2185B".to_string(), // Medium Pink
                "#D81B60".to_string(), // Pink
                "#E91E63".to_string(), // Light Pink
                "#EC407A".to_string(), // Lighter Pink
            ],
            seed,
        )
    }
    
    /// Rainbow theme - full spectrum of colors
    pub fn rainbow_theme(seed: Option<u64>) -> Self {
        Self::new(
            vec![
                "#FF0000".to_string(), // Red
                "#FF4500".to_string(), // Orange Red
                "#FF8C00".to_string(), // Dark Orange
                "#FFA500".to_string(), // Orange
                "#FFD700".to_string(), // Gold
                "#FFFF00".to_string(), // Yellow
                "#ADFF2F".to_string(), // Green Yellow
                "#32CD32".to_string(), // Lime Green
                "#008000".to_string(), // Green
                "#00FF7F".to_string(), // Spring Green
                "#00FFFF".to_string(), // Cyan
                "#1E90FF".to_string(), // Dodger Blue
                "#0000FF".to_string(), // Blue
                "#4B0082".to_string(), // Indigo
                "#8A2BE2".to_string(), // Blue Violet
                "#FF00FF".to_string(), // Magenta
                "#C71585".to_string(), // Medium Violet Red
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
