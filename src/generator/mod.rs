pub mod classic;
mod color;
pub mod grid;
pub mod shape;

use crate::Result;
use classic::ClassicGenerator;
use color::ColorManager;
use grid::{Point, TriangularGrid};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use shape::Shape;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub use color::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymmetryMode {
    None,
    Rotational6,
}

pub struct Generator {
    grid_size: u8,
    shapes_count: u8,
    opacity: f32,
    seed: Option<u64>,
    grid: Option<TriangularGrid>,
    shapes: Vec<Shape>,
    theme: Theme,
    allow_overlap: bool,
    unique_shapes: bool,
    classic_mode: bool,
    classic_generator: Option<ClassicGenerator>,
    symmetry_mode: SymmetryMode,
}

#[derive(Debug, Clone)]
struct SymmetryOrbit {
    cells: Vec<usize>,
    radius: f64,
    sector_angle: f64,
}

#[derive(Debug, Clone, Copy)]
enum MotifKind {
    Core,
    Petals,
    Halo,
    Ring,
    Crown,
    Spokes,
}

#[derive(Debug, Clone, Copy)]
struct Motif {
    kind: MotifKind,
    min_radius: f64,
    max_radius: f64,
    anchor: f64,
    width: f64,
}

impl Generator {
    pub fn new(grid_size: u8, shapes_count: u8, opacity: f32, seed: Option<u64>) -> Self {
        Self {
            grid_size: grid_size.clamp(2, 8),
            shapes_count: shapes_count.clamp(1, 10),
            opacity: opacity.clamp(0.0, 1.0),
            seed,
            grid: None,
            shapes: Vec::new(),
            theme: Theme::Classic,
            allow_overlap: false,
            unique_shapes: false,
            classic_mode: false,
            classic_generator: None,
            symmetry_mode: SymmetryMode::Rotational6,
        }
    }

    pub fn set_theme(&mut self, theme: Theme) -> &mut Self {
        self.theme = theme;
        self
    }

    pub fn set_color_scheme(&mut self, color_scheme: &str) -> &mut Self {
        self.theme = Theme::from(color_scheme);
        self
    }

    pub fn available_themes() -> Vec<String> {
        ColorManager::available_themes()
    }

    pub fn set_allow_overlap(&mut self, allow_overlap: bool) -> &mut Self {
        self.allow_overlap = allow_overlap;
        self
    }

    pub fn set_unique_shapes(&mut self, unique_shapes: bool) -> &mut Self {
        self.unique_shapes = unique_shapes;
        self
    }

    pub fn set_classic_mode(&mut self, classic_mode: bool) -> &mut Self {
        self.classic_mode = classic_mode;
        if classic_mode {
            self.symmetry_mode = SymmetryMode::None;
            self.allow_overlap = true;
            self.shapes_count = 2;
        } else {
            self.symmetry_mode = SymmetryMode::Rotational6;
        }
        self
    }

    pub fn generate(&mut self) -> Result<()> {
        self.shapes.clear();
        self.classic_generator = None;

        if self.classic_mode {
            let mut classic_gen = ClassicGenerator::new(self.seed);
            self.shapes = classic_gen.generate_classic_shapes(self.theme, self.opacity);
            self.classic_generator = Some(classic_gen);
            return Ok(());
        }

        let grid = TriangularGrid::new(100.0, self.grid_size);
        self.grid = Some(grid);
        self.generate_symmetric_modern()
    }

    fn generate_symmetric_modern(&mut self) -> Result<()> {
        let grid = self.grid.as_ref().ok_or("Grid not initialized")?;
        let orbits = build_rotational_orbits(grid);

        if orbits.is_empty() {
            return Ok(());
        }

        let max_radius = orbits
            .iter()
            .map(|orbit| orbit.radius)
            .fold(0.0_f64, f64::max)
            .max(1.0);

        let mut rng = match self.seed {
            Some(seed) => ChaCha8Rng::seed_from_u64(seed ^ 0xA5A5_A5A5_A5A5_A5A5),
            None => ChaCha8Rng::from_entropy(),
        };
        let mut color_manager = ColorManager::with_theme(self.theme, self.seed);

        if self.allow_overlap && self.shapes_count >= 2 {
            self.generate_overlap_shapes(&orbits, max_radius, &mut color_manager, &mut rng);
        } else {
            self.generate_non_overlapping_shapes(&orbits, max_radius, &mut color_manager, &mut rng);
        }

        Ok(())
    }

