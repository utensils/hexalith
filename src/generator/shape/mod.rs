use crate::generator::grid::TriangularGrid;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::{HashMap, HashSet, VecDeque};

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

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }
}

/// Shape evaluation metrics for balanced shapes
#[derive(Debug, Clone, Copy)]
pub struct ShapeMetrics {
    pub compactness: f64, // Higher is better (more compact)
    pub smoothness: f64,  // Higher is better (smoother perimeter)
    pub balance: f64,     // Higher is better (more balanced from center)
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
            }
            None => ChaCha8Rng::from_entropy(),
        };

        Self { grid, rng }
    }

    /// Ensures a shape is fully connected - removes disconnected fragments
    fn ensure_connected(&self, shape: &mut Shape) {
        if shape.cells.len() <= 1 {
            return;
        }

        // Find all connected components using BFS
        let components = self.find_connected_components(&shape.cells);

        if components.len() > 1 {
            // Keep only the largest component
            let largest = components
                .into_iter()
                .max_by_key(|c| c.len())
                .unwrap_or_default();

            shape.cells = largest;
        }
    }

    /// Find all connected components in a set of cells
    fn find_connected_components(&self, cells: &[usize]) -> Vec<Vec<usize>> {
        let cell_set: HashSet<_> = cells.iter().cloned().collect();
        let mut visited = HashSet::new();
        let mut components = Vec::new();

        for &cell in cells {
            if visited.contains(&cell) {
                continue;
            }

            // BFS to find all cells connected to this one
            let mut component = Vec::new();
            let mut queue = VecDeque::new();
            queue.push_back(cell);
            visited.insert(cell);

            while let Some(current) = queue.pop_front() {
                component.push(current);

                for &adjacent in &self.grid.adjacent_cells(current) {
                    if cell_set.contains(&adjacent) && !visited.contains(&adjacent) {
                        visited.insert(adjacent);
                        queue.push_back(adjacent);
                    }
                }
            }

            components.push(component);
        }

        components
    }

    /// Calculate dynamic size range based on grid size and shape count
    fn calculate_shape_size_range(&self, shape_count: u8) -> (usize, usize) {
        let total_cells = self.grid.cell_count();
        let avg_cells_per_shape = total_cells / shape_count.max(1) as usize;

        // More balanced size distribution
        let min_size = (avg_cells_per_shape as f32 * 0.5).max(3.0) as usize;
        let max_size = (avg_cells_per_shape as f32 * 1.5) as usize;

        (min_size, max_size)
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
        shapes
            .into_iter()
            .next()
            .unwrap_or_else(|| Shape::new(color, opacity))
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
                score_b
                    .partial_cmp(&score_a)
                    .unwrap_or(std::cmp::Ordering::Equal)
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
                    score_a
                        .partial_cmp(&score_b)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });

                // Remove the worst candidate
                frontier.remove(0);
            }
        }

        // Only smooth the shape if it's not very angular (controlled by randomness)
        if self.rng.gen::<f32>() > randomness {
            self.smooth_shape(&mut shape, target_size);
        }

        // Ensure connectivity
        self.ensure_connected(&mut shape);

        shape
    }

    /// Generate a radial shape that grows along spokes from center
    pub fn generate_radial_shape(
        &mut self,
        color: String,
        opacity: f32,
        target_size: usize,
    ) -> Shape {
        let mut shape = Shape::new(color, opacity);

        if target_size == 0 {
            return shape;
        }

        // Start from center
        let center_cells = self.find_center_cells();
        if center_cells.is_empty() {
            return shape;
        }

        let start_cell = center_cells[0];
        shape.add_cell(start_cell);

        // Number of spokes (3-6)
        let num_spokes = self.rng.gen_range(3..=6);
        let mut spokes: Vec<Vec<usize>> = vec![vec![]; num_spokes];

        // Initialize each spoke with cells in different directions
        let adjacent = self.grid.adjacent_cells(start_cell);
        for (i, &adj) in adjacent.iter().enumerate() {
            if i < num_spokes {
                spokes[i].push(adj);
                shape.add_cell(adj);
            }
        }

        // Grow spokes outward
        let cells_per_spoke = if shape.cell_count() < target_size {
            (target_size - shape.cell_count()) / num_spokes
        } else {
            0
        };

        for spoke in &mut spokes {
            let mut current_end = *spoke.last().unwrap_or(&start_cell);

            for _ in 0..cells_per_spoke {
                if shape.cell_count() >= target_size {
                    break;
                }

                // Find cells that continue in the same direction
                let candidates = self
                    .grid
                    .adjacent_cells(current_end)
                    .into_iter()
                    .filter(|&c| !shape.contains_cell(c))
                    .collect::<Vec<_>>();

                if let Some(&next) = candidates.get(self.rng.gen_range(0..candidates.len().max(1)))
                {
                    shape.add_cell(next);
                    spoke.push(next);
                    current_end = next;
                }
            }
        }

        // Fill in between spokes if needed
        while shape.cell_count() < target_size {
            let boundary = self.find_boundary_cells(&shape.cells.iter().cloned().collect());
            if boundary.is_empty() {
                break;
            }

            let idx = self.rng.gen_range(0..boundary.len());
            shape.add_cell(boundary[idx]);
        }

        self.ensure_connected(&mut shape);
        shape
    }

    /// Generate a ring shape that follows concentric patterns
    pub fn generate_ring_shape(
        &mut self,
        color: String,
        opacity: f32,
        target_size: usize,
    ) -> Shape {
        let mut shape = Shape::new(color.clone(), opacity);

        if target_size == 0 {
            return shape;
        }

        // Start from a ring around the center, not the center itself
        let center_cells = self.find_center_cells();
        if center_cells.len() < 7 {
            // Fall back to angular shape if grid is too small
            return self.generate_angular_shape(color, opacity, target_size);
        }

        // Skip the very center and start from cells at a certain distance
        let ring_start = self.rng.gen_range(3..7);
        let start_cell = center_cells
            .get(ring_start)
            .cloned()
            .unwrap_or(center_cells[0]);
        shape.add_cell(start_cell);

        // Build the ring by following adjacent cells that maintain distance from center
        let hex_center = self.grid.hex_grid().center;
        let initial_distance = if let Some(cell) = self.grid.get_cell(start_cell) {
            cell.centroid.distance(&hex_center)
        } else {
            return shape;
        };

        let tolerance = initial_distance * 0.3;
        let mut current = start_cell;
        let mut visited = HashSet::new();
        visited.insert(current);

        // Try to complete the ring
        while shape.cell_count() < target_size {
            let candidates: Vec<_> = self
                .grid
                .adjacent_cells(current)
                .into_iter()
                .filter(|&c| !shape.contains_cell(c) && !visited.contains(&c))
                .filter(|&c| {
                    if let Some(cell) = self.grid.get_cell(c) {
                        let dist = cell.centroid.distance(&hex_center);
                        (dist - initial_distance).abs() < tolerance
                    } else {
                        false
                    }
                })
                .collect();

            if candidates.is_empty() {
                // Try to find any boundary cell at similar distance
                let boundary = self.find_boundary_cells(&shape.cells.iter().cloned().collect());
                let filtered: Vec<_> = boundary
                    .into_iter()
                    .filter(|&c| {
                        if let Some(cell) = self.grid.get_cell(c) {
                            let dist = cell.centroid.distance(&hex_center);
                            (dist - initial_distance).abs() < tolerance
                        } else {
                            false
                        }
                    })
                    .collect();

                if filtered.is_empty() {
                    break;
                }

                let idx = self.rng.gen_range(0..filtered.len());
                current = filtered[idx];
            } else {
                let idx = self.rng.gen_range(0..candidates.len());
                current = candidates[idx];
            }

            shape.add_cell(current);
            visited.insert(current);
        }

        self.ensure_connected(&mut shape);
        shape
    }

    /// Generate a linear shape that connects edges of the hexagon
    pub fn generate_linear_shape(
        &mut self,
        color: String,
        opacity: f32,
        target_size: usize,
    ) -> Shape {
        let mut shape = Shape::new(color.clone(), opacity);

        if target_size == 0 {
            return shape;
        }

        // Find edge cells
        let all_cells: Vec<_> = (0..self.grid.cell_count()).collect();
        let mut edge_cells = Vec::new();

        for &cell_id in &all_cells {
            let adjacent = self.grid.adjacent_cells(cell_id);
            // Edge cells have fewer than 3 adjacent cells
            if adjacent.len() < 3 {
                edge_cells.push(cell_id);
            }
        }

        if edge_cells.len() < 2 {
            // Fall back if not enough edge cells
            return self.generate_angular_shape(color, opacity, target_size);
        }

        // Pick two edge cells that are far apart
        let start_idx = self.rng.gen_range(0..edge_cells.len());
        let start_cell = edge_cells[start_idx];

        // Find an edge cell far from the start
        let end_cell = edge_cells
            .iter()
            .filter(|&&c| c != start_cell)
            .max_by_key(|&&c| {
                let start_pos = self.grid.get_cell(start_cell).map(|cell| &cell.centroid);
                let end_pos = self.grid.get_cell(c).map(|cell| &cell.centroid);

                if let (Some(s), Some(e)) = (start_pos, end_pos) {
                    (s.distance(e) * 100.0) as i32
                } else {
                    0
                }
            })
            .cloned()
            .unwrap_or(edge_cells[(start_idx + edge_cells.len() / 2) % edge_cells.len()]);

        // Use A* or similar to find path between start and end
        shape.add_cell(start_cell);

        let mut current = start_cell;
        let mut visited = HashSet::new();
        visited.insert(current);

        // Simple pathfinding
        while shape.cell_count() < target_size && current != end_cell {
            let end_pos = self.grid.get_cell(end_cell).map(|c| &c.centroid);

            let candidates: Vec<_> = self
                .grid
                .adjacent_cells(current)
                .into_iter()
                .filter(|&c| !visited.contains(&c))
                .collect();

            if candidates.is_empty() {
                break;
            }

            // Choose cell that gets us closer to the end
            let next = candidates
                .into_iter()
                .min_by_key(|&c| {
                    if let (Some(cell), Some(end)) = (self.grid.get_cell(c), end_pos) {
                        (cell.centroid.distance(end) * 100.0) as i32
                    } else {
                        i32::MAX
                    }
                })
                .unwrap_or(current);

            shape.add_cell(next);
            visited.insert(next);
            current = next;
        }

        // Add some thickness to the line
        let thickness = self.rng.gen_range(1..=2);
        let line_cells = shape.cells.clone();

        for _ in 0..thickness {
            for &cell in &line_cells {
                if shape.cell_count() >= target_size {
                    break;
                }

                let adjacent = self.grid.adjacent_cells(cell);
                for &adj in &adjacent {
                    if !shape.contains_cell(adj)
                        && shape.cell_count() < target_size
                        && self.rng.gen::<f32>() < 0.5
                    {
                        shape.add_cell(adj);
                    }
                }
            }
        }

        self.ensure_connected(&mut shape);
        shape
    }

    /// Generate an edge-hugging shape along the hexagon perimeter
    pub fn generate_edge_shape(
        &mut self,
        color: String,
        opacity: f32,
        target_size: usize,
    ) -> Shape {
        let mut shape = Shape::new(color.clone(), opacity);

        if target_size == 0 {
            return shape;
        }

        // Find all edge cells
        let mut edge_cells = Vec::new();
        for cell_id in 0..self.grid.cell_count() {
            let adjacent = self.grid.adjacent_cells(cell_id);
            if adjacent.len() < 3 {
                edge_cells.push(cell_id);
            }
        }

        if edge_cells.is_empty() {
            return self.generate_angular_shape(color, opacity, target_size);
        }

        // Start from a random edge
        let start_idx = self.rng.gen_range(0..edge_cells.len());
        let start_cell = edge_cells[start_idx];
        shape.add_cell(start_cell);

        // Follow the edge
        let mut current = start_cell;
        let mut visited = HashSet::new();
        visited.insert(current);

        while shape.cell_count() < target_size {
            // Find adjacent edge cells
            let candidates: Vec<_> = self
                .grid
                .adjacent_cells(current)
                .into_iter()
                .filter(|&c| edge_cells.contains(&c) && !visited.contains(&c))
                .collect();

            if candidates.is_empty() {
                // Try any adjacent cell
                let all_adj: Vec<_> = self
                    .grid
                    .adjacent_cells(current)
                    .into_iter()
                    .filter(|&c| !shape.contains_cell(c))
                    .collect();

                if all_adj.is_empty() {
                    // Find a new edge cell to continue from
                    let available: Vec<_> = edge_cells
                        .iter()
                        .filter(|&&c| !shape.contains_cell(c))
                        .cloned()
                        .collect();

                    if available.is_empty() {
                        break;
                    }

                    current = available[self.rng.gen_range(0..available.len())];
                } else {
                    current = all_adj[self.rng.gen_range(0..all_adj.len())];
                }
            } else {
                current = candidates[self.rng.gen_range(0..candidates.len())];
            }

            shape.add_cell(current);
            visited.insert(current);
        }

        self.ensure_connected(&mut shape);
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
            let new_center_x = (center_x * shape.cells.len() as f64 + cell.centroid.x)
                / (shape.cells.len() + 1) as f64;
            let new_center_y = (center_y * shape.cells.len() as f64 + cell.centroid.y)
                / (shape.cells.len() + 1) as f64;

            let center_shift =
                ((new_center_x - center_x).powi(2) + (new_center_y - center_y).powi(2)).sqrt();

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
                        let adj_external: Vec<usize> = self
                            .grid
                            .adjacent_cells(adj)
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
            score_b
                .partial_cmp(&score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
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
            1.0 - ((perimeter - ideal_ratio) / perimeter).clamp(0.0, 1.0)
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
        // Now we have more variety of shape types
        let shape_type = self.rng.gen_range(0..7);

        match shape_type {
            0 => self.generate_center_shape(color, opacity, target_size),
            1 => self.generate_angular_shape(color, opacity, target_size),
            2 => self.generate_balanced_shape(color, opacity, target_size),
            3 => self.generate_radial_shape(color, opacity, target_size),
            4 => self.generate_ring_shape(color, opacity, target_size),
            5 => self.generate_linear_shape(color, opacity, target_size),
            6 => self.generate_edge_shape(color, opacity, target_size),
            _ => self.generate_angular_shape(color, opacity, target_size),
        }
    }

    /// Generate unique, non-center-based shapes with more variety
    pub fn generate_unique_shape(
        &mut self,
        color: String,
        opacity: f32,
        target_size: usize,
    ) -> Shape {
        // Heavily favor edge-based and cross-canvas patterns
        let pattern_type = self.rng.gen_range(0..100);

        match pattern_type {
            0..=25 => self.generate_cross_canvas_shape(color, opacity, target_size), // 25%
            26..=45 => self.generate_edge_to_edge_shape(color, opacity, target_size), // 20%
            46..=60 => self.generate_diagonal_cross_shape(color, opacity, target_size), // 15%
            61..=70 => self.generate_extreme_edge_shape(color, opacity, target_size), // 10%
            71..=78 => self.generate_edge_crawl_shape(color, opacity, target_size),  // 8%
            79..=85 => self.generate_snake_shape(color, opacity, target_size),       // 7%
            86..=90 => self.generate_diagonal_shape(color, opacity, target_size),    // 5%
            91..=94 => self.generate_corner_shape(color, opacity, target_size),      // 4%
            95..=97 => self.generate_arc_shape(color, opacity, target_size),         // 3%
            _ => self.generate_scatter_shape(color, opacity, target_size),           // 3%
        }
    }

    /// Generate a scattered shape with random distribution
    fn generate_scatter_shape(&mut self, color: String, opacity: f32, target_size: usize) -> Shape {
        let mut shape = Shape::new(color, opacity);

        if target_size == 0 {
            return shape;
        }

        // Get all available cells
        let all_cells: Vec<_> = (0..self.grid.cell_count()).collect();

        // Pick random starting points
        let num_clusters = self.rng.gen_range(2..=4);
        let mut clusters = Vec::new();

        for _ in 0..num_clusters {
            let idx = self.rng.gen_range(0..all_cells.len());
            clusters.push(all_cells[idx]);
        }

        // Grow from each cluster
        let cells_per_cluster = target_size / num_clusters;

        for &start in &clusters {
            let mut cluster_cells = vec![start];
            shape.add_cell(start);

            let mut queue = VecDeque::new();
            queue.push_back(start);

            while cluster_cells.len() < cells_per_cluster && !queue.is_empty() {
                let current = queue.pop_front().unwrap();

                let adjacent = self.grid.adjacent_cells(current);
                let mut candidates: Vec<_> = adjacent
                    .into_iter()
                    .filter(|&c| !shape.contains_cell(c))
                    .collect();

                candidates.shuffle(&mut self.rng);

                for &adj in candidates.iter().take(2) {
                    if cluster_cells.len() < cells_per_cluster {
                        shape.add_cell(adj);
                        cluster_cells.push(adj);
                        queue.push_back(adj);
                    }
                }
            }
        }

        // Connect clusters if possible
        self.connect_clusters(&mut shape);
        self.ensure_connected(&mut shape);

        shape
    }

    /// Generate a spiral shape
    #[allow(dead_code)]
    fn generate_spiral_shape(&mut self, color: String, opacity: f32, target_size: usize) -> Shape {
        let mut shape = Shape::new(color, opacity);

        if target_size == 0 {
            return shape;
        }

        // Start from a point offset from center
        let center_cells = self.find_center_cells();
        let start_idx = self.rng.gen_range(3..center_cells.len().min(10));
        let start_cell = center_cells
            .get(start_idx)
            .cloned()
            .unwrap_or(center_cells[0]);

        shape.add_cell(start_cell);

        let hex_center = self.grid.hex_grid().center;
        let mut current = start_cell;
        let mut angle = 0.0;
        let angle_increment = std::f64::consts::PI / 3.0; // 60 degrees

        while shape.cell_count() < target_size {
            // Find the cell that continues the spiral
            let current_pos = self.grid.get_cell(current).map(|c| &c.centroid);
            if current_pos.is_none() {
                break;
            }

            let candidates = self.grid.adjacent_cells(current);
            let next = candidates
                .into_iter()
                .filter(|&c| !shape.contains_cell(c))
                .min_by_key(|&c| {
                    if let Some(cell) = self.grid.get_cell(c) {
                        let dx = cell.centroid.x - hex_center.x;
                        let dy = cell.centroid.y - hex_center.y;
                        let cell_angle = dy.atan2(dx);
                        let angle_diff = (cell_angle - angle).abs();
                        (angle_diff * 1000.0) as i32
                    } else {
                        i32::MAX
                    }
                });

            if let Some(next_cell) = next {
                shape.add_cell(next_cell);
                current = next_cell;
                angle += angle_increment * 0.3; // Slowly increase angle for spiral effect
            } else {
                // Try to find any adjacent cell to continue
                let adjacent: Vec<_> = self
                    .grid
                    .adjacent_cells(current)
                    .into_iter()
                    .filter(|&c| !shape.contains_cell(c))
                    .collect();

                if adjacent.is_empty() {
                    break;
                }

                current = adjacent[self.rng.gen_range(0..adjacent.len())];
                shape.add_cell(current);
            }
        }

        self.ensure_connected(&mut shape);
        shape
    }

    /// Generate a diagonal stripe shape
    fn generate_diagonal_shape(
        &mut self,
        color: String,
        opacity: f32,
        target_size: usize,
    ) -> Shape {
        let mut shape = Shape::new(color, opacity);

        if target_size == 0 {
            return shape;
        }

        // Choose a diagonal direction
        let direction = self.rng.gen_range(0..3); // 3 diagonal directions in hex grid

        // Get all cells and sort by diagonal position
        let mut cells_by_diagonal: Vec<(usize, f64)> = Vec::new();

        for i in 0..self.grid.cell_count() {
            if let Some(cell) = self.grid.get_cell(i) {
                let diagonal_pos = match direction {
                    0 => cell.centroid.x + cell.centroid.y,
                    1 => cell.centroid.x - cell.centroid.y,
                    _ => cell.centroid.x * 0.5 + cell.centroid.y * 0.866, // 30-degree angle
                };
                cells_by_diagonal.push((i, diagonal_pos));
            }
        }

        cells_by_diagonal.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Pick a stripe position
        let stripe_start = self
            .rng
            .gen_range(cells_by_diagonal.len() / 4..cells_by_diagonal.len() * 3 / 4);

        // Add cells within the stripe
        for i in stripe_start..cells_by_diagonal.len() {
            if shape.cell_count() >= target_size {
                break;
            }

            let (cell_id, pos) = cells_by_diagonal[i];

            // Check if this cell is within stripe width
            let stripe_center = cells_by_diagonal[stripe_start].1;
            let stripe_width = 30.0; // Adjust for desired thickness

            if (pos - stripe_center).abs() < stripe_width {
                shape.add_cell(cell_id);
            }
        }

        self.ensure_connected(&mut shape);
        shape
    }

    /// Generate a shape starting from a corner
    fn generate_corner_shape(&mut self, color: String, opacity: f32, target_size: usize) -> Shape {
        let mut shape = Shape::new(color.clone(), opacity);

        if target_size == 0 {
            return shape;
        }

        // Find corner cells (vertices of the hexagon)
        let mut corner_cells = Vec::new();
        let hex_center = self.grid.hex_grid().center;

        for i in 0..self.grid.cell_count() {
            if let Some(cell) = self.grid.get_cell(i) {
                let adjacent = self.grid.adjacent_cells(i);
                // Corner cells typically have 2 adjacent cells
                if adjacent.len() == 2 {
                    let dist = cell.centroid.distance(&hex_center);
                    corner_cells.push((i, dist));
                }
            }
        }

        // Sort by distance from center (furthest = corners)
        corner_cells.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        if corner_cells.is_empty() {
            return self.generate_edge_shape(color, opacity, target_size);
        }

        // Pick a corner
        let corner_idx = self.rng.gen_range(0..corner_cells.len().min(6));
        let start_cell = corner_cells[corner_idx].0;
        shape.add_cell(start_cell);

        // Grow from corner inward
        let mut queue = VecDeque::new();
        queue.push_back(start_cell);

        while shape.cell_count() < target_size && !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            let adjacent = self.grid.adjacent_cells(current);

            for &adj in &adjacent {
                if !shape.contains_cell(adj) && shape.cell_count() < target_size {
                    shape.add_cell(adj);
                    queue.push_back(adj);
                }
            }
        }

        self.ensure_connected(&mut shape);
        shape
    }

    /// Generate an arc shape
    fn generate_arc_shape(&mut self, color: String, opacity: f32, target_size: usize) -> Shape {
        let mut shape = Shape::new(color, opacity);

        if target_size == 0 {
            return shape;
        }

        let hex_center = self.grid.hex_grid().center;

        // Choose arc parameters
        let radius = self.rng.gen_range(30.0..70.0);
        let start_angle = self.rng.gen_range(0.0..std::f64::consts::TAU);
        let arc_length = self
            .rng
            .gen_range(std::f64::consts::PI / 3.0..std::f64::consts::PI);

        // Find cells along the arc
        for i in 0..self.grid.cell_count() {
            if shape.cell_count() >= target_size {
                break;
            }

            if let Some(cell) = self.grid.get_cell(i) {
                let dx = cell.centroid.x - hex_center.x;
                let dy = cell.centroid.y - hex_center.y;
                let dist = (dx * dx + dy * dy).sqrt();
                let angle = dy.atan2(dx);

                // Check if cell is on the arc
                if (dist - radius).abs() < 15.0 {
                    let angle_diff = (angle - start_angle).rem_euclid(std::f64::consts::TAU);
                    if angle_diff < arc_length {
                        shape.add_cell(i);
                    }
                }
            }
        }

        self.ensure_connected(&mut shape);
        shape
    }

    /// Generate a zigzag shape
    #[allow(dead_code)]
    fn generate_zigzag_shape(&mut self, color: String, opacity: f32, target_size: usize) -> Shape {
        let mut shape = Shape::new(color, opacity);

        if target_size == 0 {
            return shape;
        }

        // Start from a random position
        let start = self.rng.gen_range(0..self.grid.cell_count());
        shape.add_cell(start);

        let mut current = start;
        let mut direction = self.rng.gen_range(0..6); // 6 directions in hex grid
        let mut steps_in_direction = 0;
        let max_steps = 3;

        while shape.cell_count() < target_size {
            let adjacent = self.grid.adjacent_cells(current);

            // Try to continue in current direction
            let candidates: Vec<_> = adjacent
                .iter()
                .enumerate()
                .filter(|(_, &c)| !shape.contains_cell(c))
                .collect();

            if candidates.is_empty() {
                break;
            }

            // Change direction for zigzag effect
            if steps_in_direction >= max_steps || self.rng.gen::<f32>() < 0.3 {
                direction = (direction + self.rng.gen_range(1..=2)) % 6;
                steps_in_direction = 0;
            }

            // Try to move in the preferred direction
            let next = if let Some((_idx, &cell)) = candidates
                .iter()
                .find(|(idx, _)| *idx == direction % candidates.len())
            {
                cell
            } else {
                *candidates[self.rng.gen_range(0..candidates.len())].1
            };

            shape.add_cell(next);
            current = next;
            steps_in_direction += 1;
        }

        self.ensure_connected(&mut shape);
        shape
    }

    /// Connect disconnected clusters in a shape
    fn connect_clusters(&mut self, shape: &mut Shape) {
        let components = self.find_connected_components(&shape.cells);

        if components.len() <= 1 {
            return;
        }

        // Try to connect components by finding shortest paths
        for i in 1..components.len() {
            let comp1 = &components[0];
            let comp2 = &components[i];

            // Find closest pair of cells between components
            let mut min_dist = f64::MAX;
            let mut closest_pair = (0, 0);

            for &c1 in comp1 {
                if let Some(cell1) = self.grid.get_cell(c1) {
                    for &c2 in comp2 {
                        if let Some(cell2) = self.grid.get_cell(c2) {
                            let dist = cell1.centroid.distance(&cell2.centroid);
                            if dist < min_dist {
                                min_dist = dist;
                                closest_pair = (c1, c2);
                            }
                        }
                    }
                }
            }

            // Try to add cells to connect the closest pair
            // This is a simplified approach - just add cells between them
            let (start, _end) = closest_pair;
            let adjacent = self.grid.adjacent_cells(start);

            for &adj in &adjacent {
                if !shape.contains_cell(adj) {
                    shape.add_cell(adj);
                    break; // Just add one connector for simplicity
                }
            }
        }
    }

    /// Generate an extreme edge shape that starts from random edge
    fn generate_extreme_edge_shape(
        &mut self,
        color: String,
        opacity: f32,
        target_size: usize,
    ) -> Shape {
        let mut shape = Shape::new(color, opacity);

        if target_size == 0 {
            return shape;
        }

        // Get ALL edge cells
        let mut edge_cells = Vec::new();
        for i in 0..self.grid.cell_count() {
            let adjacent = self.grid.adjacent_cells(i);
            if adjacent.len() < 3 {
                edge_cells.push(i);
            }
        }

        if edge_cells.is_empty() {
            return shape;
        }

        // Start from a completely random edge
        let start = edge_cells[self.rng.gen_range(0..edge_cells.len())];
        shape.add_cell(start);

        // Grow chaotically from the edge
        let mut current_front = vec![start];
        let mut next_front = Vec::new();

        while shape.cell_count() < target_size && !current_front.is_empty() {
            for &cell in &current_front {
                let adjacent = self.grid.adjacent_cells(cell);
                let mut candidates: Vec<_> = adjacent
                    .into_iter()
                    .filter(|&c| !shape.contains_cell(c))
                    .collect();

                candidates.shuffle(&mut self.rng);

                // Add 1-3 random adjacent cells
                let num_to_add = self.rng.gen_range(1..=3.min(candidates.len()).max(1));
                for &adj in candidates.iter().take(num_to_add) {
                    if shape.cell_count() < target_size {
                        shape.add_cell(adj);
                        next_front.push(adj);
                    }
                }
            }

            current_front = next_front;
            next_front = Vec::new();
        }

        self.ensure_connected(&mut shape);
        shape
    }

    /// Generate a shape that crawls along edges
    fn generate_edge_crawl_shape(
        &mut self,
        color: String,
        opacity: f32,
        target_size: usize,
    ) -> Shape {
        let mut shape = Shape::new(color, opacity);

        if target_size == 0 {
            return shape;
        }

        // Start from random edge
        let mut edge_cells = Vec::new();
        for i in 0..self.grid.cell_count() {
            if self.grid.adjacent_cells(i).len() < 3 {
                edge_cells.push(i);
            }
        }

        if edge_cells.is_empty() {
            return shape;
        }

        let start = edge_cells[self.rng.gen_range(0..edge_cells.len())];
        shape.add_cell(start);

        let mut current = start;

        // Crawl along edge, occasionally moving inward
        while shape.cell_count() < target_size {
            let adjacent = self.grid.adjacent_cells(current);
            let candidates: Vec<_> = adjacent
                .into_iter()
                .filter(|&c| !shape.contains_cell(c))
                .collect();

            if candidates.is_empty() {
                // Pick a new starting point from shape boundary
                let boundary: Vec<_> = shape
                    .cells
                    .iter()
                    .filter(|&&c| {
                        self.grid
                            .adjacent_cells(c)
                            .iter()
                            .any(|&adj| !shape.contains_cell(adj))
                    })
                    .cloned()
                    .collect();

                if boundary.is_empty() {
                    break;
                }

                current = boundary[self.rng.gen_range(0..boundary.len())];
                continue;
            }

            // Prefer edge cells 70% of the time
            let edge_candidates: Vec<_> = candidates
                .iter()
                .filter(|&&c| edge_cells.contains(&c))
                .cloned()
                .collect();

            let next = if !edge_candidates.is_empty() && self.rng.gen::<f32>() < 0.7 {
                edge_candidates[self.rng.gen_range(0..edge_candidates.len())]
            } else {
                candidates[self.rng.gen_range(0..candidates.len())]
            };

            shape.add_cell(next);
            current = next;
        }

        self.ensure_connected(&mut shape);
        shape
    }

    /// Generate a random walk shape
    #[allow(dead_code)]
    fn generate_random_walk_shape(
        &mut self,
        color: String,
        opacity: f32,
        target_size: usize,
    ) -> Shape {
        let mut shape = Shape::new(color, opacity);

        if target_size == 0 {
            return shape;
        }

        // Start from ANY random cell
        let start = self.rng.gen_range(0..self.grid.cell_count());
        shape.add_cell(start);

        let mut current = start;
        let mut visited = HashSet::new();
        visited.insert(start);

        // Random walk with memory
        while shape.cell_count() < target_size {
            let adjacent = self.grid.adjacent_cells(current);
            let unvisited: Vec<_> = adjacent
                .iter()
                .filter(|&&c| !visited.contains(&c))
                .cloned()
                .collect();

            if unvisited.is_empty() {
                // Jump to a random cell adjacent to the shape
                let boundary: Vec<_> = shape
                    .cells
                    .iter()
                    .flat_map(|&c| self.grid.adjacent_cells(c))
                    .filter(|&c| !shape.contains_cell(c))
                    .collect();

                if boundary.is_empty() {
                    break;
                }

                current = boundary[self.rng.gen_range(0..boundary.len())];
            } else {
                current = unvisited[self.rng.gen_range(0..unvisited.len())];
            }

            shape.add_cell(current);
            visited.insert(current);

            // Occasionally add multiple cells at once for variety
            if self.rng.gen::<f32>() < 0.2 {
                let adjacent = self.grid.adjacent_cells(current);
                for &adj in adjacent.iter() {
                    if !shape.contains_cell(adj)
                        && shape.cell_count() < target_size
                        && self.rng.gen::<f32>() < 0.5
                    {
                        shape.add_cell(adj);
                        visited.insert(adj);
                    }
                }
            }
        }

        self.ensure_connected(&mut shape);
        shape
    }

    /// Generate a burst shape from random point
    #[allow(dead_code)]
    fn generate_burst_shape(&mut self, color: String, opacity: f32, target_size: usize) -> Shape {
        let mut shape = Shape::new(color, opacity);

        if target_size == 0 {
            return shape;
        }

        // Start from random point
        let start = self.rng.gen_range(0..self.grid.cell_count());
        shape.add_cell(start);

        // Add all adjacent cells in bursts
        let mut wave = vec![start];
        let burst_probability = 0.7;

        while shape.cell_count() < target_size && !wave.is_empty() {
            let mut next_wave = Vec::new();

            for &cell in &wave {
                let adjacent = self.grid.adjacent_cells(cell);

                for &adj in &adjacent {
                    if !shape.contains_cell(adj)
                        && shape.cell_count() < target_size
                        && self.rng.gen::<f32>() < burst_probability
                    {
                        shape.add_cell(adj);
                        next_wave.push(adj);
                    }
                }
            }

            wave = next_wave;

            // Occasionally skip to a new location
            if wave.is_empty() && shape.cell_count() < target_size {
                let new_start = self.rng.gen_range(0..self.grid.cell_count());
                if !shape.contains_cell(new_start) {
                    shape.add_cell(new_start);
                    wave.push(new_start);
                }
            }
        }

        self.ensure_connected(&mut shape);
        shape
    }

    /// Generate a snake-like shape
    fn generate_snake_shape(&mut self, color: String, opacity: f32, target_size: usize) -> Shape {
        let mut shape = Shape::new(color, opacity);

        if target_size == 0 {
            return shape;
        }

        // Start from edge
        let mut edge_cells = Vec::new();
        for i in 0..self.grid.cell_count() {
            if self.grid.adjacent_cells(i).len() < 3 {
                edge_cells.push(i);
            }
        }

        let start = if edge_cells.is_empty() {
            self.rng.gen_range(0..self.grid.cell_count())
        } else {
            edge_cells[self.rng.gen_range(0..edge_cells.len())]
        };

        shape.add_cell(start);
        let mut current = start;
        let mut direction = self.rng.gen_range(0..6);

        // Snake movement
        while shape.cell_count() < target_size {
            let adjacent = self.grid.adjacent_cells(current);

            // Try to continue in the same direction
            let mut next = None;

            // Look for cell in preferred direction
            if direction < adjacent.len() {
                let candidate = adjacent[direction];
                if !shape.contains_cell(candidate) {
                    next = Some(candidate);
                }
            }

            // If can't continue straight, turn
            if next.is_none() {
                let candidates: Vec<_> = adjacent
                    .into_iter()
                    .filter(|&c| !shape.contains_cell(c))
                    .collect();

                if !candidates.is_empty() {
                    // Prefer turning slightly rather than reversing
                    let new_dir = self.rng.gen_range(0..candidates.len());
                    next = Some(candidates[new_dir]);
                    direction = (direction + self.rng.gen_range(1..=2)) % 6;
                }
            }

            match next {
                Some(n) => {
                    shape.add_cell(n);
                    current = n;

                    // Snake thickness - occasionally add adjacent cells
                    if self.rng.gen::<f32>() < 0.3 {
                        let adj = self.grid.adjacent_cells(n);
                        for &a in adj.iter().take(1) {
                            if !shape.contains_cell(a) && shape.cell_count() < target_size {
                                shape.add_cell(a);
                            }
                        }
                    }
                }
                None => {
                    // Find new starting point
                    let boundary: Vec<_> = shape
                        .cells
                        .iter()
                        .flat_map(|&c| self.grid.adjacent_cells(c))
                        .filter(|&c| !shape.contains_cell(c))
                        .collect();

                    if boundary.is_empty() {
                        break;
                    }

                    current = boundary[self.rng.gen_range(0..boundary.len())];
                    shape.add_cell(current);
                    direction = self.rng.gen_range(0..6);
                }
            }
        }

        self.ensure_connected(&mut shape);
        shape
    }

    /// Generates multiple shapes with option for unique patterns
    pub fn generate_unique_shapes(
        &mut self,
        colors: Vec<String>,
        opacity: f32,
        count: usize,
        size_range: (usize, usize),
    ) -> Vec<Shape> {
        let mut shapes = Vec::with_capacity(count);
        let mut used_cells = HashSet::new();

        // Use dynamic size calculation
        let (min_size, max_size) = self.calculate_shape_size_range(count as u8);
        let size_range = if size_range.0 == 0 && size_range.1 == 0 {
            (min_size, max_size)
        } else {
            size_range
        };

        for i in 0..count {
            let color = if !colors.is_empty() && i < colors.len() {
                colors[i].clone()
            } else {
                format!("#PLACEHOLDER{i}")
            };

            let size = self.rng.gen_range(size_range.0..=size_range.1);

            // Generate unique shape, avoiding used cells
            let mut shape = self.generate_unique_shape(color, opacity, size);

            // Remove cells that overlap with existing shapes
            shape.cells.retain(|&c| !used_cells.contains(&c));
            self.ensure_connected(&mut shape);

            // Only add non-empty shapes
            if !shape.is_empty() {
                for &cell_id in &shape.cells {
                    used_cells.insert(cell_id);
                }
                shapes.push(shape);
            }
        }

        shapes
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

        // Use dynamic size calculation
        let (min_size, max_size) = self.calculate_shape_size_range(count as u8);
        let size_range = if size_range.0 == 0 && size_range.1 == 0 {
            (min_size, max_size)
        } else {
            size_range
        };

        // Generate the first shape - variety in types
        if count > 0 {
            let size = self.rng.gen_range(size_range.0..=size_range.1);
            let color = if colors.is_empty() {
                String::from("#FF0000")
            } else {
                colors[0].clone()
            };

            // First shape can be any type
            let first_shape = self.generate_random_shape(color, opacity, size);

            // Add the shape's cells to used_cells
            for &cell_id in &first_shape.cells {
                used_cells.insert(cell_id);
            }

            shapes.push(first_shape);
        }

        // Generate remaining shapes with better distribution
        for i in 1..count {
            let color = if !colors.is_empty() && i < colors.len() {
                colors[i].clone()
            } else {
                format!("#PLACEHOLDER{i}")
            };

            let size = self.rng.gen_range(size_range.0..=size_range.1);

            // Mix of strategies for subsequent shapes
            let strategy = self.rng.gen_range(0..10);
            let shape = match strategy {
                0..=2 => {
                    // 30% - Connect to existing shapes
                    self.generate_connected_shape(color, opacity, size, &used_cells)
                }
                3..=4 => {
                    // 20% - Avoid existing shapes (for better distribution)
                    self.generate_shape_avoiding_cells(color, opacity, size, &used_cells)
                }
                5 => {
                    // 10% - Edge shapes
                    let mut shape = self.generate_edge_shape(color, opacity, size);
                    // Remove cells that overlap with existing shapes
                    shape.cells.retain(|&c| !used_cells.contains(&c));
                    self.ensure_connected(&mut shape);
                    shape
                }
                6 => {
                    // 10% - Linear shapes
                    let mut shape = self.generate_linear_shape(color, opacity, size);
                    // Remove cells that overlap with existing shapes
                    shape.cells.retain(|&c| !used_cells.contains(&c));
                    self.ensure_connected(&mut shape);
                    shape
                }
                7 => {
                    // 10% - Ring shapes
                    // Allow some overlap for rings
                    self.generate_ring_shape(color, opacity, size)
                }
                _ => {
                    // 20% - Radial or angular from unused areas
                    if self.rng.gen::<bool>() {
                        let mut shape = self.generate_radial_shape(color, opacity, size);
                        shape.cells.retain(|&c| !used_cells.contains(&c));
                        self.ensure_connected(&mut shape);
                        shape
                    } else {
                        self.generate_shape_avoiding_cells(color, opacity, size, &used_cells)
                    }
                }
            };

            // Only add non-empty shapes
            if !shape.is_empty() {
                for &cell_id in &shape.cells {
                    used_cells.insert(cell_id);
                }
                shapes.push(shape);
            }
        }

        // Post-process to fill gaps between shapes
        self.fill_gaps_between_shapes(&mut shapes, &mut used_cells);

        shapes
    }

    /// Fill small gaps between adjacent shapes for cleaner logos
    fn fill_gaps_between_shapes(&mut self, shapes: &mut [Shape], used_cells: &mut HashSet<usize>) {
        // Find cells that are adjacent to multiple different shapes
        let mut gap_cells: HashMap<usize, Vec<usize>> = HashMap::new();

        for (shape_idx, shape) in shapes.iter().enumerate() {
            for &cell in &shape.cells {
                for &adj in &self.grid.adjacent_cells(cell) {
                    if !used_cells.contains(&adj) {
                        gap_cells.entry(adj).or_default().push(shape_idx);
                    }
                }
            }
        }

        // Fill gaps that are adjacent to 2+ shapes
        for (cell_id, adjacent_shapes) in gap_cells {
            if adjacent_shapes.len() >= 2 && self.rng.gen::<f32>() < 0.7 {
                // Pick a random adjacent shape to add this cell to
                let shape_idx = adjacent_shapes[self.rng.gen_range(0..adjacent_shapes.len())];
                if let Some(shape) = shapes.get_mut(shape_idx) {
                    shape.add_cell(cell_id);
                    used_cells.insert(cell_id);
                }
            }
        }
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
        shapes
            .into_iter()
            .next()
            .unwrap_or_else(|| Shape::new(color, opacity))
    }

    /// Generates a shape starting from the center of the hexagon and growing outward
    /// This ensures shapes are connected, not floating isolated, and grow from the center out
    fn generate_center_shape(&mut self, color: String, opacity: f32, target_size: usize) -> Shape {
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
            0 // 80% chance to use the center
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
                    score_b
                        .partial_cmp(&score_a)
                        .unwrap_or(std::cmp::Ordering::Equal)
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

        // Ensure connectivity
        self.ensure_connected(&mut shape);

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
        cells_by_distance.iter().map(|(id, _)| *id).collect()
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
            return self.generate_shape_avoiding_cells(
                color_clone,
                opacity,
                target_size,
                used_cells,
            );
        }

        // Add randomness to choice of starting cell
        boundary_cells.shuffle(&mut self.rng);

        // Sort by distance to center but with randomness
        if self.rng.gen::<f32>() < 0.7 {
            let center_cells = self.find_center_cells();
            boundary_cells.sort_by(|&a, &b| {
                // Find position in center_cells (lower index = closer to center)
                let pos_a = center_cells
                    .iter()
                    .position(|&x| x == a)
                    .unwrap_or(usize::MAX);
                let pos_b = center_cells
                    .iter()
                    .position(|&x| x == b)
                    .unwrap_or(usize::MAX);
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
                if !shape.contains_cell(adj_id)
                    && !used_cells.contains(&adj_id)
                    && !visited.contains(&adj_id)
                {
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
                    score_b
                        .partial_cmp(&score_a)
                        .unwrap_or(std::cmp::Ordering::Equal)
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

        // Ensure connectivity
        self.ensure_connected(&mut shape);

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
                if !shape.contains_cell(adj_id)
                    && !used_cells.contains(&adj_id)
                    && !visited.contains(&adj_id)
                {
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
                    score_b
                        .partial_cmp(&score_a)
                        .unwrap_or(std::cmp::Ordering::Equal)
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

        // Ensure connectivity
        self.ensure_connected(&mut shape);

        shape
    }

    /// Generate a classic style base shape using predefined patterns
    pub fn generate_classic_base_shape(&mut self, color: String, opacity: f32) -> Shape {
        // These are predefined shape definitions for classic mode
        // Each shape consists of 6 triangle indices (0-23)
        let classic_shapes: Vec<Vec<usize>> = vec![
            vec![9, 16, 17, 23, 22, 21],
            vec![17, 23, 22, 21, 15, 14],
            vec![6, 7, 8, 9, 15, 16],
            vec![7, 8, 14, 15, 16, 17],
            vec![9, 16, 17, 23, 14, 15],
            vec![2, 8, 9, 10, 11, 16],
            vec![5, 6, 7, 8, 14, 15],
            vec![8, 9, 15, 16, 17, 18],
            vec![6, 7, 13, 14, 15, 16], // Note: original has 7 triangles, taking first 6
            vec![2, 3, 6, 7, 8, 9],     // Note: original has 7 triangles, taking first 6
            vec![1, 2, 8, 9, 15, 16],   // Note: original has 7 triangles, taking first 6
            vec![2, 7, 8, 9, 16, 17],   // Note: original has 7 triangles, taking first 6
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

        // For classic mode with grid size 2, we should have exactly 24 triangles
        // matching the original implementation
        let total_cells = self.grid.cell_count();

        // Select a random shape from the predefined patterns
        let shape_idx = self.rng.gen_range(0..classic_shapes.len());
        let pattern = &classic_shapes[shape_idx];

        let mut shape = Shape::new(color, opacity);

        // Add cells from the pattern, but only if they exist in our grid
        for &cell_id in pattern {
            if cell_id < total_cells {
                shape.add_cell(cell_id);
            }
        }

        // If we're using a larger grid, we might need to scale the pattern
        if total_cells > 24 && shape.cells.len() < 6 {
            // Try to map the pattern to the larger grid
            // This is an approximation since the original only works with 24 triangles
            let scale = (total_cells as f32 / 24.0).sqrt();
            shape.cells.clear();

            for &cell_id in pattern {
                let scaled_id = ((cell_id as f32 * scale) as usize).min(total_cells - 1);
                shape.add_cell(scaled_id);
            }
        }

        shape
    }

    /// Generate L-shape pattern
    #[allow(dead_code)]
    fn generate_l_shape(&mut self, shape: &mut Shape, target_size: usize) {
        // Start from a random position, not necessarily center
        let start = self.rng.gen_range(0..self.grid.cell_count());
        shape.add_cell(start);

        // Grow in one direction first
        let mut current = start;
        let direction1_steps = target_size / 2;

        for _ in 0..direction1_steps {
            let adjacent = self.grid.adjacent_cells(current);
            let candidates: Vec<_> = adjacent
                .into_iter()
                .filter(|&c| !shape.contains_cell(c))
                .collect();

            if !candidates.is_empty() {
                current = candidates[self.rng.gen_range(0..candidates.len())];
                shape.add_cell(current);
            }
        }

        // Then grow perpendicular
        for _ in shape.cell_count()..target_size {
            let boundary = self.find_boundary_cells(&shape.cells.iter().cloned().collect());
            if !boundary.is_empty() {
                // Pick cells that create an L-shape
                let cell = boundary[self.rng.gen_range(0..boundary.len())];
                shape.add_cell(cell);
            }
        }
    }

    /// Generate diagonal stripe pattern
    #[allow(dead_code)]
    fn generate_diagonal_stripe(&mut self, shape: &mut Shape, target_size: usize) {
        // Pick a starting edge
        let edge_cells = self.find_edge_cells();
        if edge_cells.is_empty() {
            return;
        }

        let start = edge_cells[self.rng.gen_range(0..edge_cells.len())];
        shape.add_cell(start);

        // Grow diagonally across
        let mut current_line = vec![start];

        while shape.cell_count() < target_size && !current_line.is_empty() {
            let mut next_line = Vec::new();

            for &cell in &current_line {
                let adjacent = self.grid.adjacent_cells(cell);

                // Pick cells that continue the diagonal
                for &adj in &adjacent {
                    if !shape.contains_cell(adj) && shape.cell_count() < target_size {
                        shape.add_cell(adj);
                        next_line.push(adj);
                        if shape.cell_count() >= target_size {
                            break;
                        }
                    }
                }
            }

            current_line = next_line;
        }
    }

    /// Generate star/cross pattern
    #[allow(dead_code)]
    fn generate_star_pattern(&mut self, shape: &mut Shape, target_size: usize) {
        // Start from center area
        let center_cells = self.find_center_cells();
        if center_cells.is_empty() {
            return;
        }

        let center = center_cells[self.rng.gen_range(0..center_cells.len().min(3))];
        shape.add_cell(center);

        // Create arms radiating out
        let adjacent = self.grid.adjacent_cells(center);
        let num_arms = 3.min(adjacent.len());

        for i in 0..num_arms {
            if i < adjacent.len() && shape.cell_count() < target_size {
                let mut current = adjacent[i];
                shape.add_cell(current);

                // Extend each arm
                let arm_length = (target_size - 1) / num_arms;
                for _ in 0..arm_length {
                    if shape.cell_count() >= target_size {
                        break;
                    }

                    let next_cells = self.grid.adjacent_cells(current);
                    let candidates: Vec<_> = next_cells
                        .into_iter()
                        .filter(|&c| !shape.contains_cell(c))
                        .collect();

                    if !candidates.is_empty() {
                        current = candidates[self.rng.gen_range(0..candidates.len())];
                        shape.add_cell(current);
                    }
                }
            }
        }
    }

    /// Generate zigzag pattern
    #[allow(dead_code)]
    fn generate_zigzag_pattern(&mut self, shape: &mut Shape, target_size: usize) {
        let start = self.rng.gen_range(0..self.grid.cell_count());
        shape.add_cell(start);

        let mut current = start;
        let mut _direction_changes = 0;
        let mut steps_in_direction = 0;
        let steps_before_turn = 2;

        while shape.cell_count() < target_size {
            let adjacent = self.grid.adjacent_cells(current);
            let candidates: Vec<_> = adjacent
                .into_iter()
                .filter(|&c| !shape.contains_cell(c))
                .collect();

            if candidates.is_empty() {
                break;
            }

            // Change direction periodically to create zigzag
            if steps_in_direction >= steps_before_turn {
                _direction_changes += 1;
                steps_in_direction = 0;
            }

            let next = candidates[self.rng.gen_range(0..candidates.len())];
            shape.add_cell(next);
            current = next;
            steps_in_direction += 1;
        }
    }

    /// Generate chevron/arrow pattern
    #[allow(dead_code)]
    fn generate_chevron_pattern(&mut self, shape: &mut Shape, target_size: usize) {
        // Start from one side
        let edge_cells = self.find_edge_cells();
        if edge_cells.is_empty() {
            return;
        }

        let start = edge_cells[self.rng.gen_range(0..edge_cells.len())];
        shape.add_cell(start);

        // Create two arms that meet at a point
        let mut arm1 = vec![start];
        let mut arm2 = vec![start];

        // Grow first arm
        let mut current = start;
        for _ in 0..target_size / 2 {
            if shape.cell_count() >= target_size {
                break;
            }

            let adjacent = self.grid.adjacent_cells(current);
            let candidates: Vec<_> = adjacent
                .into_iter()
                .filter(|&c| !shape.contains_cell(c))
                .collect();

            if !candidates.is_empty() {
                current = candidates[0]; // Take first direction
                shape.add_cell(current);
                arm1.push(current);
            }
        }

        // Grow second arm in different direction
        current = start;
        for _ in 0..target_size / 2 {
            if shape.cell_count() >= target_size {
                break;
            }

            let adjacent = self.grid.adjacent_cells(current);
            let candidates: Vec<_> = adjacent
                .into_iter()
                .filter(|&c| !shape.contains_cell(c) && !arm1.contains(&c))
                .collect();

            if !candidates.is_empty() {
                current = candidates[candidates.len() - 1]; // Take different direction
                shape.add_cell(current);
                arm2.push(current);
            }
        }
    }

    /// Find edge cells of the grid
    fn find_edge_cells(&self) -> Vec<usize> {
        let mut edge_cells = Vec::new();

        for i in 0..self.grid.cell_count() {
            let adjacent = self.grid.adjacent_cells(i);
            // Edge cells have fewer than maximum neighbors
            if adjacent.len() < 3 {
                edge_cells.push(i);
            }
        }

        edge_cells
    }

    /// Generate a shape that crosses the entire canvas
    fn generate_cross_canvas_shape(
        &mut self,
        color: String,
        opacity: f32,
        target_size: usize,
    ) -> Shape {
        let mut shape = Shape::new(color.clone(), opacity);

        if target_size == 0 {
            return shape;
        }

        // Get all edge cells
        let edge_cells = self.find_edge_cells();
        if edge_cells.len() < 2 {
            return self.generate_extreme_edge_shape(color, opacity, target_size);
        }

        // Pick two edges on opposite sides
        let start = edge_cells[self.rng.gen_range(0..edge_cells.len())];
        shape.add_cell(start);

        // Find the farthest edge cell from start
        let mut farthest = start;
        let mut max_dist = 0.0;

        if let Some(start_cell) = self.grid.get_cell(start) {
            for &edge in &edge_cells {
                if let Some(edge_cell) = self.grid.get_cell(edge) {
                    let dist = start_cell.centroid.distance(&edge_cell.centroid);
                    if dist > max_dist {
                        max_dist = dist;
                        farthest = edge;
                    }
                }
            }
        }

        // Create a path from start to farthest
        let mut current = start;
        let mut attempts = 0;

        while shape.cell_count() < target_size && attempts < target_size * 2 {
            attempts += 1;

            // Find the adjacent cell that gets us closer to the target
            let adjacent = self.grid.adjacent_cells(current);
            let mut best_next = None;
            let mut best_dist = f64::MAX;

            if let Some(target_cell) = self.grid.get_cell(farthest) {
                for &adj in &adjacent {
                    if !shape.contains_cell(adj) {
                        if let Some(adj_cell) = self.grid.get_cell(adj) {
                            let dist = adj_cell.centroid.distance(&target_cell.centroid);
                            if dist < best_dist {
                                best_dist = dist;
                                best_next = Some(adj);
                            }
                        }
                    }
                }
            }

            if let Some(next) = best_next {
                shape.add_cell(next);
                current = next;

                // Occasionally branch out to make the shape thicker
                if self.rng.gen::<f32>() < 0.3 {
                    let side_adjacent = self.grid.adjacent_cells(next);
                    for &adj in side_adjacent.iter().take(2) {
                        if !shape.contains_cell(adj) && shape.cell_count() < target_size {
                            shape.add_cell(adj);
                        }
                    }
                }
            } else {
                // Can't find a path, pick a random boundary cell
                let boundary = self.find_boundary_cells(&shape.cells.iter().cloned().collect());
                if !boundary.is_empty() {
                    current = boundary[self.rng.gen_range(0..boundary.len())];
                    shape.add_cell(current);
                } else {
                    break;
                }
            }
        }

        self.ensure_connected(&mut shape);
        shape
    }

    /// Generate a shape that goes from one edge to another
    fn generate_edge_to_edge_shape(
        &mut self,
        color: String,
        opacity: f32,
        target_size: usize,
    ) -> Shape {
        let mut shape = Shape::new(color.clone(), opacity);

        if target_size == 0 {
            return shape;
        }

        let edge_cells = self.find_edge_cells();
        if edge_cells.len() < 2 {
            return self.generate_extreme_edge_shape(color, opacity, target_size);
        }

        // Start from one edge
        let start = edge_cells[self.rng.gen_range(0..edge_cells.len())];
        shape.add_cell(start);

        // Pick another edge cell that's at least somewhat distant
        let mut target_candidates = Vec::new();
        if let Some(start_cell) = self.grid.get_cell(start) {
            for &edge in &edge_cells {
                if edge != start {
                    if let Some(edge_cell) = self.grid.get_cell(edge) {
                        let dist = start_cell.centroid.distance(&edge_cell.centroid);
                        if dist > 50.0 {
                            // Minimum distance threshold
                            target_candidates.push(edge);
                        }
                    }
                }
            }
        }

        if target_candidates.is_empty() {
            target_candidates = edge_cells.clone();
        }

        let target = target_candidates[self.rng.gen_range(0..target_candidates.len())];

        // Build a thick path between them
        let mut current_layer = vec![start];
        let mut next_layer = Vec::new();

        while shape.cell_count() < target_size && !current_layer.is_empty() {
            for &cell in &current_layer {
                let adjacent = self.grid.adjacent_cells(cell);

                // Add cells that move us generally toward the target
                for &adj in &adjacent {
                    if !shape.contains_cell(adj) && shape.cell_count() < target_size {
                        // Check if this moves us closer to target
                        let mut add_it = false;

                        if let (Some(adj_cell), Some(target_cell)) =
                            (self.grid.get_cell(adj), self.grid.get_cell(target))
                        {
                            let current_dist = if let Some(curr_cell) = self.grid.get_cell(cell) {
                                curr_cell.centroid.distance(&target_cell.centroid)
                            } else {
                                f64::MAX
                            };

                            let new_dist = adj_cell.centroid.distance(&target_cell.centroid);

                            // Accept if it moves us closer or with some probability even if not
                            if new_dist < current_dist || self.rng.gen::<f32>() < 0.3 {
                                add_it = true;
                            }
                        }

                        if add_it {
                            shape.add_cell(adj);
                            next_layer.push(adj);
                        }
                    }
                }
            }

            current_layer = next_layer;
            next_layer = Vec::new();
        }

        self.ensure_connected(&mut shape);
        shape
    }

    /// Generate a diagonal shape that crosses the canvas
    fn generate_diagonal_cross_shape(
        &mut self,
        color: String,
        opacity: f32,
        target_size: usize,
    ) -> Shape {
        let mut shape = Shape::new(color.clone(), opacity);

        if target_size == 0 {
            return shape;
        }

        // Find corner-like edge cells
        let edge_cells = self.find_edge_cells();
        if edge_cells.is_empty() {
            return shape;
        }

        // Pick a starting edge
        let start = edge_cells[self.rng.gen_range(0..edge_cells.len())];
        shape.add_cell(start);

        // Determine diagonal direction
        let diagonal_dir = self.rng.gen_range(0..2); // 0 = NE/SW, 1 = NW/SE

        // Grow diagonally with thickness
        let mut current_band = vec![start];
        let mut next_band = Vec::new();

        while shape.cell_count() < target_size && !current_band.is_empty() {
            for &cell in &current_band {
                let adjacent = self.grid.adjacent_cells(cell);

                for (i, &adj) in adjacent.iter().enumerate() {
                    if !shape.contains_cell(adj) && shape.cell_count() < target_size {
                        // Add cells in diagonal pattern
                        let should_add = match diagonal_dir {
                            0 => i == 0 || i == 1 || i == 4 || i == 5, // Roughly diagonal
                            _ => i == 1 || i == 2 || i == 3 || i == 4, // Other diagonal
                        };

                        if should_add || self.rng.gen::<f32>() < 0.4 {
                            shape.add_cell(adj);
                            next_band.push(adj);
                        }
                    }
                }
            }

            current_band = next_band;
            next_band = Vec::new();

            // Occasionally thicken the diagonal
            if self.rng.gen::<f32>() < 0.3 {
                let boundary = self.find_boundary_cells(&shape.cells.iter().cloned().collect());
                let add_count = (boundary.len() / 3).min(target_size - shape.cell_count());
                for &b in boundary.iter().take(add_count) {
                    shape.add_cell(b);
                }
            }
        }

        self.ensure_connected(&mut shape);
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

    #[test]
    fn test_angular_shape() {
        let grid = TriangularGrid::new(100.0, 4);
        let mut generator = ShapeGenerator::new(&grid, Some(42)); // Fixed seed for deterministic testing

        let color = "#FF0000".to_string();
        let opacity = 0.8;
        let target_size = 10;

        let shape = generator.generate_angular_shape(color, opacity, target_size);

        // Shape should have cells
        assert!(!shape.cells.is_empty());
        assert!(shape.cell_count() <= target_size);

        // Test with extreme case - zero target size
        let shape = generator.generate_angular_shape("#00FF00".to_string(), 0.5, 0);
        assert_eq!(shape.cell_count(), 0);
    }

    #[test]
    fn test_boundary_cells() {
        let grid = TriangularGrid::new(100.0, 4);
        let generator = ShapeGenerator::new(&grid, Some(42)); // Fixed seed for deterministic testing

        // Create a set of used cells
        let mut used_cells = HashSet::new();
        used_cells.insert(0);
        used_cells.insert(1);
        used_cells.insert(2);

        // Find boundary cells
        let boundary = generator.find_boundary_cells(&used_cells);

        // Should have found some boundary cells
        assert!(!boundary.is_empty());

        // Boundary cells should not include any used cells
        for &cell_id in &boundary {
            assert!(!used_cells.contains(&cell_id));
        }
    }

    #[test]
    fn test_shape_avoiding_cells() {
        let grid = TriangularGrid::new(100.0, 4);
        let mut generator = ShapeGenerator::new(&grid, Some(42)); // Fixed seed for deterministic testing

        let color = "#FF0000".to_string();
        let opacity = 0.8;
        let target_size = 10;

        // Create a set of used cells
        let mut used_cells = HashSet::new();
        used_cells.insert(0); // Use the center cell

        // Generate a shape avoiding used cells
        let shape =
            generator.generate_shape_avoiding_cells(color, opacity, target_size, &used_cells);

        // Shape should have cells
        assert!(!shape.cells.is_empty());
        assert!(shape.cell_count() <= target_size);

        // Shape should not include any used cells
        for &cell_id in &shape.cells {
            assert!(!used_cells.contains(&cell_id));
        }

        // Test with all cells used
        let mut all_used = HashSet::new();
        for i in 0..grid.cell_count() {
            all_used.insert(i);
        }

        let shape =
            generator.generate_shape_avoiding_cells("#00FF00".to_string(), 0.5, 5, &all_used);
        // Should be empty since no cells are available
        assert_eq!(shape.cell_count(), 0);
    }

    #[test]
    fn test_balanced_shape() {
        let grid = TriangularGrid::new(100.0, 4);
        let mut generator = ShapeGenerator::new(&grid, Some(42)); // Fixed seed for deterministic testing

        let color = "#FF0000".to_string();
        let opacity = 0.8;
        let target_size = 10;

        let shape = generator.generate_balanced_shape(color, opacity, target_size);

        // Shape should have cells
        assert!(!shape.cells.is_empty());
        assert!(shape.cell_count() <= target_size);

        // Test with entropy-based RNG
        let mut generator = ShapeGenerator::new(&grid, None); // No seed, use entropy
        let shape = generator.generate_balanced_shape("#00FF00".to_string(), 0.5, 8);
        assert!(!shape.cells.is_empty());
    }
}
