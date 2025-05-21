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

## Project Guidelines

- Never make backup files, we use git so they are never needed