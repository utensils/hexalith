# Hexalith

Hexalith is a CLI-first geometric logo generator written in Rust. It builds abstract marks from connected triangular cells on a hexagonal grid and exports either SVG or PNG.

## Current shape of the project

- `src/generator/` contains the core grid, color, classic-mode, and shape-generation logic.
- `src/svg/` turns generated shapes into SVG paths or classic triangle polygons.
- `src/png/` rasterizes the SVG output into PNG with `resvg`.
- `src/cli/` is the only application interface now.

The web frontend has been removed. The repo is intentionally focused on local CLI iteration while the rendering quality is still being improved.

## Features

- Random or seeded logo generation
- SVG or PNG output
- Symmetry-first modern mode with six-way rotational composition by default
- Classic mode with a fixed 24-triangle layout
- Multiple color themes: `classic`, `google`, `blues`, `greens`, `reds`, `purples`, `rainbow`
- Clipboard copy support
- Automatic parent directory creation for output paths

## Local run

### With Nix

The repo flake provides the current stable Rust toolchain via `rust-overlay`.

```bash
nix develop -c cargo run -- --format svg output/logo.svg
```

You can also use the app entrypoint directly:

```bash
nix run . -- --seed 42 --theme classic output/logo.png
```

### With a local Rust toolchain

```bash
cargo run -- --format svg output/logo.svg
```

The repo also includes [rust-toolchain.toml](/home/jamesbrink/Projects/utensils/hexalith/rust-toolchain.toml), so `cargo`/`rustup` will use the stable toolchain automatically outside Nix.

## CLI usage

```text
hexalith [OPTIONS] [OUTPUT]
```

Examples:

```bash
# Default PNG output
cargo run -- output/logo.png

# SVG with deterministic seed
cargo run -- --format svg --seed 42 output/logo.svg

# Classic mode
cargo run -- --classic --theme google output/classic-google.png

# Denser grid and more shapes
cargo run -- --grid-size 6 --shapes 5 --opacity 0.7 output/variant.png

# Symmetry-first modern default
cargo run -- --seed 4242 --theme blues output/symmetric-mark.png

# Copy SVG to clipboard instead of writing a file
cargo run -- --copy --format svg
```

Run `cargo run -- --help` for the full flag list.

## Example outputs

Generated sample renders belong in `examples/generated/`. To create them:

```bash
./examples/generate-examples.sh
```

The script uses `nix develop` when available and falls back to plain `cargo`.

## Development

Build:

```bash
nix develop -c cargo build
```

Test:

```bash
nix develop -c cargo test
```

Format:

```bash
nix develop -c cargo fmt
```

Lint:

```bash
nix develop -c cargo clippy --all-targets --all-features
```

## Rendering notes

Modern mode now builds logos from full rotational orbits of triangular cells. That gives the default output strong six-way symmetry, cleaner radial balance, and deterministic seeded layouts without introducing a large flag surface.
