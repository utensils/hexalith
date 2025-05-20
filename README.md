# Hexalith

Modern geometric logo generator in Rust - creates unique hexagonal designs with minimal configuration.

## Overview

Hexalith (HexLogoGen) is a command-line tool that generates random, geometric logos based on triangles within a hexagonal grid. The logos are visually distinctive, minimal, and ideal for open-source projects, microservices, or any application that needs a clean, modern identity.

## Features

- **Hexagonal Grid Generation**: Creates a hexagonal boundary divided into a triangular grid
- **Shape Creation**: Generates random polygon shapes made up of multiple triangles within the grid
- **Shape Overlapping**: Allows multiple shapes to overlap with color blending at intersections
- **SVG Output**: Generates clean, optimized SVG files
- **PNG Output**: Supports PNG generation with transparency
- **Deterministic Mode**: Generate the same logo consistently with the same seed or UUID
- **Customization Options**: Customize colors, opacity, and grid density

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
  -c, --colors <COLORS>        Color scheme [default: default]
  -n, --shapes <SHAPES>        Number of shapes to generate [default: 3]
  -g, --grid-size <GRID_SIZE>  Grid density (3-8) [default: 6]
  -o, --opacity <OPACITY>      Shape opacity [default: 0.8]
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

## Project Structure

- `src/generator/grid/`: Hexagonal grid geometry and triangular subdivision
- `src/generator/shape/`: Shape generation algorithms
- `src/generator/color/`: Color management and blending
- `src/svg/`: SVG output generation
- `src/png/`: PNG conversion from SVG
- `src/cli/`: Command line interface handling

## License

This project is licensed under the MIT License - see the LICENSE file for details.