    fn generate_overlap_shapes(
        &mut self,
        orbits: &[SymmetryOrbit],
        max_radius: f64,
        color_manager: &mut ColorManager,
        rng: &mut ChaCha8Rng,
    ) {
        let shared_cutoff = if self.grid_size <= 3 { 0.16 } else { 0.20 };
        let shared_core = select_matching_orbits(
            orbits,
            max_radius,
            &Motif {
                kind: MotifKind::Core,
                min_radius: 0.0,
                max_radius: shared_cutoff,
                anchor: 30.0,
                width: 30.0,
            },
            &HashSet::new(),
        );

        let left_anchor = 12.0 + rng.gen_range(0.0..10.0);
        let right_anchor = 60.0 - left_anchor;
        let colors = color_manager.get_random_colors(self.shapes_count.max(2) as usize + 2);
        let primary = colors[0].clone();
        let secondary = pick_contrasting_color(&primary, &colors);

        let motif_a = build_motif(1, left_anchor, self.unique_shapes);
        let motif_b = build_motif(2, right_anchor, self.unique_shapes);

        let shape_a_orbits = union_orbits(
            &shared_core,
            &select_matching_orbits(orbits, max_radius, &motif_a, &HashSet::new()),
        );
        let shape_b_orbits = union_orbits(
            &shared_core,
            &select_matching_orbits(orbits, max_radius, &motif_b, &HashSet::new()),
        );

        let shape_a_cells = collect_cells(orbits, &shape_a_orbits);
        let shape_b_cells = collect_cells(orbits, &shape_b_orbits);
        let overlap_cells = intersection_cells(&shape_a_cells, &shape_b_cells);

        let mut used_cells = HashSet::new();

        let shape_a_only = subtract_cells(&shape_a_cells, &overlap_cells);
        if !shape_a_only.is_empty() {
            self.shapes.push(build_shape(
                primary.clone(),
                self.opacity,
                shape_a_only.clone(),
            ));
            used_cells.extend(shape_a_only);
        }

        let shape_b_only = subtract_cells(&shape_b_cells, &overlap_cells);
        if !shape_b_only.is_empty() {
            self.shapes.push(build_shape(
                secondary.clone(),
                self.opacity,
                shape_b_only.clone(),
            ));
            used_cells.extend(shape_b_only);
        }

        if !overlap_cells.is_empty() {
            self.shapes.push(build_shape(
                blend_colors(&primary, &secondary),
                self.opacity,
                overlap_cells.clone(),
            ));
            used_cells.extend(overlap_cells);
        }

        let mut existing_colors = vec![primary, secondary];
        while self.shapes.len() < self.shapes_count as usize {
            let shape_index = self.shapes.len() + 1;
            let motif = build_motif(
                shape_index,
                20.0 + rng.gen_range(0.0..20.0),
                self.unique_shapes,
            );
            let orbit_ids =
                select_matching_orbits_from_cells(orbits, max_radius, &motif, &used_cells);
            if orbit_ids.is_empty() {
                break;
            }

            let cells = collect_cells(orbits, &orbit_ids);
            let color = color_manager.get_different_color(&existing_colors);
            existing_colors.push(color.clone());
            used_cells.extend(cells.iter().copied());
            self.shapes.push(build_shape(color, self.opacity, cells));
        }

        if self.shapes.is_empty() {
            self.generate_non_overlapping_shapes(orbits, max_radius, color_manager, rng);
        }
    }

