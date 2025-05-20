# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Hexalith (HexLogoGen) is a modern geometric logo generator written in Rust. It creates unique hexagonal designs with minimal configuration, inspired by the Mesos Framework Logo Generator style. The tool generates random, geometric logos based on equilateral triangles within a hexagonal grid.

### Key Features

- **Hexagonal Grid**: Creates a balanced, symmetric foundation for the logo
- **Equilateral Triangles**: All triangles have equal 60-degree angles for clean designs
- **Center-Out Growth**: Shapes grow from the center outward for balanced compositions
- **Multiple Color Themes**: Choose from Mesos, Google, Blues, Greens, Reds, Purples, or Rainbow themes
- **Overlapping Shapes**: By default, shapes overlap with color blending for rich designs
- **Enhanced Randomization**: Even with the same seed, designs vary slightly between generations
- **SVG and PNG Output**: Export in vector or raster formats as needed
- **Customizable Parameters**: Control grid density, shape count, opacity, and more

## Project Structure

```
hexalith/
├── src/
│   ├── cli/              # Command-line interface handling
│   ├── generator/        # Core logic for logo generation
│   │   ├── grid/         # Hexagonal grid geometry
│   │   ├── shape/        # Shape generation algorithms  
│   │   └── color/        # Color management
│   ├── svg/              # SVG output generation
│   ├── png/              # PNG conversion from SVG
│   ├── utils/            # Helper functions
│   ├── lib.rs            # Library exports
│   └── main.rs           # Application entry point
├── tests/                # Integration tests
└── examples/             # Example usages
```

## Architecture

This project follows a modular architecture with the following components:

1. **CLI Module**: Handles user input, argument parsing using Clap, and help documentation
   - Processes command-line arguments like `--theme`, `--seed`, `--grid-size`, etc.
   - Converts user input to the appropriate internal formats
   - Provides helpful error messages and usage information

2. **Generator Module**: Core logic for creating logos
   - **Hexagon/Grid Generator**: Creates the base grid with triangular cells
     - Divides the hexagon into equilateral triangular cells
     - Ensures all triangles have equal 60-degree angles
     - Provides methods to navigate and query the grid structure
   
   - **Shape Generator**: Creates random polygon shapes
     - Generates shapes that grow from the center outward
     - Ensures all triangles have connecting edges
     - Creates angular patterns similar to the original generator
     - Handles shape overlapping and intersection detection
   
   - **Color Manager**: Handles color selection and blending
     - Implements multiple color themes (Mesos, Google, Blues, etc.)
     - Provides random color selection from the chosen theme
     - Handles color blending for overlapping shapes
     - Supports opacity settings for transparency effects

3. **SVG Module**: Converts generated shapes to SVG format
   - Creates clean, optimized SVG markup
   - Translates internal shape representations to SVG paths
   - Handles appropriate viewBox and sizing attributes

4. **PNG Module**: Converts SVG to PNG when needed
   - Renders SVG to bitmap format at specified dimensions
   - Maintains transparency and proper scaling

5. **Utilities Module**: Helper functions, random number generation, etc.
   - Provides consistent random number generation with seeding
   - Handles UUID parsing for deterministic generation
   - Offers various helper functions for the application

6. **Library API**: Public API for integration with other Rust projects
   - Exposes key functionality for use in other applications
   - Allows programmatic logo generation beyond the CLI

## Development Environment

This project uses Nix Flakes for development environment management. Always run Rust commands through the Nix environment:

```bash
# Instead of direct cargo commands, use:
nix develop -c cargo build
nix develop -c cargo test
nix develop -c cargo run

# Or enter the development shell with:
nix develop
# Then you can run cargo commands directly
```

If you have direnv installed and enabled, the Nix environment will be automatically activated when you enter the directory, and you can use cargo commands directly.

## Development Commands

```bash
# Build the project
nix develop -c cargo build
# Or use the devshell command:
rs-build

# Run the project
nix develop -c cargo run
# Or use the devshell command:
rs-run

# Run with specific options
nix develop -c cargo run -- --seed 12345 --format png logo.png
# Or use the devshell command:
rs-run --seed 12345 --format png logo.png

# Run tests
nix develop -c cargo test
# Or use the devshell command:
rs-test

# Run specific test
nix develop -c cargo test <test_name>
# Or use the devshell command:
rs-test <test_name>

# Build for release
nix develop -c cargo build --release
# Or use the devshell command:
rs-release

# Check for compilation errors without building
nix develop -c cargo check
# Or use the devshell command:
rs-check

# Format code
nix develop -c cargo fmt
# Or use the devshell command:
rs-fmt

# Run linter
nix develop -c cargo clippy
# Or use the devshell command (uses direct binary path to clippy):
rs-clippy
```

## CLI Interface

The command-line interface includes these options:

```
Usage: hexlogogen [OPTIONS] [OUTPUT]

Arguments:
  [OUTPUT]  Output file path [default: logo.svg]

Options:
  -s, --seed <SEED>            Seed for deterministic generation
  -u, --uuid <UUID>            UUID for deterministic generation (overrides seed)
  -t, --theme <THEME>          Color theme [default: mesos] [possible values: mesos, google, blues, greens, reds, purples, rainbow]
  -n, --shapes <SHAPES>        Number of shapes to generate [default: 3]
  -g, --grid-size <GRID_SIZE>  Grid density (2-8) [default: 2]
  -o, --opacity <OPACITY>      Shape opacity [default: 0.8]
  --overlap                  Allow shapes to overlap with blended colors [default: true]
  -w, --width <WIDTH>          Output width in pixels (PNG only) [default: 512]
  -H, --height <HEIGHT>        Output height in pixels (PNG only) [default: 512]
  -f, --format <FORMAT>        Output format [default: svg] [possible values: svg, png]
  -v, --verbose                Enable verbose output
  -h, --help                   Print help
  -V, --version                Print version
```

