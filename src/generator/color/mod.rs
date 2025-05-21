use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;

/// Manages color selection and blending for logo generation
pub struct ColorManager {
    palette: Vec<String>,
    rng: ChaCha8Rng,
}

/// Available color themes for logo generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Mesos,   // Original Mesos style colors
    Google,  // Google brand colors
    Blues,   // Blue color theme
    Greens,  // Green color theme
    Reds,    // Red color theme
    Purples, // Purple color theme
    Rainbow, // All colors of the rainbow
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
        // Add extra randomness by combining seed with timestamp nanoseconds
        let rng = match seed {
            Some(seed) => {
                // Get the current timestamp's nanoseconds
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .subsec_nanos();

                // Combine seed and timestamp for additional randomness
                // But only use a portion of the nanoseconds to preserve some determinism
                let combined_seed = seed.wrapping_add((now % 10000) as u64);
                ChaCha8Rng::seed_from_u64(combined_seed)
            }
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
    #[allow(dead_code)]
    pub fn with_theme_name(theme_name: &str, seed: Option<u64>) -> Self {
        let theme = Theme::from(theme_name);
        Self::with_theme(theme, seed)
    }

    /// Initialize with the default (Mesos) theme
    #[allow(dead_code)]
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

    /// Get a color that's different from the provided colors
    pub fn get_different_color(&mut self, existing_colors: &[String]) -> String {
        if existing_colors.is_empty() {
            return self.get_random_color();
        }

        let mut color = self.get_random_color();
        let max_attempts = 20; // Prevent infinite loop in cases with limited palette
        let mut attempts = 0;

        while existing_colors.contains(&color) && attempts < max_attempts {
            color = self.get_random_color();
            attempts += 1;
        }

        color
    }

    /// Get a color for a shape that's harmonious with the design
    /// This avoids using the same color for adjacent shapes
    #[allow(dead_code)]
    pub fn get_color_avoiding_adjacency(
        &mut self,
        grid: &crate::generator::grid::TriangularGrid,
        shape_cells: &[usize],
        existing_shapes: &[crate::generator::shape::Shape],
    ) -> String {
        // If no existing shapes, just return a random color
        if existing_shapes.is_empty() {
            return self.get_random_color();
        }

        // Build an adjacency list to track which colors to avoid
        let mut adjacent_colors = Vec::new();

        // For each cell in our shape
        for &cell_id in shape_cells {
            // Get all adjacent cells
            let adjacent_cells = grid.adjacent_cells(cell_id);

            // For each adjacent cell, check if it belongs to an existing shape
            for &adj_cell in &adjacent_cells {
                for existing_shape in existing_shapes {
                    if existing_shape.contains_cell(adj_cell) {
                        // Add the color of the adjacent shape to our avoid list
                        adjacent_colors.push(existing_shape.color.clone());
                        break; // Once we find a shape that contains this cell, we can stop checking
                    }
                }
            }
        }

        // Remove duplicates
        adjacent_colors.sort_unstable();
        adjacent_colors.dedup();

        // Get a color different from all adjacent colors
        self.get_different_color(&adjacent_colors)
    }

    /// Assign optimal colors to a set of shapes to ensure visual harmony
    pub fn assign_harmonious_colors(
        &mut self,
        grid: &crate::generator::grid::TriangularGrid,
        shapes: &mut [crate::generator::shape::Shape],
    ) {
        // Create a map of shape index -> adjacent shape indices
        let mut adjacency_map: HashMap<usize, Vec<usize>> = HashMap::new();

        // For each shape, find adjacent shapes
        for i in 0..shapes.len() {
            let mut adjacent_shapes = Vec::new();

            // Check each cell in this shape
            for &cell_id in &shapes[i].cells {
                // Get adjacent cells
                let adjacent_cells = grid.adjacent_cells(cell_id);

                // For each adjacent cell, check if it belongs to another shape
                for &adj_cell in &adjacent_cells {
                    for (j, shape) in shapes.iter().enumerate() {
                        if i != j && shape.contains_cell(adj_cell) && !adjacent_shapes.contains(&j)
                        {
                            adjacent_shapes.push(j);
                            break;
                        }
                    }
                }
            }

            adjacency_map.insert(i, adjacent_shapes);
        }

        // Assign colors using a greedy algorithm (Welsh-Powell)
        let mut available_colors = self.get_random_colors(self.palette.len().min(shapes.len() + 3));
        let mut assigned_colors: HashMap<usize, String> = HashMap::new();

        // Sort shapes by number of adjacencies (descending)
        let mut shape_indices: Vec<usize> = (0..shapes.len()).collect();
        shape_indices.sort_by(|&a, &b| {
            let a_adj = adjacency_map.get(&a).map_or(0, |v| v.len());
            let b_adj = adjacency_map.get(&b).map_or(0, |v| v.len());
            b_adj.cmp(&a_adj) // Descending order
        });

        // Assign colors to shapes
        for &shape_idx in &shape_indices {
            // Get colors of adjacent shapes
            let adjacent_colors: Vec<String> =
                if let Some(adj_indices) = adjacency_map.get(&shape_idx) {
                    adj_indices
                        .iter()
                        .filter_map(|&adj_idx| assigned_colors.get(&adj_idx).cloned())
                        .collect()
                } else {
                    Vec::new()
                };

            // Find first available color not used by adjacent shapes
            let mut chosen_color = None;
            for color in &available_colors {
                if !adjacent_colors.contains(color) {
                    chosen_color = Some(color.clone());
                    break;
                }
            }

            // If no suitable color found, add a new random one that's different from adjacent
            let color = match chosen_color {
                Some(color) => color,
                None => {
                    let new_color = self.get_different_color(&adjacent_colors);
                    available_colors.push(new_color.clone());
                    new_color
                }
            };

            // Assign the color
            assigned_colors.insert(shape_idx, color);
        }

        // Update the actual shapes with assigned colors
        for (i, shape) in shapes.iter_mut().enumerate() {
            if let Some(color) = assigned_colors.get(&i) {
                shape.color = color.clone();
            }
        }
    }

    /// Get a pair of colors with a blended color for overlapping regions
    /// Returns (color1, color2, blend)
    #[allow(dead_code)]
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

    /// Calculate color contrast ratio between two colors
    pub fn color_contrast(color1: &str, color2: &str) -> f64 {
        // Convert to RGB
        let (r1, g1, b1) = Self::hex_to_rgb(color1);
        let (r2, g2, b2) = Self::hex_to_rgb(color2);

        // Calculate relative luminance
        let l1 = Self::relative_luminance(r1, g1, b1);
        let l2 = Self::relative_luminance(r2, g2, b2);

        // Calculate contrast ratio
        if l1 > l2 {
            (l1 + 0.05) / (l2 + 0.05)
        } else {
            (l2 + 0.05) / (l1 + 0.05)
        }
    }

    /// Calculate relative luminance for contrast calculation
    fn relative_luminance(r: u8, g: u8, b: u8) -> f64 {
        // Convert RGB to linear values first
        let r_linear = Self::to_linear(r as f64 / 255.0);
        let g_linear = Self::to_linear(g as f64 / 255.0);
        let b_linear = Self::to_linear(b as f64 / 255.0);

        // Calculate luminance (per WCAG formula)
        0.2126 * r_linear + 0.7152 * g_linear + 0.0722 * b_linear
    }

    /// Convert sRGB value to linear RGB value
    fn to_linear(value: f64) -> f64 {
        if value <= 0.03928 {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
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
    #[allow(dead_code)]
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

    #[test]
    fn test_get_different_color() {
        let mut manager = ColorManager::default(Some(42)); // Fixed seed for deterministic testing

        let existing_colors = vec![
            "#FFCC09".to_string(),
            "#F68A21".to_string(),
            "#E42728".to_string(),
        ];

        let different_color = manager.get_different_color(&existing_colors);
        assert!(!existing_colors.contains(&different_color));
    }

    #[test]
    fn test_color_contrast() {
        // Test high contrast (black/white)
        let contrast = ColorManager::color_contrast("#FFFFFF", "#000000");
        assert!(contrast > 20.0); // Should be 21.0

        // Test low contrast (similar colors)
        let contrast = ColorManager::color_contrast("#FF0000", "#FF0001");
        assert!(contrast < 1.1); // Should be very close to 1.0
    }

    #[test]
    fn test_with_theme_name() {
        // Test with a valid theme name
        let manager = ColorManager::with_theme_name("blues", Some(42));
        let palette = manager.palette();

        // Verify colors are from the blues theme
        assert!(palette.iter().any(|color| color.to_uppercase() == "#1E88E5"
            || color.to_uppercase() == "#2196F3"
            || color.to_uppercase() == "#0D47A1"));

        // Test with another theme
        let manager = ColorManager::with_theme_name("google", Some(42));
        let palette = manager.palette();

        // Verify colors are from the google theme
        assert!(palette
            .iter()
            .any(|color| color.to_uppercase() == "#4285F4"));
    }

    #[test]
    fn test_default() {
        // Test default theme (should be Mesos)
        let manager = ColorManager::default(Some(42));
        let palette = manager.palette();

        // Verify colors are from the Mesos theme
        assert!(palette
            .iter()
            .any(|color| color.to_uppercase() == "#FFCC09"));
        assert!(palette
            .iter()
            .any(|color| color.to_uppercase() == "#E42728"));
    }

    #[test]
    fn test_colors_with_blend() {
        let mut manager = ColorManager::default(Some(42));

        let (color1, color2, blend) = manager.get_colors_with_blend();

        // Verify the colors are from the palette
        assert!(manager.palette().contains(&color1));
        assert!(manager.palette().contains(&color2));

        // Verify that color1 and color2 are different
        assert_ne!(color1, color2);

        // Verify the blend logic
        let (r1, g1, b1) = ColorManager::hex_to_rgb(&color1);
        let (r2, g2, b2) = ColorManager::hex_to_rgb(&color2);
        let (rb, gb, bb) = ColorManager::hex_to_rgb(&blend);

        // The blend should be the average of the RGB values
        assert_eq!(rb, ((r1 as u16 + r2 as u16) / 2) as u8);
        assert_eq!(gb, ((g1 as u16 + g2 as u16) / 2) as u8);
        assert_eq!(bb, ((b1 as u16 + b2 as u16) / 2) as u8);
    }

    #[test]
    fn test_color_avoiding_adjacency() {
        use crate::generator::grid::triangular::TriangularGrid;
        use crate::generator::shape::Shape;

        // Create a triangular grid for testing
        let grid = TriangularGrid::new(100.0, 2);

        // Create a ColorManager
        let mut manager = ColorManager::default(Some(42));

        // Test with no existing shapes (should just return a random color)
        let shape_cells = vec![0, 1, 2];
        let existing_shapes: Vec<Shape> = vec![];
        let color = manager.get_color_avoiding_adjacency(&grid, &shape_cells, &existing_shapes);

        assert!(manager.palette().contains(&color));

        // Create a shape with a known color
        let mut shape1 = Shape::new("#FF0000".to_string(), 0.8);
        shape1.add_cell(3);
        shape1.add_cell(4);

        // Create another shape with a different color
        let mut shape2 = Shape::new("#00FF00".to_string(), 0.8);
        shape2.add_cell(6);
        shape2.add_cell(7);

        let existing_shapes = vec![shape1, shape2];

        // Get a color avoiding adjacency to these shapes
        let color = manager.get_color_avoiding_adjacency(&grid, &shape_cells, &existing_shapes);

        // The color should be from the palette
        assert!(manager.palette().contains(&color));

        // The color should be different from the existing shapes' colors
        assert_ne!(color, "#FF0000");
        assert_ne!(color, "#00FF00");
    }
}
