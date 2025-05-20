mod color;
pub mod grid;
pub mod shape;

use crate::Result;
use color::ColorManager;
use grid::TriangularGrid;
use shape::{Shape, ShapeGenerator};

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
                // Generate overlapping shapes (like the original logo generator)
                
                // Get two colors plus a blend color for the overlap
                let (color1, color2, blend) = color_manager.get_colors_with_blend();
                
                // Generate two shapes with some overlap
                let shape1 = shape_generator.generate_angular_shape(
                    color1.clone(), 
                    self.opacity,
                    size_range.1, // Use larger size for better overlap chance
                );
                
                let shape2 = shape_generator.generate_angular_shape(
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
                
                // Add additional shapes if needed
                if self.shapes_count > 2 {
                    let more_colors = color_manager.get_random_colors((self.shapes_count - 2) as usize);
                    
                    for color in more_colors {
                        let shape = shape_generator.generate_angular_shape(
                            color,
                            self.opacity,
                            size_range.1,
                        );
                        
                        self.shapes.push(shape);
                    }
                }
            } else {
                // Use the standard algorithm without overlap
                let colors = color_manager.get_random_colors(self.shapes_count as usize);

                self.shapes = shape_generator.generate_shapes(
                    colors,
                    self.opacity,
                    self.shapes_count as usize,
                    size_range,
                );
            }
        }

        Ok(())
    }

    pub fn grid(&self) -> Option<&TriangularGrid> {
        self.grid.as_ref()
    }

    pub fn shapes(&self) -> &[Shape] {
        &self.shapes
    }
}
