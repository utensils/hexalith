mod color;
pub mod grid;
pub mod shape;

use crate::Result;
use color::ColorManager;
use grid::TriangularGrid;
use shape::{Shape, ShapeGenerator};
use std::collections::HashSet;

// Re-export Theme enum for use in other modules
pub use color::Theme;

pub struct Generator {
    grid_size: u8,
    shapes_count: u8,
    opacity: f32,
    seed: Option<u64>,
    grid: Option<TriangularGrid>,
    shapes: Vec<Shape>,
    theme: Theme,
    allow_overlap: bool,
}

impl Generator {
    pub fn new(grid_size: u8, shapes_count: u8, opacity: f32, seed: Option<u64>) -> Self {
        Self {
            grid_size: grid_size.clamp(2, 8),
            shapes_count: shapes_count.clamp(1, 10),
            opacity: opacity.clamp(0.0, 1.0),
            seed,
            grid: None,
            shapes: Vec::new(),
            theme: Theme::Mesos, // Set Mesos as the default theme
            allow_overlap: false,
        }
    }

    /// Set the color theme by theme enum
    pub fn set_theme(&mut self, theme: Theme) -> &mut Self {
        self.theme = theme;
        self
    }

    /// Set the color theme by name
    pub fn set_color_scheme(&mut self, color_scheme: &str) -> &mut Self {
        self.theme = Theme::from(color_scheme);
        self
    }

    /// Get a list of available theme names
    pub fn available_themes() -> Vec<String> {
        ColorManager::available_themes()
    }

    pub fn set_allow_overlap(&mut self, allow_overlap: bool) -> &mut Self {
        self.allow_overlap = allow_overlap;
        self
    }

