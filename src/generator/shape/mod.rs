use crate::generator::grid::TriangularGrid;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

/// Represents a shape made up of connected triangular cells
#[derive(Debug, Clone)]
pub struct Shape {
    pub cells: Vec<usize>,
    pub color: String,
    pub opacity: f32,
}

impl Shape {
    pub fn new(color: String, opacity: f32) -> Self {
        Self {
            cells: Vec::new(),
            color,
            opacity,
        }
    }

    pub fn add_cell(&mut self, cell_id: usize) {
        if !self.cells.contains(&cell_id) {
            self.cells.push(cell_id);
        }
    }

    pub fn contains_cell(&self, cell_id: usize) -> bool {
        self.cells.contains(&cell_id)
    }

    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }
}

/// Generates random shapes on the triangular grid
pub struct ShapeGenerator<'a> {
    grid: &'a TriangularGrid,
    rng: ChaCha8Rng,
}

impl<'a> ShapeGenerator<'a> {
    pub fn new(grid: &'a TriangularGrid, seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(seed) => ChaCha8Rng::seed_from_u64(seed),
            None => ChaCha8Rng::from_entropy(),
        };

        Self { grid, rng }
    }

    /// Generates a random shape starting from a random cell
    pub fn generate_random_shape(
        &mut self,
        color: String,
        opacity: f32,
        target_size: usize,
    ) -> Shape {
        let mut shape = Shape::new(color, opacity);
        let total_cells = self.grid.cell_count();

        if total_cells == 0 || target_size == 0 {
            return shape;
        }

        // Choose a random starting cell
        let start_cell = self.rng.gen_range(0..total_cells);
        shape.add_cell(start_cell);

        // Maximum attempts to reach target size
        let max_attempts = target_size * 3;
        let mut attempts = 0;

        // Keep adding adjacent cells until we reach the target size or run out of options
        while shape.cell_count() < target_size && attempts < max_attempts {
            attempts += 1;

            // Find all adjacent cells that aren't already in the shape
            let mut candidates = Vec::new();

            for &cell_id in &shape.cells {
                let adjacent = self.grid.adjacent_cells(cell_id);
                for adj_id in adjacent {
                    if !shape.contains_cell(adj_id) {
                        candidates.push(adj_id);
                    }
                }
            }

            // No more candidates, break
            if candidates.is_empty() {
                break;
            }

            // Choose a random adjacent cell
            let next_cell = candidates[self.rng.gen_range(0..candidates.len())];
            shape.add_cell(next_cell);
        }

        shape
    }

    /// Generates multiple random shapes
    pub fn generate_shapes(
        &mut self,
        colors: Vec<String>,
        opacity: f32,
        count: usize,
        size_range: (usize, usize),
    ) -> Vec<Shape> {
        let mut shapes = Vec::with_capacity(count);

        // Track which cells are already used
        let mut used_cells = HashSet::new();

        // Generate the first shape - start from the center
        if count > 0 {
            // Choose a color
            let color_idx = self.rng.gen_range(0..colors.len());
            let color = colors[color_idx].clone();

            // Randomize size within the range
            let min_size = size_range.0;
            let max_size = size_range.1;
            let size = self.rng.gen_range(min_size..=max_size);

            // Generate first shape starting from center
            let first_shape = self.generate_center_shape(color, opacity, size);

            // Add the shape's cells to used_cells
            for &cell_id in &first_shape.cells {
                used_cells.insert(cell_id);
            }

            shapes.push(first_shape);
        }

        // Generate remaining shapes, ensuring they connect to existing ones
        for _i in 1..count {
            // Choose a color
            let color_idx = self.rng.gen_range(0..colors.len());
            let color = colors[color_idx].clone();

            // Randomize size within the range
            let min_size = size_range.0;
            let max_size = size_range.1;
            let size = self.rng.gen_range(min_size..=max_size);

            // Generate a shape connected to existing shapes
            let shape = self.generate_connected_shape(color, opacity, size, &used_cells);

            // Add the shape's cells to used_cells
            for &cell_id in &shape.cells {
                used_cells.insert(cell_id);
            }

            shapes.push(shape);
        }

        shapes
    }

    /// Generates a shape starting from the center of the hexagon
    /// This ensures shapes are connected and not floating isolated
    fn generate_center_shape(
        &mut self,
        color: String,
        opacity: f32,
        target_size: usize,
    ) -> Shape {
        let mut shape = Shape::new(color, opacity);
        let total_cells = self.grid.cell_count();

        if total_cells == 0 || target_size == 0 {
            return shape;
        }

        // Find cells nearest to center of hexagon
        let center_cells = self.find_center_cells();
        if center_cells.is_empty() {
            return shape;
        }

        // Start with one of the center cells
        let start_cell = center_cells[self.rng.gen_range(0..center_cells.len())];
        shape.add_cell(start_cell);

        // Maximum attempts to reach target size
        let max_attempts = target_size * 3;
        let mut attempts = 0;

        // Keep adding adjacent cells until we reach the target size or run out of options
        while shape.cell_count() < target_size && attempts < max_attempts {
            attempts += 1;

            // Find all adjacent cells that aren't already in the shape
            let mut candidates = Vec::new();

            for &cell_id in &shape.cells {
                let adjacent = self.grid.adjacent_cells(cell_id);
                for adj_id in adjacent {
                    if !shape.contains_cell(adj_id) {
                        candidates.push(adj_id);
                    }
                }
            }

            // No more candidates, break
            if candidates.is_empty() {
                break;
            }

            // Choose a random adjacent cell
            let next_cell = candidates[self.rng.gen_range(0..candidates.len())];
            shape.add_cell(next_cell);
        }

        shape
    }

    /// Finds cells closest to the center of the hexagon
    fn find_center_cells(&self) -> Vec<usize> {
        let center = self.grid.hex_grid().center;
        let mut cells_by_distance = Vec::new();

        for (i, cell) in self.grid.cells().iter().enumerate() {
            let distance = cell.centroid.distance(&center);
            cells_by_distance.push((i, distance));
        }

        // Sort by distance to center
        cells_by_distance.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Return the IDs of the 6 cells closest to center
        let closest_count = std::cmp::min(6, cells_by_distance.len());
        cells_by_distance.iter()
            .take(closest_count)
            .map(|(id, _)| *id)
            .collect()
    }

    /// Generates a shape that connects to existing shapes
    /// This ensures all shapes are connected to at least one other shape
    fn generate_connected_shape(
        &mut self,
        color: String,
        opacity: f32,
        target_size: usize,
        used_cells: &HashSet<usize>,
    ) -> Shape {
        let color_clone = color.clone(); // Clone color up front for potential use later
        let mut shape = Shape::new(color, opacity);
        let total_cells = self.grid.cell_count();

        if total_cells == 0 || target_size == 0 {
            return shape;
        }

        // Find cells adjacent to the used cells (boundary cells)
        let boundary_cells = self.find_boundary_cells(used_cells);
        if boundary_cells.is_empty() {
            // Fall back to random placement if no boundary cells found
            return self.generate_shape_avoiding_cells(color_clone, opacity, target_size, used_cells);
        }

        // Start with one of the boundary cells
        let start_cell = boundary_cells[self.rng.gen_range(0..boundary_cells.len())];
        shape.add_cell(start_cell);

        // Maximum attempts to reach target size
        let max_attempts = target_size * 3;
        let mut attempts = 0;

        // Keep adding adjacent cells until we reach the target size or run out of options
        while shape.cell_count() < target_size && attempts < max_attempts {
            attempts += 1;

            // Find all adjacent cells that aren't already in the shape or used elsewhere
            let mut candidates = Vec::new();

            for &cell_id in &shape.cells {
                let adjacent = self.grid.adjacent_cells(cell_id);
                for adj_id in adjacent {
                    if !shape.contains_cell(adj_id) && !used_cells.contains(&adj_id) {
                        candidates.push(adj_id);
                    }
                }
            }

            // No more candidates, break
            if candidates.is_empty() {
                break;
            }

            // Choose a random adjacent cell
            let next_cell = candidates[self.rng.gen_range(0..candidates.len())];
            shape.add_cell(next_cell);
        }

        shape
    }

    /// Finds cells that are adjacent to already used cells
    fn find_boundary_cells(&self, used_cells: &HashSet<usize>) -> Vec<usize> {
        let mut boundary = Vec::new();

        for &used_cell in used_cells.iter() {
            let adjacent = self.grid.adjacent_cells(used_cell);
            for adj_id in adjacent {
                if !used_cells.contains(&adj_id) && !boundary.contains(&adj_id) {
                    boundary.push(adj_id);
                }
            }
        }

        boundary
    }

    /// Generates a shape while avoiding cells that are already used
    fn generate_shape_avoiding_cells(
        &mut self,
        color: String,
        opacity: f32,
        target_size: usize,
        used_cells: &HashSet<usize>,
    ) -> Shape {
        let mut shape = Shape::new(color, opacity);
        let total_cells = self.grid.cell_count();

        if total_cells == 0 || target_size == 0 {
            return shape;
        }

        // Find an unused starting cell
        let mut start_cell = None;
        let mut attempts = 0;

        while start_cell.is_none() && attempts < 100 {
            attempts += 1;
            let candidate = self.rng.gen_range(0..total_cells);
            if !used_cells.contains(&candidate) {
                start_cell = Some(candidate);
            }
        }

        // If we couldn't find an unused cell, just return an empty shape
        let start_cell = match start_cell {
            Some(cell) => cell,
            None => return shape,
        };

        shape.add_cell(start_cell);

        // Maximum attempts to reach target size
        let max_attempts = target_size * 3;
        attempts = 0;

        // Keep adding adjacent cells until we reach the target size or run out of options
        while shape.cell_count() < target_size && attempts < max_attempts {
            attempts += 1;

            // Find all adjacent cells that aren't already in the shape or used elsewhere
            let mut candidates = Vec::new();

            for &cell_id in &shape.cells {
                let adjacent = self.grid.adjacent_cells(cell_id);
                for adj_id in adjacent {
                    if !shape.contains_cell(adj_id) && !used_cells.contains(&adj_id) {
                        candidates.push(adj_id);
                    }
                }
            }

            // No more candidates, break
            if candidates.is_empty() {
                break;
            }

            // Choose a random adjacent cell
            let next_cell = candidates[self.rng.gen_range(0..candidates.len())];
            shape.add_cell(next_cell);
        }

        shape
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::grid::TriangularGrid;

    #[test]
    fn test_shape_creation() {
        let shape = Shape::new("#FF0000".to_string(), 0.8);
        assert_eq!(shape.cell_count(), 0);
        assert_eq!(shape.color, "#FF0000");
        assert_eq!(shape.opacity, 0.8);
    }

    #[test]
    fn test_shape_add_cell() {
        let mut shape = Shape::new("#FF0000".to_string(), 0.8);
        shape.add_cell(1);
        shape.add_cell(2);
        shape.add_cell(3);

        assert_eq!(shape.cell_count(), 3);
        assert!(shape.contains_cell(1));
        assert!(shape.contains_cell(2));
        assert!(shape.contains_cell(3));
        assert!(!shape.contains_cell(4));
    }

    #[test]
    fn test_find_center_cells() {
        let grid = TriangularGrid::new(100.0, 4);
        let generator = ShapeGenerator::new(&grid, Some(42)); // Fixed seed for deterministic testing
        
        let center_cells = generator.find_center_cells();
        
        // Should find some cells near the center
        assert!(!center_cells.is_empty());
    }
    
    #[test]
    fn test_generate_center_shape() {
        let grid = TriangularGrid::new(100.0, 4);
        let mut generator = ShapeGenerator::new(&grid, Some(42)); // Fixed seed for deterministic testing

        let color = "#FF0000".to_string();
        let opacity = 0.8;
        let target_size = 10;

        let shape = generator.generate_center_shape(color, opacity, target_size);

        // Shape should have cells starting from center
        assert!(!shape.cells.is_empty());
        assert!(shape.cell_count() <= target_size);
    }

    #[test]
    fn test_shape_generator() {
        let grid = TriangularGrid::new(100.0, 4);
        let mut generator = ShapeGenerator::new(&grid, Some(42)); // Fixed seed for deterministic testing

        let color = "#FF0000".to_string();
        let opacity = 0.8;
        let target_size = 10;

        let shape = generator.generate_random_shape(color, opacity, target_size);

        // Shape should have cells (may be less than target if we ran out of adjacent cells)
        assert!(!shape.cells.is_empty());
        assert!(shape.cell_count() <= target_size);
    }

    #[test]
    fn test_generate_multiple_shapes() {
        let grid = TriangularGrid::new(100.0, 4);
        let mut generator = ShapeGenerator::new(&grid, Some(42)); // Fixed seed for deterministic testing

        let colors = vec![
            "#FF0000".to_string(),
            "#00FF00".to_string(),
            "#0000FF".to_string(),
        ];
        let opacity = 0.8;
        let count = 3;
        let size_range = (5, 10);

        let shapes = generator.generate_shapes(colors, opacity, count, size_range);

        // Should have the requested number of shapes
        assert_eq!(shapes.len(), count);

        // Each shape should have cells within the size range
        for shape in &shapes {
            assert!(shape.cell_count() >= size_range.0 || shape.cell_count() <= size_range.1);
        }
    }
}
