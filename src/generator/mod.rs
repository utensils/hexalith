mod color;
pub mod grid;
pub mod shape;

use crate::Result;
use color::ColorManager;
use grid::TriangularGrid;
use shape::{Shape, ShapeGenerator};

pub struct Generator {
    grid_size: u8,
    shapes_count: u8,
    opacity: f32,
    seed: Option<u64>,
    grid: Option<TriangularGrid>,
    shapes: Vec<Shape>,
    color_scheme: String,
}

impl Generator {
    pub fn new(grid_size: u8, shapes_count: u8, opacity: f32, seed: Option<u64>) -> Self {
        Self {
            grid_size: grid_size.clamp(3, 8),
            shapes_count: shapes_count.clamp(1, 10),
            opacity: opacity.clamp(0.0, 1.0),
            seed,
            grid: None,
            shapes: Vec::new(),
            color_scheme: "default".to_string(),
        }
    }

    pub fn set_color_scheme(&mut self, color_scheme: &str) -> &mut Self {
        self.color_scheme = color_scheme.to_string();
        self
    }

    pub fn generate(&mut self) -> Result<()> {
        // Initialize the triangular grid
        let grid = TriangularGrid::new(100.0, self.grid_size);
        self.grid = Some(grid);

        // Generate shapes
        if let Some(grid) = &self.grid {
            // Set up color manager
            let mut color_manager = ColorManager::default(self.seed);

            // Calculate shape size based on grid density
            // Higher density = smaller shapes
            let total_cells = grid.cell_count();
            let min_size = (total_cells as f32 * 0.01).round() as usize;
            let max_size = (total_cells as f32 * 0.05).round() as usize;
            let size_range = (min_size.max(3), max_size.max(min_size + 2));

            // Generate the shapes
            let mut shape_generator = ShapeGenerator::new(grid, self.seed);
            let colors = color_manager.get_random_colors(self.shapes_count as usize);

            self.shapes = shape_generator.generate_shapes(
                colors,
                self.opacity,
                self.shapes_count as usize,
                size_range,
            );
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
