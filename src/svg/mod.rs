use crate::generator::grid::Point;
use crate::generator::{grid::TriangularGrid, Generator};
use crate::Result;
use std::fs;
use std::path::Path;
use svg::node::element::path::Data;
use svg::node::element::Path as SvgPath;
use svg::Document;

/// Converts the generator output to SVG format
pub fn generate_svg(generator: &Generator, width: u32, height: u32) -> Result<String> {
    let grid = match generator.grid() {
        Some(grid) => grid,
        None => return Err("Grid not initialized. Call generate() first.".into()),
    };

    // Create an SVG document
    let mut document = Document::new()
        .set("viewBox", (-100, -100, 200, 200))
        .set("width", width)
        .set("height", height);

    // We don't add the hexagonal boundary anymore to avoid having a border

    // Create a group for each shape
    for shape in generator.shapes() {
        let path_data = create_shape_path(grid, shape.cells.as_slice());

        let shape_path = SvgPath::new()
            .set("d", path_data)
            .set("fill", shape.color.clone())
            .set("fill-opacity", shape.opacity)
            .set("stroke", "none");

        document = document.add(shape_path);
    }

    Ok(document.to_string())
}

// No hexagon boundary is drawn in the SVG to avoid having a border

/// Creates an SVG path for a shape made up of triangular cells
fn create_shape_path(grid: &TriangularGrid, cell_ids: &[usize]) -> Data {
    let mut data = Data::new();

    // Group the cells into contiguous regions to create a more efficient path
    let mut regions = Vec::new();
    let mut visited = vec![false; cell_ids.len()];

    for i in 0..cell_ids.len() {
        if visited[i] {
            continue;
        }

        let mut region = vec![cell_ids[i]];
        visited[i] = true;

        let mut j = 0;
        while j < region.len() {
            let current = region[j];

            // Find adjacent cells in the shape
            for k in 0..cell_ids.len() {
                if !visited[k] {
                    let cell_id = cell_ids[k];
                    if is_adjacent(grid, current, cell_id) {
                        region.push(cell_id);
                        visited[k] = true;
                    }
                }
            }

            j += 1;
        }

        regions.push(region);
    }

    // Create a path for each region
    for region in regions {
        data = add_region_to_path(data, grid, &region);
    }

    data
}

/// Adds a region of cells to the SVG path
fn add_region_to_path(mut data: Data, grid: &TriangularGrid, cell_ids: &[usize]) -> Data {
    if cell_ids.is_empty() {
        return data;
    }

    let boundary = compute_region_boundary(grid, cell_ids);

    // Start the path at the first point
    if let Some(first) = boundary.first() {
        data = data.move_to((first.x, first.y));

        // Add line segments for the rest of the boundary
        for point in boundary.iter().skip(1) {
            data = data.line_to((point.x, point.y));
        }

        // Close the path
        data = data.close();
    }

    data
}

/// Computes the boundary points of a region of cells
fn compute_region_boundary(grid: &TriangularGrid, cell_ids: &[usize]) -> Vec<Point> {
    // Collect all edges of the cells
    let mut edges = Vec::new();

    for &cell_id in cell_ids {
        if let Some(cell) = grid.get_cell(cell_id) {
            // Add the three edges of the cell
            edges.push((cell.vertices[0], cell.vertices[1]));
            edges.push((cell.vertices[1], cell.vertices[2]));
            edges.push((cell.vertices[2], cell.vertices[0]));
        }
    }

    // Find boundary edges (those that appear only once)
    let mut boundary_edges = Vec::new();

    for (i, edge1) in edges.iter().enumerate() {
        let mut is_boundary = true;

        for (j, edge2) in edges.iter().enumerate() {
            if i != j {
                // Check if edge2 is the reverse of edge1
                if (edge1.0.x - edge2.1.x).abs() < 1e-6
                    && (edge1.0.y - edge2.1.y).abs() < 1e-6
                    && (edge1.1.x - edge2.0.x).abs() < 1e-6
                    && (edge1.1.y - edge2.0.y).abs() < 1e-6
                {
                    is_boundary = false;
                    break;
                }
            }
        }

        if is_boundary {
            boundary_edges.push(*edge1);
        }
    }

    // Sort the boundary edges to form a continuous path
    let mut ordered_edges = Vec::new();

    if let Some(first_edge) = boundary_edges.first() {
        ordered_edges.push(*first_edge);
        boundary_edges.remove(0);

        while !boundary_edges.is_empty() {
            let last_edge = ordered_edges.last().unwrap();
            let last_point = last_edge.1;

            // Find the next edge that starts with the last point
            let mut found = false;

            for i in 0..boundary_edges.len() {
                if (boundary_edges[i].0.x - last_point.x).abs() < 1e-6
                    && (boundary_edges[i].0.y - last_point.y).abs() < 1e-6
                {
                    ordered_edges.push(boundary_edges[i]);
                    boundary_edges.remove(i);
                    found = true;
                    break;
                }

                // Also check the reverse direction
                if (boundary_edges[i].1.x - last_point.x).abs() < 1e-6
                    && (boundary_edges[i].1.y - last_point.y).abs() < 1e-6
                {
                    ordered_edges.push((boundary_edges[i].1, boundary_edges[i].0));
                    boundary_edges.remove(i);
                    found = true;
                    break;
                }
            }

            if !found {
                // If we can't find a connected edge, we might have multiple disjoint regions
                // Just add the remaining edges in arbitrary order
                ordered_edges.append(&mut boundary_edges);
            }
        }
    }

    // Extract the points from the ordered edges
    let mut boundary_points = Vec::new();

    for edge in ordered_edges {
        boundary_points.push(edge.0);
    }

    boundary_points
}

/// Checks if two cells are adjacent
fn is_adjacent(grid: &TriangularGrid, cell_id1: usize, cell_id2: usize) -> bool {
    let adjacent = grid.adjacent_cells(cell_id1);
    adjacent.contains(&cell_id2)
}

/// Saves an SVG string to a file
pub fn save_svg<P: AsRef<Path>>(svg: &str, path: P) -> Result<()> {
    fs::write(path, svg)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::Generator;

    #[test]
    fn test_svg_generation() {
        // Create a generator with a simple configuration
        let mut generator = Generator::new(4, 2, 0.8, Some(42));

        // Generate the logo
        generator.generate().unwrap();

        // Generate SVG
        let svg = generate_svg(&generator, 200, 200).unwrap();

        // Basic checks
        assert!(svg.contains("<svg"));
        assert!(svg.contains("viewBox"));
        assert!(svg.contains("</svg>"));

        // Should contain paths for the shapes
        assert!(svg.contains("<path"));
    }
}
