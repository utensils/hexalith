use super::grid::{Point, TriangularGrid};
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

const GLYPH_COLUMNS: usize = 5;
const GLYPH_ROWS: usize = 7;
const GLYPH_BOUNDS: GlyphBounds = GlyphBounds {
    min_x: 0.22,
    max_x: 0.78,
    min_y: 0.12,
    max_y: 0.88,
};

#[derive(Clone, Copy)]
struct GlyphBounds {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}

pub struct GlyphLayers {
    pub core: Vec<usize>,
    pub halo: Vec<usize>,
    pub highlight: Vec<usize>,
    pub shadow: Vec<usize>,
    pub highlight_shell: Vec<usize>,
    pub shadow_shell: Vec<usize>,
    pub halo_band: Vec<usize>,
}

pub fn build_glyph_layers(
    grid: &TriangularGrid,
    letter: char,
    rng: &mut ChaCha8Rng,
) -> GlyphLayers {
    let normalized = normalized_cell_points(grid);
    let mut core = normalized
        .iter()
        .filter(|(_, point)| point_matches_glyph(letter, point.x, point.y))
        .map(|(cell_id, _)| *cell_id)
        .collect::<Vec<_>>();

    if core.is_empty() {
        core = normalized
            .iter()
            .filter(|(_, point)| point_matches_glyph_relaxed(letter, point.x, point.y))
            .map(|(cell_id, _)| *cell_id)
            .collect();
    }

    core.sort_unstable();
    core.dedup();

    let core_set = core.iter().copied().collect::<HashSet<_>>();
    let shell = expand_ring(grid, &core_set, 1);
    let shell_set = shell.iter().copied().collect::<HashSet<_>>();
    let shell_start = core_set.union(&shell_set).copied().collect::<HashSet<_>>();
    let halo = expand_ring(grid, &shell_start, 1);
    let halo_set = halo.iter().copied().collect::<HashSet<_>>();

    let highlight_anchor: f64 = rng.gen_range(285.0..345.0);
    let shadow_anchor = (highlight_anchor + 180.0).rem_euclid(360.0);

    let highlight = select_band(&normalized, &core_set, highlight_anchor, 58.0);
    let shadow = select_band(&normalized, &core_set, shadow_anchor, 70.0);
    let highlight_shell = select_band(&normalized, &shell_set, highlight_anchor, 42.0);
    let shadow_shell = select_band(&normalized, &shell_set, shadow_anchor, 42.0);
    let halo_band = select_band(&normalized, &halo_set, shadow_anchor + 30.0, 58.0);

    GlyphLayers {
        core,
        halo,
        highlight,
        shadow,
        highlight_shell,
        shadow_shell,
        halo_band,
    }
}

fn normalized_cell_points(grid: &TriangularGrid) -> Vec<(usize, Point)> {
    let max_x = grid
        .cells()
        .iter()
        .map(|cell| cell.centroid.x.abs())
        .fold(1.0_f64, f64::max);
    let max_y = grid
        .cells()
        .iter()
        .map(|cell| cell.centroid.y.abs())
        .fold(1.0_f64, f64::max);

    grid.cells()
        .iter()
        .map(|cell| {
            (
                cell.id,
                Point::new(
                    0.5 + (cell.centroid.x / max_x) * 0.5,
                    0.5 - (cell.centroid.y / max_y) * 0.5,
                ),
            )
        })
        .collect()
}

fn point_matches_glyph(letter: char, x: f64, y: f64) -> bool {
    point_matches_glyph_with_padding(letter, x, y, 0.03)
}

fn point_matches_glyph_relaxed(letter: char, x: f64, y: f64) -> bool {
    point_matches_glyph_with_padding(letter, x, y, 0.06)
}

fn point_matches_glyph_with_padding(letter: char, x: f64, y: f64, padding: f64) -> bool {
    let sample_offsets = [
        (0.0, 0.0),
        (-padding, 0.0),
        (padding, 0.0),
        (0.0, -padding),
        (0.0, padding),
    ];

    sample_offsets.into_iter().any(|(dx, dy)| {
        let sample_x = x + dx;
        let sample_y = y + dy;
        glyph_pixel(letter, sample_x, sample_y)
    })
}

fn glyph_pixel(letter: char, x: f64, y: f64) -> bool {
    if !(GLYPH_BOUNDS.min_x..=GLYPH_BOUNDS.max_x).contains(&x)
        || !(GLYPH_BOUNDS.min_y..=GLYPH_BOUNDS.max_y).contains(&y)
    {
        return false;
    }

    let width = GLYPH_BOUNDS.max_x - GLYPH_BOUNDS.min_x;
    let height = GLYPH_BOUNDS.max_y - GLYPH_BOUNDS.min_y;
    let local_x = ((x - GLYPH_BOUNDS.min_x) / width).clamp(0.0, 0.999_999);
    let local_y = ((y - GLYPH_BOUNDS.min_y) / height).clamp(0.0, 0.999_999);

    let col = (local_x * GLYPH_COLUMNS as f64).floor() as usize;
    let row = (local_y * GLYPH_ROWS as f64).floor() as usize;
    let pattern = glyph_pattern(letter);

    pattern[row].as_bytes()[col] == b'#'
}

