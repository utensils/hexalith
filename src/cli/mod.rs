use crate::clipboard;
use crate::generator::{Generator, SymmetryMode};
use crate::png;
use crate::svg;
use crate::utils;
use crate::Result;
use clap::{Parser, ValueEnum};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Modern geometric logo generator in Rust - creates unique hexagonal designs with minimal configuration",
    long_about = None,
)]
pub struct Cli {
    /// Output file path
    #[arg(default_value = "logo.png")]
    pub output: String,

    /// Seed for deterministic generation
    #[arg(short, long)]
    pub seed: Option<u64>,

    /// UUID for deterministic generation (overrides seed)
    #[arg(short, long)]
    pub uuid: Option<String>,

    /// Color theme (ink, horizon, ember, citrus, orchid, classic, google, blues, greens, reds, purples, rainbow)
    #[arg(short = 't', long = "theme", default_value = "ink")]
    pub theme: String,

    /// Number of shapes to generate
    #[arg(short = 'n', long, default_value_t = 4)]
    pub shapes: u8,

    /// Grid density (2-8)
    #[arg(short, long, default_value_t = 4)]
    pub grid_size: u8,

    /// Shape opacity
    #[arg(short, long, default_value_t = 0.8)]
    pub opacity: f32,

    /// Output width in pixels (PNG only)
    #[arg(short, long, default_value_t = 512)]
    pub width: u32,

    /// Output height in pixels (PNG only)
    #[arg(short = 'H', long, default_value_t = 512)]
    pub height: u32,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = Format::Png)]
    pub format: Format,

    /// Allow shapes to overlap with blended colors
    #[arg(long, default_value_t = true)]
    pub overlap: bool,

    /// Generate unique shapes that don't necessarily grow from center
    #[arg(long, default_value_t = false)]
    pub unique_shapes: bool,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Copy to clipboard instead of saving to file
    #[arg(short, long)]
    pub copy: bool,

    /// Use classic style generation with fixed triangle layout (2 overlapping shapes)
    #[arg(long)]
    pub classic: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Format {
    Svg,
    Png,
}

impl Format {
    pub fn extension(&self) -> &'static str {
        match self {
            Format::Svg => "svg",
            Format::Png => "png",
        }
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Svg => write!(f, "svg"),
            Format::Png => write!(f, "png"),
        }
    }
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    // Process seed/UUID
    let seed = match &cli.uuid {
        Some(uuid) => Some(utils::uuid_to_seed(uuid)?),
        None => cli.seed,
    };

    // Set up the generator
    // For classic mode, default to grid size 2 to match original (24 triangles)
    let grid_size = if cli.classic && cli.grid_size == 4 {
        2 // Default to 2 for classic mode to match original
    } else {
        cli.grid_size
    };

    let mut generator = Generator::new(grid_size, cli.shapes, cli.opacity, seed);
    generator
        .set_color_scheme(&cli.theme)
        .set_allow_overlap(cli.overlap)
        .set_unique_shapes(cli.unique_shapes)
        .set_classic_mode(cli.classic);

    // Generate the logo
    generator.generate()?;

    // Handle clipboard copy or file save
    if cli.copy {
        // Copy to clipboard without saving to file
        match cli.format {
            Format::Svg => {
                let svg_data = svg::generate_svg(&generator, cli.width, cli.height)?;
                clipboard::copy_svg_to_clipboard(&svg_data)?;
                if cli.verbose {
                    println!("Logo copied to clipboard as SVG text");
                }
            }
            Format::Png => {
                let png_data = png::generate_png(&generator, cli.width, cli.height)?;
                clipboard::copy_png_to_clipboard(&png_data)?;
                if cli.verbose {
                    println!("Logo copied to clipboard as PNG image");
                }
            }
        }
    } else {
        // Save to file as before
        // Make sure the output path has the correct extension
        let mut output_path = PathBuf::from(&cli.output);
        if let Some(ext) = output_path.extension().and_then(|e| e.to_str()) {
            if ext != cli.format.extension() {
                if cli.verbose {
                    println!(
                        "Warning: Changing extension from .{ext} to .{}",
                        cli.format.extension()
                    );
                }
                output_path.set_extension(cli.format.extension());
            }
        } else {
            output_path.set_extension(cli.format.extension());
        }

        // Generate and save the output
        if let Some(parent) = output_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        match cli.format {
            Format::Svg => {
                let svg_data = svg::generate_svg(&generator, cli.width, cli.height)?;
                svg::save_svg(&svg_data, &output_path)?;
            }
            Format::Png => {
                let png_data = png::generate_png(&generator, cli.width, cli.height)?;
                png::save_png(&png_data, &output_path)?;
            }
        }

        if cli.verbose {
            let seed_info = match &cli.uuid {
                Some(uuid) => format!("UUID: {uuid}"),
                None => match seed {
                    Some(s) => format!("Seed: {s}"),
                    None => "Random generation (no seed)".to_string(),
                },
            };

            println!("Logo generated successfully:");
            println!("  Output: {}", output_path.display());
            println!("  Format: {}", cli.format);
            println!("  Theme: {}", cli.theme);
            if cli.classic {
                println!("  Mode: Classic (24 fixed triangles)");
                println!("  Shapes: 2 overlapping");
            } else {
                println!("  Grid size: {grid_size}");
                println!("  Shapes: {}", cli.shapes);
                println!(
                    "  Symmetry: {}",
                    match generator.symmetry_mode() {
                        SymmetryMode::None => "none",
                        SymmetryMode::Emblem => "emblem (bilateral / triadic / restrained hex)",
                    }
                );
            }
            println!("  Opacity: {}", cli.opacity);
            println!(
                "  Overlap: {}",
                if cli.overlap { "enabled" } else { "disabled" }
            );
            if !cli.classic {
                println!(
                    "  Unique shapes: {}",
                    if cli.unique_shapes {
                        "enabled"
                    } else {
                        "disabled"
                    }
                );
            }
            println!("  {seed_info}");
        }
    }

    Ok(())
}
