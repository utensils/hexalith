use std::f64::consts::PI;

/// A 2D point using floating point coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// A triangular cell within the hexagonal grid
#[derive(Debug, Clone)]
pub struct Cell {
    pub id: usize,
    pub vertices: [Point; 3],
    pub centroid: Point,
}

impl Cell {
    pub fn new(id: usize, vertices: [Point; 3]) -> Self {
        // Calculate centroid of the triangle
        let centroid = Point::new(
            (vertices[0].x + vertices[1].x + vertices[2].x) / 3.0,
            (vertices[0].y + vertices[1].y + vertices[2].y) / 3.0,
        );

        Self {
            id,
            vertices,
            centroid,
        }
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        // Using barycentric coordinates to check if a point is inside a triangle
        let p1 = &self.vertices[0];
        let p2 = &self.vertices[1];
        let p3 = &self.vertices[2];

        let area =
            0.5 * (-p2.y * p3.x + p1.y * (-p2.x + p3.x) + p1.x * (p2.y - p3.y) + p2.x * p3.y);
        let s = 1.0 / (2.0 * area)
            * (p1.y * p3.x - p1.x * p3.y + (p3.y - p1.y) * point.x + (p1.x - p3.x) * point.y);
        let t = 1.0 / (2.0 * area)
            * (p1.x * p2.y - p1.y * p2.x + (p1.y - p2.y) * point.x + (p2.x - p1.x) * point.y);

        s >= 0.0 && t >= 0.0 && (1.0 - s - t) >= 0.0
    }

    pub fn is_adjacent(&self, other: &Cell) -> bool {
        // Two triangular cells are adjacent if they share exactly two vertices
        let mut shared_vertices = 0;

        for v1 in &self.vertices {
            for v2 in &other.vertices {
                if (v1.x - v2.x).abs() < 1e-6 && (v1.y - v2.y).abs() < 1e-6 {
                    shared_vertices += 1;
                }
            }
        }

        shared_vertices == 2
    }
}

/// Represents the hexagonal grid structure
#[derive(Debug)]
pub struct HexGrid {
    pub size: f64,
    pub grid_density: u8,
    pub center: Point,
    pub vertices: Vec<Point>,
    pub cells: Vec<Cell>,
}

impl HexGrid {
    /// Creates a new hexagonal grid with the specified parameters
    ///
    /// * `size` - The size of the hexagon (distance from center to any vertex)
    /// * `grid_density` - Controls how finely the hexagon is divided (should be 3-8)
    /// * `center` - The center point of the hexagon
    pub fn new(size: f64, grid_density: u8, center: Point) -> Self {
        // Ensure grid density is within acceptable range
        let grid_density = grid_density.clamp(3, 8);

        // Generate the 6 vertices of the regular hexagon
        let mut vertices = Vec::with_capacity(6);
        for i in 0..6 {
            let angle = (i as f64) * PI / 3.0;
            let x = center.x + size * angle.cos();
            let y = center.y + size * angle.sin();
            vertices.push(Point::new(x, y));
        }

        // Create an empty cells vector that will be populated by the triangular grid
        let cells = Vec::new();

        Self {
            size,
            grid_density,
            center,
            vertices,
            cells,
        }
    }

    /// Returns the total number of triangular cells expected in this grid
    pub fn expected_cell_count(&self) -> usize {
        // For a grid density of n, the hexagon contains 6n² triangular cells
        6 * (self.grid_density as usize).pow(2)
    }

    /// Retrieves a cell by its ID
    pub fn get_cell(&self, id: usize) -> Option<&Cell> {
        self.cells.get(id)
    }

    /// Finds all cells adjacent to the specified cell
    pub fn adjacent_cells(&self, cell_id: usize) -> Vec<usize> {
        let mut adjacent = Vec::new();

        if let Some(cell) = self.get_cell(cell_id) {
            for (i, other_cell) in self.cells.iter().enumerate() {
                if i != cell_id && cell.is_adjacent(other_cell) {
                    adjacent.push(i);
                }
            }
        }

        adjacent
    }

    /// Checks if a point is inside the hexagonal boundary
    pub fn contains_point(&self, point: &Point) -> bool {
        // The point-in-polygon algorithm needs special handling for boundary points

        // First check for exact vertex match
        for vertex in &self.vertices {
            if (vertex.x - point.x).abs() < 1e-6 && (vertex.y - point.y).abs() < 1e-6 {
                return true;
            }
        }

        // Then check if point is on any edge
        let mut j = self.vertices.len() - 1;
        for i in 0..self.vertices.len() {
            let vi = &self.vertices[i];
            let vj = &self.vertices[j];

            // Check if point is on this edge
            let edge_length = vj.distance(vi);
            let d1 = point.distance(vi);
            let d2 = point.distance(vj);

            if (d1 + d2 - edge_length).abs() < 1e-6 {
                return true;
            }

            j = i;
        }

        // Finally, check if point is inside using ray casting algorithm
        let mut inside = false;
        j = self.vertices.len() - 1;

        for i in 0..self.vertices.len() {
            let vi = &self.vertices[i];
            let vj = &self.vertices[j];

            if ((vi.y > point.y) != (vj.y > point.y))
                && (point.x < (vj.x - vi.x) * (point.y - vi.y) / (vj.y - vi.y) + vi.x)
            {
                inside = !inside;
            }

            j = i;
        }

        inside
    }
}