    pub fn generate(&mut self) -> Result<()> {
        // Initialize the triangular grid
        let grid = TriangularGrid::new(100.0, self.grid_size);
        self.grid = Some(grid);

        // Generate shapes
        if let Some(grid) = &self.grid {
            // Set up color manager with the selected theme
            let mut color_manager = ColorManager::with_theme(self.theme, self.seed);

            // Calculate shape size based on grid density
            // Higher density = smaller shapes
            let total_cells = grid.cell_count();

            // With grid density of 2, we have exactly 24 cells, like the original logo generator
            // Let's adjust our size range to work well with both small and large grid densities
            let min_size = if self.grid_size <= 2 {
                // For grid_size 2 (24 cells total), use 2-5 cells per shape
                2
            } else {
                (total_cells as f32 * 0.01).round() as usize
            };

            let max_size = if self.grid_size <= 2 {
                // For grid_size 2, limit the max size to keep multiple shapes visible
                5.min(total_cells / self.shapes_count as usize)
            } else {
                (total_cells as f32 * 0.05).round() as usize
            };

            let size_range = (min_size, max_size.max(min_size + 1));

            // Generate the shapes
            let mut shape_generator = ShapeGenerator::new(grid, self.seed);

            if self.allow_overlap && self.shapes_count >= 2 {
                // Generate overlapping shapes with improved algorithms

                // Get colors with high contrast
                let available_colors = color_manager.get_random_colors(self.palette_size());

                // Take the first color
                let color1 = available_colors[0].clone();

                // Find the color with highest contrast against the first color
                let color2 = {
                    let mut best_color = available_colors[1].clone();
                    let mut best_contrast = ColorManager::color_contrast(&color1, &best_color);

                    for color in available_colors.iter().skip(2) {
                        let contrast = ColorManager::color_contrast(&color1, color);
                        if contrast > best_contrast {
                            best_contrast = contrast;
                            best_color = color.clone();
                        }
                    }

                    best_color
                };

                // Generate the blended color for overlaps
                let (r1, g1, b1) = ColorManager::hex_to_rgb(&color1);
                let (r2, g2, b2) = ColorManager::hex_to_rgb(&color2);

                let blend_r = (r1 as u16 + r2 as u16) / 2;
                let blend_g = (g1 as u16 + g2 as u16) / 2;
                let blend_b = (b1 as u16 + b2 as u16) / 2;

                let blend = ColorManager::rgb_to_hex(blend_r as u8, blend_g as u8, blend_b as u8);

                // Generate two shapes with better aesthetics
                let shape1 = shape_generator.generate_balanced_shape(
                    color1.clone(),
                    self.opacity,
                    size_range.1, // Use larger size for better overlap chance
                );

                let shape2 = shape_generator.generate_balanced_shape(
                    color2.clone(),
                    self.opacity,
                    size_range.1,
                );

                // Find overlapping cells
                let mut overlap_cells = Vec::new();
                let mut overlap_shape = Shape::new(blend, self.opacity);

                for &cell1 in &shape1.cells {
                    if shape2.cells.contains(&cell1) {
                        overlap_cells.push(cell1);
                        overlap_shape.add_cell(cell1);
                    }
                }

                // Add the shapes to our collection
                // First add non-overlapping parts of each shape
                let mut shape1_no_overlap = Shape::new(color1.clone(), self.opacity);
                let mut shape2_no_overlap = Shape::new(color2.clone(), self.opacity);

                for &cell in &shape1.cells {
                    if !overlap_cells.contains(&cell) {
                        shape1_no_overlap.add_cell(cell);
                    }
                }

                for &cell in &shape2.cells {
                    if !overlap_cells.contains(&cell) {
                        shape2_no_overlap.add_cell(cell);
                    }
                }

                self.shapes.push(shape1_no_overlap);
                self.shapes.push(shape2_no_overlap);

                // Only add the overlap if it's not empty
                if !overlap_cells.is_empty() {
                    self.shapes.push(overlap_shape);
                }

                // Create a set of cells already used
                let mut used_cells = HashSet::new();
                for shape in &self.shapes {
                    for &cell in &shape.cells {
                        used_cells.insert(cell);
                    }
                }

                // Add additional shapes if needed with improved color selection
                if self.shapes_count > 2 {
                    // Get colors for additional shapes
                    let additional_colors_needed = (self.shapes_count - 2) as usize;

                    // If there are other colors in the initial set, use those first
                    let mut additional_colors = Vec::new();

                    // Filter out colors we've already used
                    let used_colors = [color1.clone(), color2.clone()];

                    // Add remaining colors from available_colors
                    for color in available_colors {
                        if !used_colors.contains(&color) && !additional_colors.contains(&color) {
                            additional_colors.push(color);
                            if additional_colors.len() >= additional_colors_needed {
                                break;
                            }
                        }
                    }

                    // If we still need more colors, get random ones that are different from existing
                    while additional_colors.len() < additional_colors_needed {
                        let current_colors: Vec<String> =
                            self.shapes.iter().map(|s| s.color.clone()).collect();

                        let new_color = color_manager.get_different_color(&current_colors);
                        additional_colors.push(new_color);
                    }

                    // Generate the additional shapes with the selected colors
                    for color in additional_colors {
                        // For harmony, we'll use balanced shapes that avoid existing ones
                        let shape = shape_generator.generate_shape_avoiding_cells(
                            color,
                            self.opacity,
                            size_range.1,
                            &used_cells,
                        );

                        // Update the used cells
                        for &cell in &shape.cells {
                            used_cells.insert(cell);
                        }

                        self.shapes.push(shape);
                    }
                }
            } else {
                // Use the improved algorithm without overlap

                // Generate shapes using intelligent color assignment
                let mut shapes = shape_generator.generate_shapes(
                    Vec::new(), // We'll assign colors after generation
                    self.opacity,
                    self.shapes_count as usize,
                    size_range,
                );

                // Assign harmonious colors to avoid same-colored neighbors
                color_manager.assign_harmonious_colors(grid, &mut shapes);

                self.shapes = shapes;
            }
        }

        Ok(())
    }

    /// Determine number of colors to use based on grid size and shape count
    fn palette_size(&self) -> usize {
        // We want at least as many colors as shapes
        // but also want enough colors to provide good variety
        let base_size = self.shapes_count as usize + 2; // Add 2 for some extra variety

        // For larger grids, we may want more colors for variety
        if self.grid_size >= 4 {
            base_size + 2
        } else {
            base_size
        }
    }

    pub fn grid(&self) -> Option<&TriangularGrid> {
        self.grid.as_ref()
    }

    pub fn shapes(&self) -> &[Shape] {
        &self.shapes
    }
}