    fn generate_non_overlapping_shapes(
        &mut self,
        orbits: &[SymmetryOrbit],
        max_radius: f64,
        color_manager: &mut ColorManager,
        rng: &mut ChaCha8Rng,
    ) {
        let mut used_cells = HashSet::new();
        let mut existing_colors = Vec::new();

        for shape_index in 0..self.shapes_count as usize {
            let anchor = if shape_index % 2 == 0 {
                14.0 + rng.gen_range(0.0..14.0)
            } else {
                38.0 + rng.gen_range(0.0..10.0)
            };
            let motif = build_motif(shape_index, anchor, self.unique_shapes);
            let orbit_ids =
                select_matching_orbits_from_cells(orbits, max_radius, &motif, &used_cells);
            if orbit_ids.is_empty() {
                continue;
            }

            let cells = collect_cells(orbits, &orbit_ids);
            let color = color_manager.get_different_color(&existing_colors);
            existing_colors.push(color.clone());
            used_cells.extend(cells.iter().copied());
            self.shapes.push(build_shape(color, self.opacity, cells));
        }

        if self.shapes.is_empty() {
            let fallback_cells = collect_cells(orbits, &[0]);
            self.shapes.push(build_shape(
                color_manager.get_random_color(),
                self.opacity,
                fallback_cells,
            ));
        }
    }

    pub fn grid(&self) -> Option<&TriangularGrid> {
        self.grid.as_ref()
    }

    pub fn shapes(&self) -> &[Shape] {
        &self.shapes
    }

    pub fn classic_generator(&self) -> Option<&ClassicGenerator> {
        self.classic_generator.as_ref()
    }

    pub fn symmetry_mode(&self) -> SymmetryMode {
        self.symmetry_mode
    }
}

fn build_motif(index: usize, anchor: f64, unique_shapes: bool) -> Motif {
    let cycle = index % 6;
    let width_adjustment = if unique_shapes { 2.0 } else { 0.0 };

    match cycle {
        0 => Motif {
            kind: MotifKind::Core,
            min_radius: 0.0,
            max_radius: 0.22,
            anchor: 30.0,
            width: 30.0,
        },
        1 => Motif {
            kind: MotifKind::Petals,
            min_radius: 0.16,
            max_radius: 0.55,
            anchor,
            width: 8.0 + width_adjustment,
        },
        2 => Motif {
            kind: MotifKind::Halo,
            min_radius: 0.38,
            max_radius: 0.80,
            anchor: 30.0,
            width: 18.0 + width_adjustment,
        },
        3 => Motif {
            kind: MotifKind::Ring,
            min_radius: 0.48,
            max_radius: 0.68,
            anchor: 30.0,
            width: 30.0,
        },
        4 => Motif {
            kind: MotifKind::Crown,
            min_radius: 0.68,
            max_radius: 1.0,
            anchor,
            width: 12.0 + width_adjustment,
        },
        _ => Motif {
            kind: MotifKind::Spokes,
            min_radius: 0.24,
            max_radius: 0.92,
            anchor: 0.0,
            width: 9.0 + width_adjustment,
        },
    }
}

fn build_rotational_orbits(grid: &TriangularGrid) -> Vec<SymmetryOrbit> {
    let center = grid.hex_grid().center;
    let lookup: HashMap<(i64, i64), usize> = grid
        .cells()
        .iter()
        .map(|cell| (point_key(cell.centroid), cell.id))
        .collect();

    let mut visited = HashSet::new();
    let mut orbits = Vec::new();

    for cell in grid.cells() {
        if visited.contains(&cell.id) {
            continue;
        }

        let mut cells = HashSet::new();
        for turns in 0..6 {
            let rotated = rotate_point(cell.centroid, turns, center);
            if let Some(cell_id) = lookup_cell_id(grid, &lookup, rotated) {
                cells.insert(cell_id);
            }
        }

        let mut cells: Vec<usize> = cells.into_iter().collect();
        cells.sort_unstable();
        visited.extend(cells.iter().copied());

        let angle = normalize_sector_angle(cell.centroid, center);
        orbits.push(SymmetryOrbit {
            radius: cell.centroid.distance(&center),
            sector_angle: angle,
            cells,
        });
    }

    orbits.sort_by(|a, b| {
        a.radius
            .partial_cmp(&b.radius)
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                a.sector_angle
                    .partial_cmp(&b.sector_angle)
                    .unwrap_or(Ordering::Equal)
            })
    });

    orbits
}

