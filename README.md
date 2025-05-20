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
USAGE:
    hexlogogen [OPTIONS] [OUTPUT]

ARGS:
    <output>    Output file path [default: logo.svg]

OPTIONS:
    -s, --seed <SEED>             Seed for deterministic generation
    -u, --uuid <UUID>             UUID for deterministic generation (overrides seed)
    -c, --colors <COLORS>         Color scheme [default: default]
    -n, --shapes <NUM>            Number of shapes to generate [default: 3]
    -g, --grid-size <SIZE>        Grid density (3-8) [default: 6]
    -o, --opacity <OPACITY>       Shape opacity [default: 0.8]
    -w, --width <WIDTH>           Output width in pixels (PNG only)
    -h, --height <HEIGHT>         Output height in pixels (PNG only)
    -f, --format <FORMAT>         Output format: svg or png [default: svg]
    -v, --verbose                 Enable verbose output
        --help                    Print help information
        --version                 Print version information
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
hexlogogen --format png --width 512 --height 512 my-logo.png
```

## Development

### Prerequisites

- Rust 1.74 or later
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

## License

This project is licensed under the MIT License - see the LICENSE file for details.