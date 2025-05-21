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

    /// Subdivides a triangle into smaller equiangular triangular cells with connecting edges
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

        // Create a grid of points that ensures equiangular (60-60-60) triangles
        let mut points = Vec::with_capacity((divisions + 1) * (divisions + 2) / 2);

        // Calculate triangular grid points using 60-degree angles
        // This ensures all triangles have the same angles (60-60-60)
        for i in 0..=divisions {
            for j in 0..=divisions - i {
                // Use linear interpolation to maintain equiangular triangles
                let u = i as f64 / divisions as f64;
                let v = j as f64 / divisions as f64;
                let w = 1.0 - u - v;

                // Barycentric coordinates for equiangular triangles
                let x = p1.x * w + p2.x * u + p3.x * v;
                let y = p1.y * w + p2.y * u + p3.y * v;

                points.push(Point::new(x, y));
            }
        }

        // Connect the points to form equiangular triangular cells with shared edges
        let mut id = cells.len() + base_id;

        // Map from triangle grid indices to points array indices
        let idx = |i, j| -> usize {
            let offset = (0..i).map(|k| divisions + 1 - k).sum::<usize>();
            offset + j
        };

        for i in 0..divisions {
            for j in 0..divisions - i {
                // Create first equiangular triangle
                cells.push(Cell::new(
                    id,
                    [
                        points[idx(i, j)],
                        points[idx(i + 1, j)],
                        points[idx(i, j + 1)],
                    ],
                ));
                id += 1;

                // Create second equiangular triangle if not on the edge
                if j < divisions - i - 1 {
                    cells.push(Cell::new(
                        id,
                        [
                            points[idx(i + 1, j)],
                            points[idx(i + 1, j + 1)],
                            points[idx(i, j + 1)],
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

    /// Generates a grid with exactly 24 equiangular triangles, similar to the original hexagonal logo generator
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

        // Use 1/3 and 2/3 distances to create equiangular triangles that grow from center
        let inner_distance1 = size * (1.0 / 3.0); // First inner ring
        let inner_distance2 = size * (2.0 / 3.0); // Second inner ring

        // Generate the points at the inner hexagon corners
        let mut inner_points1 = Vec::with_capacity(6);
        let mut inner_points2 = Vec::with_capacity(6);

        for i in 0..6 {
            let angle = i as f64 * 60.0; // 60 degrees per hexagon corner
            inner_points1.push(point_at(angle, inner_distance1));
            inner_points2.push(point_at(angle, inner_distance2));
        }

        // Create the 24 triangles (4 per sector) that grow from center outward
        let mut id = 0;

        for sector in 0..6 {
            let v = hex_grid.vertices[sector]; // Outer vertex
            let next_sector = (sector + 1) % 6;

            // Inner points from first ring
            let p1 = inner_points1[sector];
            let p1_next = inner_points1[next_sector];

            // Inner points from second ring
            let p2 = inner_points2[sector];
            let p2_next = inner_points2[next_sector];

            // 1. Center triangle (connects to center)
            cells.push(Cell::new(id, [center, p1, p1_next]));
            id += 1;

            // 2. First ring triangle
            cells.push(Cell::new(id, [p1, p2, p1_next]));
            id += 1;

            // 3. Bridge triangle connecting rings
            cells.push(Cell::new(id, [p1_next, p2, p2_next]));
            id += 1;

            // 4. Outer triangle connecting to vertex
            cells.push(Cell::new(id, [p2, v, p2_next]));
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