fn lookup_cell_id(
    grid: &TriangularGrid,
    lookup: &HashMap<(i64, i64), usize>,
    point: Point,
) -> Option<usize> {
    if let Some(cell_id) = lookup.get(&point_key(point)) {
        return Some(*cell_id);
    }

    grid.cells()
        .iter()
        .find(|cell| cell.centroid.distance(&point) < 1e-4)
        .map(|cell| cell.id)
}

fn point_key(point: Point) -> (i64, i64) {
    (
        (point.x * 100_000.0).round() as i64,
        (point.y * 100_000.0).round() as i64,
    )
}

fn rotate_point(point: Point, turns: usize, center: Point) -> Point {
    let angle = turns as f64 * std::f64::consts::PI / 3.0;
    let translated_x = point.x - center.x;
    let translated_y = point.y - center.y;

    Point::new(
        center.x + translated_x * angle.cos() - translated_y * angle.sin(),
        center.y + translated_x * angle.sin() + translated_y * angle.cos(),
    )
}

fn normalize_sector_angle(point: Point, center: Point) -> f64 {
    let angle = (point.y - center.y).atan2(point.x - center.x).to_degrees();
    angle.rem_euclid(60.0)
}

fn select_matching_orbits(
    orbits: &[SymmetryOrbit],
    max_radius: f64,
    motif: &Motif,
    excluded_orbits: &HashSet<usize>,
) -> Vec<usize> {
    let mut matches = Vec::new();

    for (index, orbit) in orbits.iter().enumerate() {
        if excluded_orbits.contains(&index) {
            continue;
        }

        if orbit_matches(orbit, max_radius, motif) {
            matches.push(index);
        }
    }

    if matches.is_empty() {
        fallback_orbit(orbits, max_radius, motif, excluded_orbits)
            .into_iter()
            .collect()
    } else {
        matches
    }
}

fn select_matching_orbits_from_cells(
    orbits: &[SymmetryOrbit],
    max_radius: f64,
    motif: &Motif,
    used_cells: &HashSet<usize>,
) -> Vec<usize> {
    let mut matches = Vec::new();

    for (index, orbit) in orbits.iter().enumerate() {
        if orbit.cells.iter().any(|cell| used_cells.contains(cell)) {
            continue;
        }

        if orbit_matches(orbit, max_radius, motif) {
            matches.push(index);
        }
    }

    if matches.is_empty() {
        fallback_orbit_from_cells(orbits, max_radius, motif, used_cells)
            .into_iter()
            .collect()
    } else {
        matches
    }
}

fn orbit_matches(orbit: &SymmetryOrbit, max_radius: f64, motif: &Motif) -> bool {
    let normalized_radius = orbit.radius / max_radius;
    if normalized_radius < motif.min_radius || normalized_radius > motif.max_radius {
        return false;
    }

    let angle = orbit.sector_angle;
    let centered_distance = (angle - 30.0).abs();
    let edge_distance = angle.min(60.0 - angle);
    let anchor_distance = (angle - motif.anchor).abs();

    match motif.kind {
        MotifKind::Core | MotifKind::Ring => true,
        MotifKind::Petals | MotifKind::Crown => anchor_distance <= motif.width,
        MotifKind::Halo => centered_distance <= motif.width,
        MotifKind::Spokes => edge_distance <= motif.width,
    }
}

fn fallback_orbit(
    orbits: &[SymmetryOrbit],
    max_radius: f64,
    motif: &Motif,
    excluded_orbits: &HashSet<usize>,
) -> Option<usize> {
    let target_radius = (motif.min_radius + motif.max_radius) * 0.5;
    orbits
        .iter()
        .enumerate()
        .filter(|(index, _)| !excluded_orbits.contains(index))
        .min_by(|(_, left), (_, right)| {
            let left_score = ((left.radius / max_radius) - target_radius).abs();
            let right_score = ((right.radius / max_radius) - target_radius).abs();
            left_score
                .partial_cmp(&right_score)
                .unwrap_or(Ordering::Equal)
        })
        .map(|(index, _)| index)
}

