# Hexalith

Modern geometric logo generator in Rust - creates unique hexagonal designs with minimal configuration.

## Overview

Hexalith (HexLogoGen) is a command-line tool that generates random, geometric logos based on triangles within a hexagonal grid. The logos are visually distinctive, minimal, and ideal for open-source projects, microservices, or any application that needs a clean, modern identity.

## Features

- **Hexagonal Grid Generation**: Creates a hexagonal grid divided into equilateral triangular cells
- **Shape Creation**: Generates polygon shapes made up of connected triangles that grow from the center outward
- **Equiangular Triangles**: All triangles have equal 60-degree angles for clean, balanced designs
- **Connected Edges**: All triangles share edges with adjacent triangles for cohesive patterns
- **Center-Out Growth**: All shapes grow from the center outward for balanced, harmonious designs
- **Angular Style**: Default mode creates logos similar to the original hexagonal logo generator
- **Shape Overlapping**: By default, shapes overlap with color blending at intersections for richer designs
- **Multiple Color Themes**: Choose from various color themes including Mesos (default), Google, Blues, Greens, Reds, Purples, and Rainbow
- **SVG Output**: Generates clean, optimized SVG files
- **PNG Output**: Supports PNG generation with transparency
- **Deterministic Mode**: Generate the same logo consistently with the same seed or UUID
- **Customization Options**: Customize themes, opacity, and grid density

## Installation

### Using Cargo

```bash
cargo install hexlogogen
```

### Using Nix Flakes

```bash
nix run github:utensils/hexalith
```

## Usage

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
  --overlap                    Allow shapes to overlap with blended colors [default: true]
  -w, --width <WIDTH>          Output width in pixels (PNG only) [default: 512]
  -H, --height <HEIGHT>        Output height in pixels (PNG only) [default: 512]
  -f, --format <FORMAT>        Output format [default: svg] [possible values: svg, png]
  -v, --verbose                Enable verbose output
  -h, --help                   Print help
  -V, --version                Print version
```

## Examples

Generate a random logo in SVG format:
```bash
hexlogogen my-logo.svg
```

Generate a deterministic logo using a seed:
```bash
hexlogogen --seed 12345 my-logo.svg
```

Generate a PNG with custom dimensions:
```bash
hexlogogen --format png --width 800 --height 800 my-logo.png
```

Generate a logo with custom parameters:
```bash
hexlogogen --grid-size 8 --shapes 5 --opacity 0.7 --verbose logo.svg
```

Generate a logo with a specific color theme:
```bash
hexlogogen --theme blues --seed 42 logo.svg
```

Generate a logo without overlapping shapes:
```bash
hexlogogen --no-overlap --seed 42 logo.svg
```

Try different color themes:
```bash
hexlogogen --theme google logo_google.svg
hexlogogen --theme rainbow logo_rainbow.svg
hexlogogen --theme greens logo_green.svg
hexlogogen --theme purples logo_purple.svg
```

Use a UUID for deterministic generation:
```bash
hexlogogen --uuid f47ac10b-58cc-4372-a567-0e02b2c3d479 logo.svg
```

## Development

### Prerequisites

- Rust 1.60 or later
- Nix with flakes enabled (optional)

### Setup with Nix

If you have Nix with flakes enabled and direnv installed:

```bash
# Clone the repository
git clone https://github.com/utensils/hexalith.git
cd hexalith

# Allow direnv
direnv allow

# Build the project
cargo build
```

### Setup without Nix

```bash
# Clone the repository
git clone https://github.com/utensils/hexalith.git
cd hexalith

# Build the project
cargo build
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific tests
cargo test grid  # Run grid-related tests
cargo test svg   # Run SVG output tests
```

### Code Coverage

If you have Nix with flakes enabled:

```bash
# Generate code coverage report
nix develop -c rs-coverage

# The HTML report will open in your browser
# Or you can find it at ./target/coverage/html/index.html
```

For non-Nix users, you can generate coverage using grcov directly:

```bash
# Install necessary tools
rustup component add llvm-tools-preview
cargo install grcov

# Set up environment variables
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Cinstrument-coverage -Ccodegen-units=1 -Copt-level=0"
export LLVM_PROFILE_FILE="hexalith-%p-%m.profraw"

# Run tests
cargo test

# Generate coverage report
grcov . --binary-path ./target/debug/deps/ \
  --source-dir . \
  --output-path ./target/coverage/ \
  --output-type html \
  --branch \
  --ignore "target/*" \
  --ignore "tests/*" \
  --ignore-not-existing \
  --llvm
```

> **Note**: The current coverage report provides reliable metrics for line coverage but has limitations with branch coverage measurement. This is a known issue with LLVM instrumentation in stable Rust. For more accurate branch coverage, a specialized setup with nightly Rust and additional tools may be required in the future.

## Project Structure

- `src/generator/grid/`: Hexagonal grid geometry and triangular subdivision
- `src/generator/shape/`: Shape generation algorithms
- `src/generator/color/`: Color management and blending
- `src/svg/`: SVG output generation
- `src/png/`: PNG conversion from SVG
- `src/cli/`: Command line interface handling

## License

This project is licensed under the MIT License - see the LICENSE file for details.