use crate::generator::grid::TriangularGrid;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::{HashSet, VecDeque};

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

/// Shape evaluation metrics for balanced shapes
#[derive(Debug, Clone, Copy)]
pub struct ShapeMetrics {
    pub compactness: f64,  // Higher is better (more compact)
    pub smoothness: f64,   // Higher is better (smoother perimeter)
    pub balance: f64,      // Higher is better (more balanced from center)
}

/// Generates random shapes on the triangular grid
pub struct ShapeGenerator<'a> {
    grid: &'a TriangularGrid,
    rng: ChaCha8Rng,
}

impl<'a> ShapeGenerator<'a> {
    pub fn new(grid: &'a TriangularGrid, seed: Option<u64>) -> Self {
        // Add extra randomness by combining seed with timestamp nanoseconds
        let rng = match seed {
            Some(seed) => {
                // Get the current timestamp's nanoseconds
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .subsec_nanos();
                
                // Combine seed and timestamp for additional randomness
                // But only use a portion of the nanoseconds to keep some seed determinism
                let combined_seed = seed.wrapping_add((now % 10000) as u64);
                ChaCha8Rng::seed_from_u64(combined_seed)
            },
            None => ChaCha8Rng::from_entropy(),
        };

        Self { grid, rng }
    }

    /// Generates a more angular shape with equiangular triangles and connecting edges
    /// that grows from the center outward, but with improved balance
    pub fn generate_angular_shape(
        &mut self,
        color: String,
        opacity: f32,
        target_size: usize,
    ) -> Shape {
        // Generate multiple candidate shapes and select the best one
        let candidates = 3;
        let mut shapes = Vec::with_capacity(candidates);
        
        for _ in 0..candidates {
            shapes.push(self.generate_angular_shape_candidate(color.clone(), opacity, target_size));
        }
        
        // Sort shapes by quality metric
        shapes.sort_by(|a, b| {
            let score_a = self.evaluate_shape_quality(a);
            let score_b = self.evaluate_shape_quality(b);
            
            // Higher is better, but add randomness to avoid always picking the same shape
            let random_factor = self.rng.gen_range(-0.1..0.1);
            (score_b.total_score() + random_factor)
                .partial_cmp(&score_a.total_score())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Return the best shape
        shapes.into_iter().next().unwrap_or_else(|| Shape::new(color, opacity))
    }
    
    /// Internal function to generate a candidate shape with angular properties
    fn generate_angular_shape_candidate(
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
        
        // Choose a starting cell
        // We now have a small chance to not start exactly at the center
        let center_cells = self.find_center_cells();
        let start_cell_idx = if self.rng.gen::<f32>() < 0.7 {
            // 70% chance to start from the very center
            0
        } else {
            // 30% chance to start from one of the next closest cells to center
            self.rng.gen_range(0..center_cells.len().min(3))
        };
        
        let start_cell = center_cells[start_cell_idx];
        shape.add_cell(start_cell);
        
        // Maximum attempts to reach target size
        let max_attempts = target_size * 3;
        let mut attempts = 0;
        
        // Use a modified breadth-first growth approach that creates balanced, angular patterns
        // while maintaining connectivity and growing from center out
        let mut current_layer = vec![start_cell];
        let mut next_layer = Vec::new();
        
        // Keep a frontier of cells to consider in the current layer
        let mut frontier = Vec::new();
        
        // Track boundary cells (cells with at least one non-filled adjacent cell)
        let mut boundary = HashSet::new();
        boundary.insert(start_cell);
        
        // Randomness factor for this particular shape
        // This controls how much randomness vs. balance we want
        let randomness = self.rng.gen_range(0.2..0.5); 
        
        // Keep adding cells until we reach the target size
        while shape.cell_count() < target_size && attempts < max_attempts {
            attempts += 1;
            
            // If frontier is empty, refill it from current layer
            if frontier.is_empty() {
                if current_layer.is_empty() {
                    // Move to next layer if current is empty
                    current_layer = next_layer;
                    next_layer = Vec::new();
                    
                    // If both layers are empty, we can't grow any further
                    if current_layer.is_empty() {
                        break;
                    }
                }
                
                // Get a cell from current layer and find its adjacent cells
                let cell = current_layer.remove(0);
                
                // Find all adjacent cells that aren't already in the shape
                for adj_id in self.grid.adjacent_cells(cell) {
                    if !shape.contains_cell(adj_id) && !frontier.contains(&adj_id) {
                        frontier.push(adj_id);
                    }
                }
                
                // If no adjacent cells available, continue to next cell in layer
                if frontier.is_empty() {
                    continue;
                }
            }
            
            // Sort the frontier by a balanced scoring heuristic
            frontier.sort_by(|&a, &b| {
                let score_a = self.score_candidate_cell(&shape, a);
                let score_b = self.score_candidate_cell(&shape, b);
                // Compare scores (higher is better)
                score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
            });
            
            // Introduce more randomness in selection
            let selected_idx = if self.rng.gen::<f32>() < randomness {
                // Sometimes pick completely randomly for variety
                self.rng.gen_range(0..frontier.len())
            } else {
                // Choose from top candidates but with some randomness 
                let top_candidates = (frontier.len() / 2).max(1).min(frontier.len());
                self.rng.gen_range(0..top_candidates)
            };
            
            // Choose a cell from the frontier
            let next_cell = frontier.remove(selected_idx);
            
            // Add the cell to the shape
            shape.add_cell(next_cell);
            
            // Add it to the next layer for future expansion
            next_layer.push(next_cell);
            
            // Update boundary cells
            boundary.remove(&next_cell);
            
            // Check if this cell has any adjacent cells not in the shape
            let adjacent_cells = self.grid.adjacent_cells(next_cell);
            let mut has_non_filled_adjacent = false;
            
            for &adj in &adjacent_cells {
                if !shape.contains_cell(adj) {
                    has_non_filled_adjacent = true;
                    if !frontier.contains(&adj) {
                        frontier.push(adj);
                    }
                }
            }
            
            if has_non_filled_adjacent {
                boundary.insert(next_cell);
            }
            
            // For angular shapes, periodically skip cells to create more angles
            // but with more controlled selection based on shape quality
            if self.rng.gen::<f32>() < (0.1 + randomness) && frontier.len() > 2 {
                // Remove a cell that would create the least balanced addition
                frontier.sort_by(|&a, &b| {
                    let score_a = self.score_candidate_cell(&shape, a);
                    let score_b = self.score_candidate_cell(&shape, b);
                    // Compare scores (lower is worse)
                    score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
                });
                
                // Remove the worst candidate
                frontier.remove(0);
            }
        }
        
        // Only smooth the shape if it's not very angular (controlled by randomness)
        if self.rng.gen::<f32>() > randomness {
            self.smooth_shape(&mut shape, target_size);
        }
        
        shape
    }
    
    /// Score a candidate cell for addition to a shape
    /// Higher scores indicate better candidates for balanced shapes
    fn score_candidate_cell(&self, shape: &Shape, cell_id: usize) -> f64 {
        if shape.cells.is_empty() {
            return 1.0; // All cells are equally good for empty shapes
        }
        
        let mut score = 0.0;
        
        // Get the cell
        if let Some(cell) = self.grid.get_cell(cell_id) {
            // Get the shape center (average of all cell centroids)
            let mut center_x = 0.0;
            let mut center_y = 0.0;
            
            for &id in &shape.cells {
                if let Some(existing_cell) = self.grid.get_cell(id) {
                    center_x += existing_cell.centroid.x;
                    center_y += existing_cell.centroid.y;
                }
            }
            
            center_x /= shape.cells.len() as f64;
            center_y /= shape.cells.len() as f64;
            
            // Compute factors that influence score
            
            // 1. Adjacency factor: more adjacent cells in the shape is better
            let adjacent_cells = self.grid.adjacent_cells(cell_id);
            let mut adjacent_in_shape = 0;
            
            for &adj in &adjacent_cells {
                if shape.contains_cell(adj) {
                    adjacent_in_shape += 1;
                }
            }
            
            // Scale adjacency score (2 is ideal - creates smoother boundaries)
            let adjacency_score = if adjacent_in_shape == 0 {
                0.0 // Must be connected
            } else if adjacent_in_shape == 1 {
                0.7 // Connected but could create jaggy shape
            } else if adjacent_in_shape == 2 {
                1.0 // Ideal - creates smoother boundary
            } else {
                0.6 // More than 2 - filling in concave areas
            };
            
            // 2. Distance from center factor
            // We want cells that maintain a somewhat circular growth
            // with center x and y, not too far or too close
            let dx = cell.centroid.x - center_x;
            let dy = cell.centroid.y - center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            // Calculate expected radius for a circular shape of current size
            let expected_radius = (shape.cells.len() as f64).sqrt() * 1.2;
            
            // Penalize cells that are significantly closer or further 
            // from expected radius - aim for balanced growth
            let dist_ratio = distance / expected_radius;
            let distance_score = 1.0 - (dist_ratio - 1.0).abs().min(1.0);
            
            // 3. Balance factor: prefer cells that maintain overall shape balance
            // This checks if adding this cell would move the shape center
            // significantly or keep it balanced
            let new_center_x = (center_x * shape.cells.len() as f64 + cell.centroid.x) / 
                              (shape.cells.len() + 1) as f64;
            let new_center_y = (center_y * shape.cells.len() as f64 + cell.centroid.y) / 
                              (shape.cells.len() + 1) as f64;
            
            let center_shift = ((new_center_x - center_x).powi(2) + 
                               (new_center_y - center_y).powi(2)).sqrt();
            
            // Normalize by the expected radius
            let balance_score = 1.0 - (center_shift / expected_radius).min(1.0);
            
            // Combine scores with appropriate weights
            score = adjacency_score * 0.4 + distance_score * 0.4 + balance_score * 0.2;
        }
        
        score
    }
    
    /// Apply smoothing to fill in sharp concave areas
    fn smooth_shape(&mut self, shape: &mut Shape, target_size: usize) {
        // If shape is too small, don't smooth
        if shape.cell_count() < 3 || shape.cell_count() >= target_size {
            return;
        }
        
        // Find concave regions that could be filled
        let mut candidates = Vec::new();
        
        // Find all boundary cells (cells with at least one adjacent cell not in shape)
        let mut boundary_cells = Vec::new();
        
        for &cell_id in &shape.cells {
            let adjacent = self.grid.adjacent_cells(cell_id);
            for &adj in &adjacent {
                if !shape.contains_cell(adj) {
                    boundary_cells.push(cell_id);
                    break;
                }
            }
        }
        
        // For each boundary cell, check if it has adjacent cells also on the boundary
        for &cell_id in &boundary_cells {
            let adjacent = self.grid.adjacent_cells(cell_id);
            
            // Count adjacent cells that are also on the boundary
            let mut adj_boundary = 0;
            for &adj in &adjacent {
                if shape.contains_cell(adj) && boundary_cells.contains(&adj) {
                    adj_boundary += 1;
                }
            }
            
            // Find potential cells to add
            if adj_boundary >= 2 {
                // This cell has at least 2 neighbors on the boundary
                // Find external cells adjacent to both this and its boundary neighbors
                let mut external_cells = HashSet::new();
                
                for &adj in &adjacent {
                    if shape.contains_cell(adj) && boundary_cells.contains(&adj) {
                        // This is a boundary neighbor
                        let adj_external: Vec<usize> = self.grid.adjacent_cells(adj)
                            .into_iter()
                            .filter(|&id| !shape.contains_cell(id))
                            .collect();
                        
                        for &ext in &adj_external {
                            external_cells.insert(ext);
                        }
                    }
                }
                
                // Check if any external cell is adjacent to 2+ boundary cells
                // This would fill in a concave area
                for &ext in &external_cells {
                    let ext_adjacent = self.grid.adjacent_cells(ext);
                    let mut connected_boundary = 0;
                    
                    for &ext_adj in &ext_adjacent {
                        if shape.contains_cell(ext_adj) && boundary_cells.contains(&ext_adj) {
                            connected_boundary += 1;
                        }
                    }
                    
                    if connected_boundary >= 2 {
                        // This external cell would fill in a concave region
                        candidates.push(ext);
                    }
                }
            }
        }
        
        // Add smoothing cells up to target size
        candidates.sort_by(|&a, &b| {
            let score_a = self.score_candidate_cell(shape, a);
            let score_b = self.score_candidate_cell(shape, b);
            // Higher score is better
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Add randomness - maybe don't fill all concave areas
        let fill_count = if candidates.is_empty() {
            0
        } else {
            self.rng.gen_range(0..=candidates.len())
        };
        
        for (i, &cell_id) in candidates.iter().enumerate() {
            if i < fill_count && shape.cell_count() < target_size && !shape.contains_cell(cell_id) {
                shape.add_cell(cell_id);
            } else {
                break;
            }
        }
    }
    
    /// Evaluate the overall quality of a shape based on multiple metrics
    pub fn evaluate_shape_quality(&self, shape: &Shape) -> ShapeMetrics {
        if shape.cells.is_empty() {
            return ShapeMetrics {
                compactness: 0.0,
                smoothness: 0.0,
                balance: 0.0,
            };
        }
        
        // 1. Calculate the shape's center
        let mut center_x = 0.0;
        let mut center_y = 0.0;
        
        for &id in &shape.cells {
            if let Some(cell) = self.grid.get_cell(id) {
                center_x += cell.centroid.x;
                center_y += cell.centroid.y;
            }
        }
        
        center_x /= shape.cells.len() as f64;
        center_y /= shape.cells.len() as f64;
        
        // 2. Calculate compactness (ratio of perimeter to area)
        let mut boundary_edges = 0;
        let area = shape.cells.len() as f64;
        
        for &cell_id in &shape.cells {
            let adjacent = self.grid.adjacent_cells(cell_id);
            for &adj in &adjacent {
                if !shape.contains_cell(adj) {
                    boundary_edges += 1;
                }
            }
        }
        
        // Normalize compactness (lower is more compact, so we invert it)
        // Perfect circle would have perimeter² / area = 4π, so we use that as reference
        let perimeter = boundary_edges as f64;
        let compactness = if perimeter > 0.0 {
            // Normalize to 0-1 range, higher is more compact
            let ideal_ratio = (12.56 * area).sqrt(); // 4π * area
            1.0 - ((perimeter - ideal_ratio) / perimeter).min(1.0).max(0.0)
        } else {
            0.0
        };
        
        // 3. Calculate smoothness (absence of sharp angles in the boundary)
        let mut smoothness = 1.0;
        
        // Find boundary cells
        let mut boundary_cells = Vec::new();
        for &cell_id in &shape.cells {
            let adjacent = self.grid.adjacent_cells(cell_id);
            for &adj in &adjacent {
                if !shape.contains_cell(adj) {
                    boundary_cells.push(cell_id);
                    break;
                }
            }
        }
        
        // Calculate how many boundary cells have multiple adjacent boundary cells
        // More such cells means more potential for sharp angles
        if !boundary_cells.is_empty() {
            let mut sharp_angles = 0;
            
            for &cell_id in &boundary_cells {
                let adjacent = self.grid.adjacent_cells(cell_id);
                let mut adj_boundary = 0;
                
                for &adj in &adjacent {
                    if shape.contains_cell(adj) && boundary_cells.contains(&adj) {
                        adj_boundary += 1;
                    }
                }
                
                // More than 2 adjacent boundary cells could indicate sharp angle
                if adj_boundary > 2 {
                    sharp_angles += 1;
                }
            }
            
            // Calculate smoothness ratio (0-1, higher is smoother)
            smoothness = 1.0 - (sharp_angles as f64 / boundary_cells.len() as f64).min(1.0);
        }
        
        // 4. Calculate balance (how evenly distributed cells are around center)
        let mut max_dist: f64 = 0.0;
        let mut avg_dist = 0.0;
        let mut variance = 0.0;
        
        for &id in &shape.cells {
            if let Some(cell) = self.grid.get_cell(id) {
                let dx = cell.centroid.x - center_x;
                let dy = cell.centroid.y - center_y;
                let dist = (dx * dx + dy * dy).sqrt();
                
                avg_dist += dist;
                max_dist = max_dist.max(dist);
            }
        }
        
        avg_dist /= shape.cells.len() as f64;
        
        // Calculate variance of distances
        for &id in &shape.cells {
            if let Some(cell) = self.grid.get_cell(id) {
                let dx = cell.centroid.x - center_x;
                let dy = cell.centroid.y - center_y;
                let dist = (dx * dx + dy * dy).sqrt();
                
                variance += (dist - avg_dist).powi(2);
            }
        }
        
        variance /= shape.cells.len() as f64;
        
        // Lower variance means more uniform distribution around center
        let balance = 1.0 - (variance / max_dist.powi(2)).min(1.0);
        
        ShapeMetrics {
            compactness,
            smoothness,
            balance,
        }
    }

    /// Generates a shape with connected edges that grows from the center outward
    /// This replaces the previous random shape generation to ensure all shapes grow from center
    pub fn generate_random_shape(
        &mut self,
        color: String,
        opacity: f32,
        target_size: usize,
    ) -> Shape {
        // Now we have a chance to do either a center shape or angular shape
        if self.rng.gen::<f32>() < 0.5 {
            return self.generate_center_shape(color, opacity, target_size);
        } else {
            return self.generate_angular_shape(color, opacity, target_size);
        }
    }

    /// Generates multiple shapes that grow from the center out with connecting edges
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

        // Generate the first shape - always start from the center
        if count > 0 {
            // Randomize size within the range
            let min_size = size_range.0;
            let max_size = size_range.1;
            let size = self.rng.gen_range(min_size..=max_size);

            // Generate first shape - variety for first shape type
            let first_shape = if self.rng.gen::<f32>() < 0.5 {
                self.generate_balanced_shape(
                    if colors.is_empty() { String::from("#FF0000") } else { colors[0].clone() },
                    opacity, 
                    size
                )
            } else {
                self.generate_angular_shape(
                    if colors.is_empty() { String::from("#FF0000") } else { colors[0].clone() },
                    opacity, 
                    size
                )
            };

            // Add the shape's cells to used_cells
            for &cell_id in &first_shape.cells {
                used_cells.insert(cell_id);
            }

            shapes.push(first_shape);
        }

        // Generate remaining shapes, ensuring they connect to existing ones and grow outward
        for i in 1..count {
            // Use a color from the provided list if available
            let color = if !colors.is_empty() && i < colors.len() {
                colors[i].clone()
            } else {
                // Use a placeholder color if no colors were provided (will be replaced later)
                format!("#PLACEHOLDER{}", i)
            };

            // Randomize size within the range
            let min_size = size_range.0;
            let max_size = size_range.1;
            let size = self.rng.gen_range(min_size..=max_size);

            // Generate a shape that connects to existing shapes or is avoiding them
            // Add more variety in shape types
            let shape = if self.rng.gen::<f32>() < 0.3 {
                // Sometimes create shapes that avoid existing ones
                self.generate_shape_avoiding_cells(color, opacity, size, &used_cells)
            } else {
                // Usually create shapes that connect to existing ones
                self.generate_connected_shape(color, opacity, size, &used_cells)
            };

            // Add the shape's cells to used_cells
            for &cell_id in &shape.cells {
                used_cells.insert(cell_id);
            }

            shapes.push(shape);
        }

        shapes
    }
    
    /// Generate a balanced, aesthetically pleasing shape
    pub fn generate_balanced_shape(
        &mut self,
        color: String,
        opacity: f32,
        target_size: usize,
    ) -> Shape {
        // Generate multiple candidates and select the best one
        let candidates = 3;
        let mut shapes = Vec::with_capacity(candidates);
        
        for _ in 0..candidates {
            shapes.push(self.generate_center_shape(color.clone(), opacity, target_size));
        }
        
        // Sort shapes by quality metrics
        shapes.sort_by(|a, b| {
            let metrics_a = self.evaluate_shape_quality(a);
            let metrics_b = self.evaluate_shape_quality(b);
            
            let score_a = metrics_a.total_score();
            let score_b = metrics_b.total_score();
            
            // Higher score is better, but add randomness to avoid always picking the same shape
            let random_factor = self.rng.gen_range(-0.1..0.1);
            (score_b + random_factor)
                .partial_cmp(&score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Return the best shape
        shapes.into_iter().next().unwrap_or_else(|| Shape::new(color, opacity))
    }

    /// Generates a shape starting from the center of the hexagon and growing outward
    /// This ensures shapes are connected, not floating isolated, and grow from the center out
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

        // Start with a cell near center, but not always the exact center
        let start_idx = if self.rng.gen::<f32>() < 0.8 {
            0  // 80% chance to use the center
        } else {
            self.rng.gen_range(0..center_cells.len().min(3))
        };
        
        let start_cell = center_cells[start_idx];
        shape.add_cell(start_cell);

        // Maximum attempts to reach target size
        let max_attempts = target_size * 3;
        let mut attempts = 0;
        
        // Randomness factor for this shape
        let randomness = self.rng.gen_range(0.1..0.4);
        
        // Use a breadth-first approach for uniform growth from center
        let mut queue = VecDeque::new();
        queue.push_back(start_cell);
        
        let mut visited = HashSet::new();
        visited.insert(start_cell);
        
        while shape.cell_count() < target_size && attempts < max_attempts && !queue.is_empty() {
            attempts += 1;
            
            let current_cell = queue.pop_front().unwrap();
            
            // Find candidates among adjacent cells
            let mut candidates = Vec::new();
            for &adj_id in &self.grid.adjacent_cells(current_cell) {
                if !shape.contains_cell(adj_id) && !visited.contains(&adj_id) {
                    candidates.push(adj_id);
                    visited.insert(adj_id);
                }
            }
            
            // Sometimes shuffle candidates for more variety
            if self.rng.gen::<f32>() < randomness {
                candidates.shuffle(&mut self.rng);
            } else {
                // Otherwise sort candidates by quality heuristic
                candidates.sort_by(|&a, &b| {
                    let score_a = self.score_candidate_cell(&shape, a);
                    let score_b = self.score_candidate_cell(&shape, b);
                    // Higher score is better
                    score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            
            // Add candidates to shape
            for candidate in candidates {
                if shape.cell_count() < target_size {
                    shape.add_cell(candidate);
                    queue.push_back(candidate);
                } else {
                    break;
                }
            }
        }
        
        // Apply smoothing (but not always)
        if self.rng.gen::<f32>() > randomness {
            self.smooth_shape(&mut shape, target_size);
        }
        
        shape
    }

    /// Finds cells closest to the center of the hexagon, sorted by distance
    fn find_center_cells(&self) -> Vec<usize> {
        let center = self.grid.hex_grid().center;
        let mut cells_by_distance = Vec::new();

        for (i, cell) in self.grid.cells().iter().enumerate() {
            let distance = cell.centroid.distance(&center);
            cells_by_distance.push((i, distance));
        }

        // Sort by distance to center (closest first)
        cells_by_distance.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Return all cell IDs sorted by distance from center
        // This is critical for growing from center outward in a structured way
        cells_by_distance.iter()
            .map(|(id, _)| *id)
            .collect()
    }

    /// Generates a shape that connects to existing shapes and grows outward
    /// This ensures all shapes are connected to at least one other shape
    /// and maintain the pattern of growing from center outward
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
        let mut boundary_cells = self.find_boundary_cells(used_cells);
        if boundary_cells.is_empty() {
            // Fall back to random placement if no boundary cells found
            return self.generate_shape_avoiding_cells(color_clone, opacity, target_size, used_cells);
        }

        // Add randomness to choice of starting cell
        boundary_cells.shuffle(&mut self.rng);
        
        // Sort by distance to center but with randomness
        if self.rng.gen::<f32>() < 0.7 {
            let center_cells = self.find_center_cells();
            boundary_cells.sort_by(|&a, &b| {
                // Find position in center_cells (lower index = closer to center)
                let pos_a = center_cells.iter().position(|&x| x == a).unwrap_or(usize::MAX);
                let pos_b = center_cells.iter().position(|&x| x == b).unwrap_or(usize::MAX);
                pos_a.cmp(&pos_b)
            });
        }

        // Start with the selected boundary cell
        let start_cell_idx = self.rng.gen_range(0..boundary_cells.len().min(3));
        let start_cell = boundary_cells[start_cell_idx];
        shape.add_cell(start_cell);

        // Maximum attempts to reach target size
        let max_attempts = target_size * 3;
        let mut attempts = 0;
        
        // Randomness factor
        let randomness = self.rng.gen_range(0.1..0.4);
        
        // Use a breadth-first approach for balanced growth
        let mut queue = VecDeque::new();
        queue.push_back(start_cell);
        
        let mut visited = HashSet::new();
        visited.insert(start_cell);
        
        while shape.cell_count() < target_size && attempts < max_attempts && !queue.is_empty() {
            attempts += 1;
            
            let current_cell = queue.pop_front().unwrap();
            
            // Find adjacent cells not in the shape and not already used
            let mut candidates = Vec::new();
            for &adj_id in &self.grid.adjacent_cells(current_cell) {
                if !shape.contains_cell(adj_id) && !used_cells.contains(&adj_id) && !visited.contains(&adj_id) {
                    candidates.push(adj_id);
                    visited.insert(adj_id);
                }
            }
            
            // Sometimes shuffle for true randomness
            if self.rng.gen::<f32>() < randomness {
                candidates.shuffle(&mut self.rng);
            } else {
                // Sort candidates by quality
                candidates.sort_by(|&a, &b| {
                    let score_a = self.score_candidate_cell(&shape, a);
                    let score_b = self.score_candidate_cell(&shape, b);
                    // Higher score is better
                    score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            
            // Add candidates that improve shape quality
            for candidate in candidates {
                if shape.cell_count() < target_size {
                    shape.add_cell(candidate);
                    queue.push_back(candidate);
                } else {
                    break;
                }
            }
        }
        
        // Apply smoothing (but not always)
        if self.rng.gen::<f32>() > randomness {
            self.smooth_shape(&mut shape, target_size);
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

    /// Generates a shape with connected edges that grows from center outward while avoiding used cells
    pub fn generate_shape_avoiding_cells(
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

        // Get all cells sorted by distance from center
        let center_cells = self.find_center_cells();
        
        // Find the first unused cell that is closest to the center
        let mut start_cell = None;
        for &cell_id in &center_cells {
            if !used_cells.contains(&cell_id) {
                start_cell = Some(cell_id);
                break;
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
        let mut attempts = 0;
        
        // Randomness factor
        let randomness = self.rng.gen_range(0.1..0.4);
        
        // Use a breadth-first approach for balanced growth
        let mut queue = VecDeque::new();
        queue.push_back(start_cell);
        
        let mut visited = HashSet::new();
        visited.insert(start_cell);
        
        while shape.cell_count() < target_size && attempts < max_attempts && !queue.is_empty() {
            attempts += 1;
            
            let current_cell = queue.pop_front().unwrap();
            
            // Find adjacent cells not in the shape and not already used
            let mut candidates = Vec::new();
            for &adj_id in &self.grid.adjacent_cells(current_cell) {
                if !shape.contains_cell(adj_id) && !used_cells.contains(&adj_id) && !visited.contains(&adj_id) {
                    candidates.push(adj_id);
                    visited.insert(adj_id);
                }
            }
            
            // Sometimes shuffle for more randomness
            if self.rng.gen::<f32>() < randomness {
                candidates.shuffle(&mut self.rng);
            } else {
                // Sort candidates by quality
                candidates.sort_by(|&a, &b| {
                    let score_a = self.score_candidate_cell(&shape, a);
                    let score_b = self.score_candidate_cell(&shape, b);
                    // Higher score is better
                    score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            
            // Add candidates that improve shape quality
            for candidate in candidates {
                if shape.cell_count() < target_size {
                    shape.add_cell(candidate);
                    queue.push_back(candidate);
                } else {
                    break;
                }
            }
        }
        
        // Apply smoothing (but not always)
        if self.rng.gen::<f32>() > randomness {
            self.smooth_shape(&mut shape, target_size);
        }
        
        shape
    }
}

// Extension to ShapeMetrics to calculate the total score
impl ShapeMetrics {
    pub fn total_score(&self) -> f64 {
        // Weight the different metrics based on their importance
        // Higher score means more aesthetically pleasing shape
        self.compactness * 0.4 + self.smoothness * 0.4 + self.balance * 0.2
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
    
    #[test]
    fn test_evaluate_shape_quality() {
        let grid = TriangularGrid::new(100.0, 4);
        let mut generator = ShapeGenerator::new(&grid, Some(42)); // Fixed seed for deterministic testing

        let color = "#FF0000".to_string();
        let opacity = 0.8;
        let size = 12;

        let shape = generator.generate_balanced_shape(color, opacity, size);
        let metrics = generator.evaluate_shape_quality(&shape);
        
        // Basic sanity checks on the metrics
        assert!(metrics.compactness >= 0.0 && metrics.compactness <= 1.0);
        assert!(metrics.smoothness >= 0.0 && metrics.smoothness <= 1.0);
        assert!(metrics.balance >= 0.0 && metrics.balance <= 1.0);
        
        // Total score should be in valid range
        let total = metrics.total_score();
        assert!(total >= 0.0 && total <= 1.0);
    }
    
    #[test]
    fn test_shape_smoothing() {
        let grid = TriangularGrid::new(100.0, 4);
        let mut generator = ShapeGenerator::new(&grid, Some(42)); // Fixed seed for deterministic testing

        let color = "#FF0000".to_string();
        let opacity = 0.8;
        let size = 15;

        // Create a shape without smoothing first
        let mut shape = Shape::new(color.clone(), opacity);
        let start_cell = generator.find_center_cells()[0];
        shape.add_cell(start_cell);
        
        // Add some cells in an angular pattern
        let adjacent = grid.adjacent_cells(start_cell);
        for (i, &adj) in adjacent.iter().enumerate() {
            if i % 2 == 0 && shape.cell_count() < size {
                shape.add_cell(adj);
            }
        }
        
        // Apply smoothing
        let original_count = shape.cell_count();
        generator.smooth_shape(&mut shape, size);
        
        // Should have added some cells
        assert!(shape.cell_count() >= original_count);
        
        // Should not exceed target size
        assert!(shape.cell_count() <= size);
    }
}