fn fallback_orbit_from_cells(
    orbits: &[SymmetryOrbit],
    max_radius: f64,
    motif: &Motif,
    used_cells: &HashSet<usize>,
) -> Option<usize> {
    let target_radius = (motif.min_radius + motif.max_radius) * 0.5;
    orbits
        .iter()
        .enumerate()
        .filter(|(_, orbit)| !orbit.cells.iter().any(|cell| used_cells.contains(cell)))
        .min_by(|(_, left), (_, right)| {
            let left_score = ((left.radius / max_radius) - target_radius).abs();
            let right_score = ((right.radius / max_radius) - target_radius).abs();
            left_score
                .partial_cmp(&right_score)
                .unwrap_or(Ordering::Equal)
        })
        .map(|(index, _)| index)
}

fn union_orbits(left: &[usize], right: &[usize]) -> Vec<usize> {
    let mut merged = left.to_vec();
    for orbit_id in right {
        if !merged.contains(orbit_id) {
            merged.push(*orbit_id);
        }
    }
    merged.sort_unstable();
    merged
}

fn collect_cells(orbits: &[SymmetryOrbit], orbit_ids: &[usize]) -> Vec<usize> {
    let mut cells = Vec::new();
    for orbit_id in orbit_ids {
        if let Some(orbit) = orbits.get(*orbit_id) {
            for cell_id in &orbit.cells {
                if !cells.contains(cell_id) {
                    cells.push(*cell_id);
                }
            }
        }
    }
    cells.sort_unstable();
    cells
}

fn intersection_cells(left: &[usize], right: &[usize]) -> Vec<usize> {
    left.iter()
        .copied()
        .filter(|cell| right.contains(cell))
        .collect()
}

fn subtract_cells(source: &[usize], to_remove: &[usize]) -> Vec<usize> {
    source
        .iter()
        .copied()
        .filter(|cell| !to_remove.contains(cell))
        .collect()
}

fn build_shape(color: String, opacity: f32, cells: Vec<usize>) -> Shape {
    let mut shape = Shape::new(color, opacity);
    for cell in cells {
        shape.add_cell(cell);
    }
    shape
}

fn pick_contrasting_color(base: &str, candidates: &[String]) -> String {
    candidates
        .iter()
        .filter(|candidate| candidate.as_str() != base)
        .max_by(|left, right| {
            let left_score = ColorManager::color_contrast(base, left);
            let right_score = ColorManager::color_contrast(base, right);
            left_score
                .partial_cmp(&right_score)
                .unwrap_or(Ordering::Equal)
        })
        .cloned()
        .unwrap_or_else(|| base.to_string())
}

fn blend_colors(left: &str, right: &str) -> String {
    let (r1, g1, b1) = ColorManager::hex_to_rgb(left);
    let (r2, g2, b2) = ColorManager::hex_to_rgb(right);

    ColorManager::rgb_to_hex(
        ((r1 as u16 + r2 as u16) / 2) as u8,
        ((g1 as u16 + g2 as u16) / 2) as u8,
        ((b1 as u16 + b2 as u16) / 2) as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::svg;

    #[test]
    fn modern_generation_is_seed_stable() {
        let mut first = Generator::new(4, 4, 0.8, Some(42));
        first.set_allow_overlap(true);
        first.generate().unwrap();

        let mut second = Generator::new(4, 4, 0.8, Some(42));
        second.set_allow_overlap(true);
        second.generate().unwrap();

        let first_svg = svg::generate_svg(&first, 512, 512).unwrap();
        let second_svg = svg::generate_svg(&second, 512, 512).unwrap();

        assert_eq!(first_svg, second_svg);
    }

    #[test]
    fn modern_shapes_follow_complete_rotational_orbits() {
        let mut generator = Generator::new(5, 4, 0.8, Some(7));
        generator.set_allow_overlap(true);
        generator.generate().unwrap();

        let grid = generator.grid().unwrap();
        let orbits = build_rotational_orbits(grid);

        for shape in generator.shapes() {
            for orbit in &orbits {
                let matches = orbit
                    .cells
                    .iter()
                    .filter(|cell_id| shape.cells.contains(cell_id))
                    .count();

                assert!(matches == 0 || matches == orbit.cells.len());
            }
        }
    }
}
