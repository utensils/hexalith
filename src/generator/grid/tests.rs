#[cfg(test)]
mod tests {
    use crate::generator::grid::geometry::{HexGrid, Point};
    use crate::generator::grid::triangular::TriangularGrid;

    #[test]
    fn test_hexagon_creation() {
        let center = Point::new(0.0, 0.0);
        let size = 100.0;
        let grid_density = 6;

        let hex_grid = HexGrid::new(size, grid_density, center);

        // Verify that the hexagon has 6 vertices
        assert_eq!(hex_grid.vertices.len(), 6);

        // Verify that all vertices are at the correct distance from the center
        for vertex in &hex_grid.vertices {
            let distance = center.distance(vertex);
            assert!((distance - size).abs() < 1e-6);
        }

        // Verify expected cell count
        assert_eq!(
            hex_grid.expected_cell_count(),
            6 * (grid_density as usize).pow(2)
        );
    }

    #[test]
    fn test_triangular_grid_creation() {
        let size = 100.0;

        // Test with different grid densities
        for grid_density in 3..=8 {
            let grid = TriangularGrid::new(size, grid_density);
            let expected_cells = 6 * (grid_density as usize).pow(2);

            // Verify cell count
            assert_eq!(grid.cell_count(), expected_cells);

            // Each cell should have 3 vertices
            if let Some(cell) = grid.get_cell(0) {
                assert_eq!(cell.vertices.len(), 3);
            }
        }
    }

    #[test]
    fn test_point_in_hexagon() {
        let center = Point::new(0.0, 0.0);
        let size = 100.0;
        let grid_density = 6;

        let hex_grid = HexGrid::new(size, grid_density, center);

        // Points inside the hexagon
        assert!(hex_grid.contains_point(&center));

        // Points on the boundary
        for vertex in &hex_grid.vertices {
            assert!(hex_grid.contains_point(vertex));
        }

        // Points outside the hexagon
        let outside = Point::new(size * 2.0, size * 2.0);
        assert!(!hex_grid.contains_point(&outside));
    }

    #[test]
    fn test_cell_adjacency() {
        let size = 100.0;
        let grid_density = 3; // Small grid for easier testing

        let grid = TriangularGrid::new(size, grid_density);

        // Each cell should have at least one adjacent cell
        for i in 0..grid.cell_count() {
            let adjacent = grid.adjacent_cells(i);
            assert!(!adjacent.is_empty());
        }
    }

    #[test]
    fn test_original_style_grid() {
        let size = 100.0;
        let grid_density = 2; // This should trigger the original style grid

        let grid = TriangularGrid::new(size, grid_density);

        // Original style grid should have exactly 24 cells
        assert_eq!(grid.cell_count(), 24);

        // Verify centroid functionality
        for i in 0..grid.cell_count() {
            let centroid = grid.get_cell_centroid(i);
            assert!(centroid.is_some());
        }

        // Non-existent cell should return None
        assert!(grid.get_cell_centroid(100).is_none());
        assert!(grid.get_cell(100).is_none());

        // Verify cells access method
        let cells = grid.cells();
        assert_eq!(cells.len(), 24);

        // Verify hex_grid and hex_grid_mut methods
        let hex_grid = grid.hex_grid();
        assert_eq!(hex_grid.size, size);

        let mut grid = TriangularGrid::new(size, grid_density);
        let hex_grid_mut = grid.hex_grid_mut();
        assert_eq!(hex_grid_mut.size, size);
    }
}
