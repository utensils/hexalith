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
│   ├── web/              # Web interface implementation
│   │   ├── routes.rs     # API endpoints
│   │   └── templates.rs  # HTML templates
│   ├── utils/            # Helper functions
│   ├── lib.rs            # Library exports
│   ├── main.rs           # CLI application entry point
│   └── web_main.rs       # Web interface entry point
├── tests/                # Integration tests
└── examples/             # Example usages
```

[... rest of the existing content remains the same ...]

## Nix Configuration

The project includes a Nix flake setup that supports:

1. Development environment via `nix develop`
2. Direct usage via `nix run github:utensils/hexalith` (CLI)
3. Direct web interface usage via `nix run github:utensils/hexalith#web`

The flake uses a rustup-based approach for maximum compatibility across environments. The implementation:
- Creates temporary directories for clean builds
- Sets up an isolated rustup environment
- Avoids compatibility issues with Rust compiler flags
- Preserves all command line options when running via `nix run`

## Project Guidelines

- Never make backup files, we use git so they are never needed