## Dependencies

The project relies on these Rust crates:

- **Clap**: Command-line argument parsing
- **uuid**: UUID generation and handling
- **rand** and **rand_chacha**: Random number generation
- **svg**: SVG string generation
- **tiny-skia** and **resvg/usvg**: For PNG output

## Technical Implementation Notes

### Grid and Shape System

- The hexagonal grid is divided into equilateral triangular cells
- All triangles have equal 60-degree angles (60-60-60) for balanced designs
- All triangles share edges with adjacent triangles for cohesive patterns
- All shapes grow from the center outward for harmonious designs
- For grid density `n`, the hexagon is divided into `6n²` triangular cells
- Grid density values range from 2 to 8 (default: 2)
- With grid density=2, the generator creates a grid with exactly 24 triangles like the original
- Logos are generated by creating multiple shapes made of connected triangular cells
- By default, shapes overlap with color blending at intersections for richer designs

### Color Themes

The generator supports multiple color themes:

1. **Mesos** (default): Original colors from the Mesos Framework Logo Generator
   - Includes yellows, oranges, reds, pinks, purples, blues, and greens
   - Used for the nostalgic, original look

2. **Google**: Google brand colors and variations
   - Blue, red, yellow, and green color families
   - Perfect for Google-themed projects

3. **Blues**: Various shades of blue
   - From deep navy to light sky blue
   - Creates a calming, corporate aesthetic

4. **Greens**: Various shades of green
   - From deep forest green to light mint
   - Great for environmental or health-related projects

5. **Reds**: Various shades of red and orange
   - From deep crimson to light orange
   - Creates energetic, vibrant designs

6. **Purples**: Various shades of purple and pink
   - From deep violet to light lavender
   - Ideal for creative or luxury branding

7. **Rainbow**: Full spectrum of colors
   - Covers the entire rainbow with vibrant hues
   - Perfect for colorful, playful designs

### Other Features

- Shapes can have different colors with configurable opacity
- Color blending happens at shape intersections when overlap is enabled
- The tool supports both random and deterministic (seeded) generation
- Seed-based generation provides semi-reproducible results with added randomness
- Each run includes some time-based randomization for more variation in the designs
- UUID can be used for deterministic generation
- SVG output is crisp and scalable
- PNG output with configurable dimensions

## Code Standards

- Follow idiomatic Rust practices
- Use meaningful variable and function names
- Write unit tests for core functionality
- Implement proper error handling with the Result type
- Add documentation comments for public APIs
- Keep the code clean, modular and maintainable

## Common Tasks

### Adding a New Feature

1. Identify the module where the feature belongs
2. Implement the core functionality in the appropriate module
3. Update the CLI interface in `src/cli/mod.rs` if needed
4. Add tests for the new functionality
5. Update documentation in README.md and CLAUDE.md

### Fixing a Bug

1. Write a test that reproduces the bug
2. Fix the bug in the appropriate module
3. Ensure all tests pass
4. Document the fix in the commit message

### Adding a New Output Format

1. Create a new module in `src/` for the format
2. Implement conversion from the internal representation
3. Add the new format to the Format enum in `src/cli/mod.rs`
4. Update the CLI to handle the new format
5. Add tests for the new format

### Adding a New Color Theme

1. Open `src/generator/color/mod.rs`
2. Add a new variant to the `Theme` enum
3. Add an implementation for `std::fmt::Display` for the new theme
4. Add a case in the `From<&str>` implementation for string conversion
5. Add the theme name to the `available_themes()` method
6. Create a new function (e.g., `my_theme_theme()`) that returns a `ColorManager` with your theme colors
7. Add a case in the `with_theme()` method to use your new theme function
8. Update the CLI help text and documentation

## Using Color Themes

### Example Commands

```bash
# Generate a logo with the default Mesos theme
hexlogogen logo_mesos.svg

# Generate a logo with the Google theme
hexlogogen --theme google logo_google.svg

# Generate a logo with the Blues theme
hexlogogen --theme blues logo_blues.svg

# Generate a blue logo with more shapes
hexlogogen --theme blues --shapes 5 --grid-size 4 logo_complex_blue.svg

# Generate a rainbow logo with many shapes and high grid density
hexlogogen --theme rainbow --shapes 7 --grid-size 6 logo_rainbow.svg

# Create a deterministic logo with a specific theme
hexlogogen --theme purples --seed 12345 logo_purple.svg

# Create a PNG with the Greens theme
hexlogogen --theme greens --format png logo_green.png
```

### Theme Selection Tips

- **For corporate/business logos**: Use the Blues or Google themes
- **For environmental projects**: Use the Greens theme
- **For energetic brands**: Use the Reds theme
- **For creative projects**: Use the Purples or Rainbow themes
- **For nostalgic Mesos feel**: Use the default Mesos theme
- **For variety**: Generate multiple logos with different themes and select the best one

The theme system is designed to be easily extensible, so you can add your own custom color palettes to match your specific brand requirements.