fn glyph_pattern(letter: char) -> [&'static str; GLYPH_ROWS] {
    match letter.to_ascii_uppercase() {
        'A' => [
            ".###.", "#...#", "#...#", "#####", "#...#", "#...#", "#...#",
        ],
        'B' => [
            "####.", "#...#", "#...#", "####.", "#...#", "#...#", "####.",
        ],
        'C' => [
            ".###.", "#...#", "#....", "#....", "#....", "#...#", ".###.",
        ],
        'D' => [
            "####.", "#...#", "#...#", "#...#", "#...#", "#...#", "####.",
        ],
        'E' => [
            "#####", "#....", "#....", "####.", "#....", "#....", "#####",
        ],
        'F' => [
            "#####", "#....", "#....", "####.", "#....", "#....", "#....",
        ],
        'G' => [
            ".###.", "#...#", "#....", "#.###", "#...#", "#...#", ".###.",
        ],
        'H' => [
            "#...#", "#...#", "#...#", "#####", "#...#", "#...#", "#...#",
        ],
        'I' => [
            "#####", "..#..", "..#..", "..#..", "..#..", "..#..", "#####",
        ],
        'J' => [
            "..###", "....#", "....#", "....#", "#...#", "#...#", ".###.",
        ],
        'K' => [
            "#...#", "#..#.", "#.#..", "##...", "#.#..", "#..#.", "#...#",
        ],
        'L' => [
            "#....", "#....", "#....", "#....", "#....", "#....", "#####",
        ],
        'M' => [
            "#...#", "##.##", "#.#.#", "#.#.#", "#...#", "#...#", "#...#",
        ],
        'N' => [
            "#...#", "##..#", "##..#", "#.#.#", "#..##", "#..##", "#...#",
        ],
        'O' => [
            ".###.", "#...#", "#...#", "#...#", "#...#", "#...#", ".###.",
        ],
        'P' => [
            "####.", "#...#", "#...#", "####.", "#....", "#....", "#....",
        ],
        'Q' => [
            ".###.", "#...#", "#...#", "#...#", "#.#.#", "#..#.", ".##.#",
        ],
        'R' => [
            "####.", "#...#", "#...#", "####.", "#.#..", "#..#.", "#...#",
        ],
        'S' => [
            ".####", "#....", "#....", ".###.", "....#", "....#", "####.",
        ],
        'T' => [
            "#####", "..#..", "..#..", "..#..", "..#..", "..#..", "..#..",
        ],
        'U' => [
            "#...#", "#...#", "#...#", "#...#", "#...#", "#...#", ".###.",
        ],
        'V' => [
            "#...#", "#...#", "#...#", "#...#", ".#.#.", ".#.#.", "..#..",
        ],
        'W' => [
            "#...#", "#...#", "#...#", "#.#.#", "#.#.#", "##.##", "#...#",
        ],
        'X' => [
            "#...#", "#...#", ".#.#.", "..#..", ".#.#.", "#...#", "#...#",
        ],
        'Y' => [
            "#...#", "#...#", ".#.#.", "..#..", "..#..", "..#..", "..#..",
        ],
        'Z' => [
            "#####", "....#", "...#.", "..#..", ".#...", "#....", "#####",
        ],
        _ => [
            ".###.", "#...#", "#...#", "#####", "#...#", "#...#", "#...#",
        ],
    }
}

fn expand_ring(grid: &TriangularGrid, start: &HashSet<usize>, steps: usize) -> Vec<usize> {
    let mut frontier = start.clone();
    let mut visited = start.clone();
    let mut ring = HashSet::new();

    for _ in 0..steps {
        let mut next = HashSet::new();
        for cell_id in &frontier {
            for adjacent in grid.adjacent_cells(*cell_id) {
                if visited.insert(adjacent) {
                    next.insert(adjacent);
                    ring.insert(adjacent);
                }
            }
        }
        frontier = next;
    }

    let mut cells = ring.into_iter().collect::<Vec<_>>();
    cells.sort_unstable();
    cells
}

fn select_band(
    normalized: &[(usize, Point)],
    allowed: &HashSet<usize>,
    anchor_angle: f64,
    width: f64,
) -> Vec<usize> {
    let mut cells = normalized
        .iter()
        .filter(|(cell_id, point)| {
            allowed.contains(cell_id)
                && angular_distance(point_angle(*point), anchor_angle) <= width
        })
        .map(|(cell_id, _)| *cell_id)
        .collect::<Vec<_>>();
    cells.sort_unstable();
    cells
}

fn point_angle(point: Point) -> f64 {
    (0.5 - point.y)
        .atan2(point.x - 0.5)
        .to_degrees()
        .rem_euclid(360.0)
}

fn angular_distance(left: f64, right: f64) -> f64 {
    let delta = (left - right).rem_euclid(360.0);
    delta.min(360.0 - delta)
}
