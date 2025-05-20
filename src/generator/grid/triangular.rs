use super::geometry::{Cell, HexGrid, Point};

/// Represents a triangular grid subdividing a hexagon
#[derive(Debug)]
pub struct TriangularGrid {
    hex_grid: HexGrid,
}

impl TriangularGrid {
    /// Creates a new triangular grid inside a hexagon
    pub fn new(size: f64, grid_density: u8) -> Self {
        // Create the base hexagonal grid with the specified size and density
        let center = Point::new(0.0, 0.0);
        let mut hex_grid = HexGrid::new(size, grid_density, center);

        // Generate the triangular cells within the hexagon
        let cells = Self::generate_triangular_cells(&hex_grid);
        hex_grid.cells = cells;

        Self { hex_grid }
    }

    /// Generates the triangular cells filling the hexagon
    fn generate_triangular_cells(hex_grid: &HexGrid) -> Vec<Cell> {
        let n = hex_grid.grid_density as usize;
        let mut cells = Vec::with_capacity(hex_grid.expected_cell_count());
        
        // Special case for grid_density=2, generate a grid similar to the original 24-triangle layout
        if n == 2 {
            return Self::generate_original_style_grid(hex_grid);
        }

        // We'll divide the hexagon into 6 triangular sectors (center to each vertex pair)
        // and then further subdivide each sector
        for sector in 0..6 {
            let center = hex_grid.center;
            let v1 = hex_grid.vertices[sector];
            let v2 = hex_grid.vertices[(sector + 1) % 6];

            // Create a triangular sector and subdivide it
            Self::subdivide_triangle(&mut cells, center, v1, v2, n, 0);
        }

        cells
    }

    /// Subdivides a triangle into smaller triangular cells
    fn subdivide_triangle(
        cells: &mut Vec<Cell>,
        p1: Point,
        p2: Point,
        p3: Point,
        divisions: usize,
        base_id: usize,
    ) {
        if divisions <= 1 {
            let id = cells.len() + base_id;
            cells.push(Cell::new(id, [p1, p2, p3]));
            return;
        }

        // Create a grid of points within the triangle
        let mut points = Vec::with_capacity((divisions + 1) * (divisions + 2) / 2);

        for i in 0..=divisions {
            let alpha = i as f64 / divisions as f64;

            for j in 0..=i {
                let beta = if i == 0 { 0.0 } else { j as f64 / i as f64 };

                // Barycentric coordinates to Cartesian
                let gamma = 1.0 - alpha - beta * alpha;
                let x = gamma * p1.x + alpha * (1.0 - beta) * p2.x + alpha * beta * p3.x;
                let y = gamma * p1.y + alpha * (1.0 - beta) * p2.y + alpha * beta * p3.y;

                points.push(Point::new(x, y));
            }
        }

        // Connect the points to form triangular cells
        let mut id = cells.len() + base_id;

        for row in 1..=divisions {
            let row_start = (row * (row + 1)) / 2;
            let prev_row_start = (row - 1) * row / 2;

            for col in 0..row {
                // Upper triangle
                cells.push(Cell::new(
                    id,
                    [
                        points[prev_row_start + col],
                        points[row_start + col],
                        points[row_start + col + 1],
                    ],
                ));
                id += 1;

                // Lower triangle (except for the last column)
                if col < row - 1 {
                    cells.push(Cell::new(
                        id,
                        [
                            points[prev_row_start + col],
                            points[row_start + col + 1],
                            points[prev_row_start + col + 1],
                        ],
                    ));
                    id += 1;
                }
            }
        }
    }

    /// Returns a reference to the underlying hexagonal grid
    pub fn hex_grid(&self) -> &HexGrid {
        &self.hex_grid
    }

    /// Returns a mutable reference to the underlying hexagonal grid
    pub fn hex_grid_mut(&mut self) -> &mut HexGrid {
        &mut self.hex_grid
    }
    
    /// Generates a grid with exactly 24 triangles, similar to the original hexagonal logo generator
    fn generate_original_style_grid(hex_grid: &HexGrid) -> Vec<Cell> {
        let size = hex_grid.size;
        let center = hex_grid.center;
        let mut cells = Vec::with_capacity(24); // Exactly 24 triangles
        
        // Helper function to create a point at specific angle and distance
        let point_at = |angle: f64, distance: f64| -> Point {
            let rad_angle = angle * std::f64::consts::PI / 180.0;
            let x = center.x + distance * rad_angle.cos();
            let y = center.y + distance * rad_angle.sin();
            Point::new(x, y)
        };
        
        let inner_distance = size * 0.5; // Distance to inner points
        
        // Generate the 6 points at the inner hexagon corners
        let mut inner_points = Vec::with_capacity(6);
        for i in 0..6 {
            let angle = i as f64 * 60.0; // 60 degrees per hexagon corner
            inner_points.push(point_at(angle, inner_distance));
        }
        
        // Create the 24 triangles (4 per sector)
        let mut id = 0;
        
        for sector in 0..6 {
            let v1 = hex_grid.vertices[sector];
            let v2 = hex_grid.vertices[(sector + 1) % 6];
            
            let inner1 = inner_points[sector];
            let inner2 = inner_points[(sector + 1) % 6];
            
            // Center to inner point 1
            cells.push(Cell::new(id, [center, inner1, inner2]));
            id += 1;
            
            // Inner point 1 to vertex 1
            cells.push(Cell::new(id, [inner1, v1, v2]));
            id += 1;
            
            // Inner point 1 to inner point 2, to vertex 2
            cells.push(Cell::new(id, [inner1, inner2, v2]));
            id += 1;
            
            // Inner point 2 to center, to inner point 1
            cells.push(Cell::new(id, [inner2, center, inner1]));
            id += 1;
        }
        
        cells
    }

    /// Gets the triangular cell at the specified index
    pub fn get_cell(&self, index: usize) -> Option<&Cell> {
        self.hex_grid.cells.get(index)
    }

    /// Returns the total number of cells in the grid
    pub fn cell_count(&self) -> usize {
        self.hex_grid.cells.len()
    }

    /// Finds all cells adjacent to the given cell
    pub fn adjacent_cells(&self, cell_id: usize) -> Vec<usize> {
        self.hex_grid.adjacent_cells(cell_id)
    }

    /// Gets the centroid point for the cell with the given ID
    pub fn get_cell_centroid(&self, cell_id: usize) -> Option<Point> {
        self.get_cell(cell_id).map(|cell| cell.centroid)
    }

    /// Gets all cells in the grid
    pub fn cells(&self) -> &[Cell] {
        &self.hex_grid.cells
    }
}
