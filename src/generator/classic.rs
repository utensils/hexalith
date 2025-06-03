use crate::generator::color::{ColorManager, Theme};
use crate::generator::shape::Shape;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Classic logo generator with fixed 24-triangle layout
pub struct ClassicGenerator {
    rng: ChaCha8Rng,
}

impl ClassicGenerator {
    pub fn new(seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(seed) => ChaCha8Rng::seed_from_u64(seed),
            None => ChaCha8Rng::from_entropy(),
        };

        Self { rng }
    }

    /// Get the exact triangle vertices for classic mode
    /// Returns a vector of 24 triangles, each with 3 vertices as (x, y) tuples
    pub fn get_triangle_vertices() -> Vec<[(f64, f64); 3]> {
        // Classic triangle dimensions
        let d = 120.0;
        let x = (60.0_f64.to_radians()).sin() * d; // 103.923
        let y = d / 2.0; // 60.0

        // Offsets to position the design
        let x_off = 120.0;
        let y_off = 10.0;

        let mut triangles = Vec::new();
        let mut id = 0;

        // Row 1: 2 triangles starting at (120, 10)
        triangles.extend(Self::generate_row(
            d,
            x,
            y,
            d + x_off,
            y_off,
            false,
            false,
            &mut id,
        ));

        // Row 2: 5 triangles starting at (223.923, 70)
        triangles.extend(Self::generate_row(
            d,
            x,
            y,
            x + x_off,
            y + y_off,
            true,
            false,
            &mut id,
        ));

        // Row 3: 7 triangles starting at (327.846, 130)
        triangles.extend(Self::generate_row(
            d,
            x,
            y,
            2.0 * x + x_off,
            2.0 * y + y_off,
            true,
            true,
            &mut id,
        ));

        // Row 4: 10 triangles starting at (327.846, 250)
        triangles.extend(Self::generate_row(
            d,
            x,
            y,
            2.0 * x + x_off,
            4.0 * y + y_off,
            false,
            true,
            &mut id,
        ));

        triangles
    }

    /// Generate a row of triangles in classic pattern
    #[allow(clippy::too_many_arguments)]
    fn generate_row(
        d: f64,
        x: f64,
        y: f64,
        x_off: f64,
        y_off: f64,
        big: bool,
        left: bool,
        id: &mut usize,
    ) -> Vec<[(f64, f64); 3]> {
        let mut triangles = Vec::new();
        let nr = if big { 3 } else { 2 };

        // Start with right-pointing triangle if not left
        if !left {
            triangles.push(Self::create_triangle(d, x, x_off, y_off, false));
            *id += 1;
        }

        // Generate pairs of triangles
        for v in 0..nr {
            let v_f = v as f64;
            // Left-pointing triangle
            triangles.push(Self::create_triangle(
                d,
                x,
                x_off - v_f * x,
                y_off + v_f * y,
                true,
            ));
            *id += 1;

            // Right-pointing triangle
            triangles.push(Self::create_triangle(
                d,
                x,
                x_off - v_f * x - x,
                y_off + v_f * y + y,
                false,
            ));
            *id += 1;
        }

        // End with left-pointing triangle if left
        if left {
            let v_f = nr as f64;
            triangles.push(Self::create_triangle(
                d,
                x,
                x_off - v_f * x,
                y_off + v_f * y,
                true,
            ));
            *id += 1;
        }

        triangles
    }

    /// Create a single triangle with specific coordinates
    fn create_triangle(
        d: f64,
        x: f64,
        offset_x: f64,
        offset_y: f64,
        left: bool,
    ) -> [(f64, f64); 3] {
        let top = (x + offset_x, offset_y);
        let bottom = (x + offset_x, d + offset_y);
        let side = (
            offset_x + if left { 0.0 } else { 2.0 * x },
            d / 2.0 + offset_y,
        );

        [top, bottom, side]
    }

    /// Generate two overlapping shapes in classic style
    pub fn generate_classic_shapes(&mut self, theme: Theme, opacity: f32) -> Vec<Shape> {
        // Classic shape definitions - each contains 6 triangles
        let shapes = vec![
            vec![9, 16, 17, 23, 22, 21],
            vec![17, 23, 22, 21, 15, 14],
            vec![6, 7, 8, 9, 15, 16],
            vec![7, 8, 14, 15, 16, 17],
            vec![9, 16, 17, 23, 14, 15],
            vec![2, 8, 9, 10, 11, 16],
            vec![5, 6, 7, 8, 14, 15],
            vec![8, 9, 15, 16, 17, 18],
            vec![6, 7, 13, 14, 15, 16, 19],
            vec![2, 3, 6, 7, 8, 9, 16],
            vec![1, 2, 8, 9, 15, 16, 17],
            vec![2, 7, 8, 9, 16, 17, 23],
            vec![9, 10, 15, 16, 17, 21],
            vec![7, 12, 13, 14, 15, 16],
            vec![2, 7, 8, 9, 10, 16],
            vec![6, 7, 8, 13, 14, 15],
            vec![9, 10, 11, 14, 15, 16],
            vec![9, 13, 14, 15, 16, 19],
            vec![9, 10, 13, 14, 15, 16],
            vec![7, 8, 9, 10, 13, 14],
            vec![7, 8, 9, 13, 14, 15],
            vec![7, 9, 13, 14, 15, 16],
            vec![8, 9, 10, 14, 15, 16],
            vec![2, 7, 8, 9, 14, 16],
            vec![8, 9, 14, 15, 16, 17],
            vec![2, 3, 6, 7, 8, 9],
            vec![7, 13, 14, 15, 16, 19],
        ];

        // Set up color manager
        let mut color_manager = ColorManager::with_theme(theme, Some(self.rng.gen()));

        // Get colors for the shapes
        let colors = if theme == Theme::Classic {
            // Use classic color palette
            let classic_colors = [
                "#FFCC09", "#F68A21", "#E4272B", "#E81F6F", "#BD3D93", "#71459B", "#4D499C",
                "#3960A9", "#20B7E8", "#46B78C", "#49B650", "#78BF44",
            ];

            let idx1 = self.rng.gen_range(0..classic_colors.len());
            let idx2 = (idx1 + self.rng.gen_range(1..classic_colors.len())) % classic_colors.len();

            vec![
                classic_colors[idx1].to_string(),
                classic_colors[idx2].to_string(),
            ]
        } else {
            color_manager.get_random_colors(2)
        };

        let color1 = colors[0].clone();
        let color2 = colors[1].clone();

        // Keep generating until we get shapes that overlap by at least 2 triangles
        let mut attempts = 0;
        loop {
            // Select two random shape patterns
            let shape1_idx = self.rng.gen_range(0..shapes.len());
            let mut shape2_idx = self.rng.gen_range(0..shapes.len());

            // Ensure different patterns
            while shape2_idx == shape1_idx {
                shape2_idx = self.rng.gen_range(0..shapes.len());
            }

            let pattern1 = &shapes[shape1_idx];
            let pattern2 = &shapes[shape2_idx];

            // Find overlapping triangles
            let overlap: Vec<usize> = pattern1
                .iter()
                .filter(|&&t| pattern2.contains(&t))
                .cloned()
                .collect();

            // Check if we have enough overlap (at least 2 triangles)
            if overlap.len() >= 2 || attempts > 100 {
                // Calculate blend color for overlaps using simple averaging
                let (r1, g1, b1) = ColorManager::hex_to_rgb(&color1);
                let (r2, g2, b2) = ColorManager::hex_to_rgb(&color2);
                let blend = ColorManager::rgb_to_hex(
                    ((r1 as u16 + r2 as u16) / 2) as u8,
                    ((g1 as u16 + g2 as u16) / 2) as u8,
                    ((b1 as u16 + b2 as u16) / 2) as u8,
                );

                // Create the shapes
                let mut shape1 = Shape::new(color1.clone(), opacity);
                let mut shape2 = Shape::new(color2.clone(), opacity);
                let mut overlap_shape = Shape::new(blend, opacity);

                // Add non-overlapping triangles to shape1
                for &triangle_id in pattern1 {
                    if !overlap.contains(&triangle_id) {
                        shape1.add_cell(triangle_id);
                    }
                }

                // Add non-overlapping triangles to shape2
                for &triangle_id in pattern2 {
                    if !overlap.contains(&triangle_id) {
                        shape2.add_cell(triangle_id);
                    }
                }

                // Add overlapping triangles to overlap shape
                for &triangle_id in &overlap {
                    overlap_shape.add_cell(triangle_id);
                }

                // Return shapes: shape1, shape2, and overlap (if any)
                let mut result = vec![shape1, shape2];
                if !overlap_shape.is_empty() {
                    result.push(overlap_shape);
                }

                return result;
            }

            attempts += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classic_triangle_count() {
        let triangles = ClassicGenerator::get_triangle_vertices();
        assert_eq!(
            triangles.len(),
            24,
            "Classic mode should have exactly 24 triangles"
        );
    }

    #[test]
    fn test_classic_shape_generation() {
        let mut generator = ClassicGenerator::new(Some(42));
        let shapes = generator.generate_classic_shapes(Theme::Classic, 0.8);

        // Should generate 2 or 3 shapes (2 main shapes + optional overlap)
        assert!(shapes.len() >= 2, "Should have at least 2 shapes");
        assert!(
            shapes.len() <= 3,
            "Should have at most 3 shapes (2 shapes + overlap)"
        );

        // Check that shapes have triangles
        for shape in &shapes {
            assert!(!shape.is_empty(), "Shapes should not be empty");
        }
    }

    #[test]
    fn test_classic_shapes_overlap() {
        // Test multiple seeds to ensure we get overlapping shapes
        let mut found_overlap = false;

        for seed in 0..20 {
            let mut generator = ClassicGenerator::new(Some(seed));
            let shapes = generator.generate_classic_shapes(Theme::Classic, 0.8);

            if shapes.len() == 3 {
                // We have an overlap shape
                found_overlap = true;

                // The overlap shape should have at least 2 triangles
                let overlap_shape = &shapes[2];
                assert!(
                    overlap_shape.cells.len() >= 2,
                    "Overlap shape should have at least 2 triangles"
                );
                break;
            }
        }

        assert!(
            found_overlap,
            "Should find overlapping shapes in multiple attempts"
        );
    }

    #[test]
    fn test_classic_deterministic() {
        // Same seed should produce same shapes
        let mut gen1 = ClassicGenerator::new(Some(42));
        let mut gen2 = ClassicGenerator::new(Some(42));

        let shapes1 = gen1.generate_classic_shapes(Theme::Classic, 0.8);
        let shapes2 = gen2.generate_classic_shapes(Theme::Classic, 0.8);

        assert_eq!(
            shapes1.len(),
            shapes2.len(),
            "Same seed should produce same number of shapes"
        );

        for (s1, s2) in shapes1.iter().zip(shapes2.iter()) {
            assert_eq!(
                s1.cells, s2.cells,
                "Same seed should produce same triangle arrangements"
            );
            assert_eq!(s1.color, s2.color, "Same seed should produce same colors");
        }
    }
}
