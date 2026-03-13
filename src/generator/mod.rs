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
use std::collections::HashSet;

pub use color::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymmetryMode {
    None,
    Emblem,
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

#[derive(Debug, Clone, Copy)]
struct CellProfile {
    cell_id: usize,
    radius: f64,
    angle: f64,
}

#[derive(Debug, Clone, Copy)]
enum EmblemFamily {
    Bilateral,
    Triadic,
    Hex,
}

#[derive(Debug, Clone)]
struct EmblemBlueprint {
    family: EmblemFamily,
    shared_core: LayerSpec,
    layers: Vec<LayerSpec>,
}

#[derive(Debug, Clone)]
struct LayerSpec {
    min_radius: f64,
    max_radius: f64,
    targets: Vec<f64>,
    width: f64,
    fill_band: bool,
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
            symmetry_mode: SymmetryMode::Emblem,
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
            self.symmetry_mode = SymmetryMode::Emblem;
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
        self.generate_emblem_modern()
    }

    fn generate_emblem_modern(&mut self) -> Result<()> {
        let grid = self.grid.as_ref().ok_or("Grid not initialized")?;
        let profiles = build_cell_profiles(grid);
        if profiles.is_empty() {
            return Ok(());
        }

        let mut rng = match self.seed {
            Some(seed) => ChaCha8Rng::seed_from_u64(seed ^ 0xA5A5_A5A5_A5A5_A5A5),
            None => ChaCha8Rng::from_entropy(),
        };
        let mut color_manager = ColorManager::with_theme(self.theme, self.seed);
        let blueprint = build_blueprint(&mut rng, self.unique_shapes);

        let core_cells = select_layer_cells(&profiles, &blueprint.shared_core);
        let layer_cells: Vec<Vec<usize>> = blueprint
            .layers
            .iter()
            .map(|layer| select_layer_cells(&profiles, layer))
            .filter(|cells| !cells.is_empty())
            .collect();

        if self.allow_overlap && layer_cells.len() >= 2 && self.shapes_count >= 2 {
            self.generate_overlapping_emblem(
                &core_cells,
                &layer_cells,
                &mut color_manager,
                &blueprint,
            );
        } else {
            self.generate_non_overlapping_emblem(&core_cells, &layer_cells, &mut color_manager);
        }

        if self.shapes.is_empty() {
            let fallback_cells = if !core_cells.is_empty() {
                core_cells
            } else {
                vec![profiles[0].cell_id]
            };
            self.shapes.push(build_shape(
                color_manager.get_random_color(),
                self.opacity,
                fallback_cells,
            ));
        }

        Ok(())
    }

    fn generate_overlapping_emblem(
        &mut self,
        core_cells: &[usize],
        layer_cells: &[Vec<usize>],
        color_manager: &mut ColorManager,
        blueprint: &EmblemBlueprint,
    ) {
        let colors = color_manager.get_random_colors(self.shapes_count.max(2) as usize + 2);
        let primary = colors[0].clone();
        let secondary = pick_contrasting_color(&primary, &colors);

        let shape_a_cells = union_cells(core_cells, &layer_cells[0]);
        let shape_b_cells = union_cells(core_cells, &layer_cells[1]);
        let overlap_cells = intersection_cells(&shape_a_cells, &shape_b_cells);

        let shape_a_only = subtract_cells(&shape_a_cells, &overlap_cells);
        if !shape_a_only.is_empty() {
            self.shapes.push(build_shape(
                primary.clone(),
                self.opacity,
                shape_a_only.clone(),
            ));
        }

        let shape_b_only = subtract_cells(&shape_b_cells, &overlap_cells);
        if !shape_b_only.is_empty() {
            self.shapes.push(build_shape(
                secondary.clone(),
                self.opacity,
                shape_b_only.clone(),
            ));
        }

        if !overlap_cells.is_empty() {
            self.shapes.push(build_shape(
                blend_colors(&primary, &secondary),
                self.opacity,
                overlap_cells.clone(),
            ));
        }

        let mut used_cells = union_cells(&shape_a_cells, &shape_b_cells)
            .into_iter()
            .collect::<HashSet<_>>();
        let mut existing_colors = vec![primary, secondary];

        for cells in layer_cells.iter().skip(2) {
            if self.shapes.len() >= self.shapes_count as usize {
                break;
            }

            let trimmed = subtract_used(cells, &used_cells);
            if trimmed.is_empty() {
                continue;
            }

            let accent_cells = if matches!(blueprint.family, EmblemFamily::Hex) {
                let limit = trimmed.len().min(18);
                trimmed.into_iter().take(limit).collect()
            } else {
                trimmed
            };

            if accent_cells.is_empty() {
                continue;
            }

            let color = color_manager.get_different_color(&existing_colors);
            existing_colors.push(color.clone());
            used_cells.extend(accent_cells.iter().copied());
            self.shapes
                .push(build_shape(color, self.opacity, accent_cells));
        }
    }

    fn generate_non_overlapping_emblem(
        &mut self,
        core_cells: &[usize],
        layer_cells: &[Vec<usize>],
        color_manager: &mut ColorManager,
    ) {
        let mut used_cells = HashSet::new();
        let mut existing_colors = Vec::new();

        let mut ordered_layers = Vec::new();
        if let Some(first_layer) = layer_cells.first() {
            ordered_layers.push(union_cells(core_cells, first_layer));
        } else if !core_cells.is_empty() {
            ordered_layers.push(core_cells.to_vec());
        }

        for cells in layer_cells.iter().skip(1) {
            ordered_layers.push(cells.clone());
        }

        for cells in ordered_layers {
            if self.shapes.len() >= self.shapes_count as usize {
                break;
            }

            let trimmed = subtract_used(&cells, &used_cells);
            if trimmed.is_empty() {
                continue;
            }

            let color = color_manager.get_different_color(&existing_colors);
            existing_colors.push(color.clone());
            used_cells.extend(trimmed.iter().copied());
            self.shapes.push(build_shape(color, self.opacity, trimmed));
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

fn build_cell_profiles(grid: &TriangularGrid) -> Vec<CellProfile> {
    let center = grid.hex_grid().center;
    let max_radius = grid
        .cells()
        .iter()
        .map(|cell| cell.centroid.distance(&center))
        .fold(0.0_f64, f64::max)
        .max(1.0);

    let mut profiles = grid
        .cells()
        .iter()
        .map(|cell| CellProfile {
            cell_id: cell.id,
            radius: cell.centroid.distance(&center) / max_radius,
            angle: normalize_angle(cell.centroid, center),
        })
        .collect::<Vec<_>>();

    profiles.sort_by(|left, right| {
        left.radius
            .partial_cmp(&right.radius)
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                left.angle
                    .partial_cmp(&right.angle)
                    .unwrap_or(Ordering::Equal)
            })
    });

    profiles
}

fn build_blueprint(rng: &mut ChaCha8Rng, unique_shapes: bool) -> EmblemBlueprint {
    match rng.gen_range(0..3) {
        0 => bilateral_blueprint(rng, unique_shapes),
        1 => triadic_blueprint(rng, unique_shapes),
        _ => hex_blueprint(rng, unique_shapes),
    }
}

fn bilateral_blueprint(rng: &mut ChaCha8Rng, unique_shapes: bool) -> EmblemBlueprint {
    let axis = pick_axis(rng);
    let loosen = if unique_shapes { 3.0 } else { 0.0 };

    EmblemBlueprint {
        family: EmblemFamily::Bilateral,
        shared_core: LayerSpec {
            min_radius: 0.0,
            max_radius: 0.16,
            targets: Vec::new(),
            width: 0.0,
            fill_band: true,
        },
        layers: vec![
            LayerSpec {
                min_radius: 0.12,
                max_radius: 0.46,
                targets: vec![axis - 26.0, axis + 26.0],
                width: 17.0 + loosen,
                fill_band: false,
            },
            LayerSpec {
                min_radius: 0.18,
                max_radius: 0.62,
                targets: vec![axis + 180.0],
                width: 16.0,
                fill_band: false,
            },
            LayerSpec {
                min_radius: 0.48,
                max_radius: 0.80,
                targets: vec![axis - 8.0, axis + 8.0],
                width: 9.0 + loosen * 0.5,
                fill_band: false,
            },
            LayerSpec {
                min_radius: 0.30,
                max_radius: 0.56,
                targets: vec![axis - 54.0, axis + 54.0],
                width: 10.0 + loosen,
                fill_band: false,
            },
        ],
    }
}

fn triadic_blueprint(rng: &mut ChaCha8Rng, unique_shapes: bool) -> EmblemBlueprint {
    let axis = pick_axis(rng);
    let loosen = if unique_shapes { 2.5 } else { 0.0 };

    EmblemBlueprint {
        family: EmblemFamily::Triadic,
        shared_core: LayerSpec {
            min_radius: 0.0,
            max_radius: 0.20,
            targets: Vec::new(),
            width: 0.0,
            fill_band: true,
        },
        layers: vec![
            LayerSpec {
                min_radius: 0.14,
                max_radius: 0.50,
                targets: rotated_targets(axis + 12.0, 3, 120.0),
                width: 15.0 + loosen,
                fill_band: false,
            },
            LayerSpec {
                min_radius: 0.22,
                max_radius: 0.58,
                targets: rotated_targets(axis + 72.0, 3, 120.0),
                width: 12.0 + loosen,
                fill_band: false,
            },
            LayerSpec {
                min_radius: 0.50,
                max_radius: 0.82,
                targets: rotated_targets(axis + 12.0, 3, 120.0),
                width: 7.0 + loosen * 0.5,
                fill_band: false,
            },
        ],
    }
}

fn hex_blueprint(rng: &mut ChaCha8Rng, unique_shapes: bool) -> EmblemBlueprint {
    let axis = pick_axis(rng);
    let loosen = if unique_shapes { 2.0 } else { 0.0 };

    EmblemBlueprint {
        family: EmblemFamily::Hex,
        shared_core: LayerSpec {
            min_radius: 0.0,
            max_radius: 0.22,
            targets: Vec::new(),
            width: 0.0,
            fill_band: true,
        },
        layers: vec![
            LayerSpec {
                min_radius: 0.18,
                max_radius: 0.44,
                targets: rotated_targets(axis, 6, 60.0),
                width: 8.5 + loosen,
                fill_band: false,
            },
            LayerSpec {
                min_radius: 0.28,
                max_radius: 0.60,
                targets: vec![axis, axis + 60.0, axis + 180.0, axis + 240.0],
                width: 10.0 + loosen,
                fill_band: false,
            },
            LayerSpec {
                min_radius: 0.54,
                max_radius: 0.76,
                targets: vec![axis + 30.0, axis + 150.0, axis + 270.0],
                width: 7.5 + loosen * 0.5,
                fill_band: false,
            },
        ],
    }
}

fn pick_axis(rng: &mut ChaCha8Rng) -> f64 {
    const AXES: [f64; 12] = [
        0.0, 30.0, 60.0, 90.0, 120.0, 150.0, 180.0, 210.0, 240.0, 270.0, 300.0, 330.0,
    ];
    AXES[rng.gen_range(0..AXES.len())]
}

fn rotated_targets(start: f64, repeats: usize, step: f64) -> Vec<f64> {
    (0..repeats)
        .map(|index| start + index as f64 * step)
        .collect()
}

fn select_layer_cells(profiles: &[CellProfile], layer: &LayerSpec) -> Vec<usize> {
    let mut cells = profiles
        .iter()
        .filter(|profile| {
            if profile.radius < layer.min_radius || profile.radius > layer.max_radius {
                return false;
            }

            if layer.fill_band || layer.targets.is_empty() {
                return true;
            }

            layer
                .targets
                .iter()
                .any(|target| angular_distance(profile.angle, *target) <= layer.width)
        })
        .map(|profile| profile.cell_id)
        .collect::<Vec<_>>();

    cells.sort_unstable();
    cells
}

fn normalize_angle(point: Point, center: Point) -> f64 {
    (point.y - center.y)
        .atan2(point.x - center.x)
        .to_degrees()
        .rem_euclid(360.0)
}

fn angular_distance(left: f64, right: f64) -> f64 {
    let delta = (left - right).rem_euclid(360.0);
    delta.min(360.0 - delta)
}

fn union_cells(left: &[usize], right: &[usize]) -> Vec<usize> {
    let mut merged = left.to_vec();
    for cell in right {
        if !merged.contains(cell) {
            merged.push(*cell);
        }
    }
    merged.sort_unstable();
    merged
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

fn subtract_used(source: &[usize], used_cells: &HashSet<usize>) -> Vec<usize> {
    source
        .iter()
        .copied()
        .filter(|cell| !used_cells.contains(cell))
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
    fn modern_generation_has_a_bold_center() {
        let mut generator = Generator::new(4, 4, 0.8, Some(7));
        generator.set_allow_overlap(true);
        generator.generate().unwrap();

        let grid = generator.grid().unwrap();
        let profiles = build_cell_profiles(grid);
        let center_cells = profiles
            .iter()
            .filter(|profile| profile.radius <= 0.2)
            .map(|profile| profile.cell_id)
            .collect::<Vec<_>>();

        let center_coverage = generator
            .shapes()
            .iter()
            .flat_map(|shape| shape.cells.iter())
            .filter(|cell_id| center_cells.contains(cell_id))
            .count();

        assert!(center_coverage > 0);
    }
}
