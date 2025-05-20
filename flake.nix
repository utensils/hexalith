{
  description = "Hexalith - Modern geometric logo generator in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    devshell = {
      url = "github:numtide/devshell";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, devshell, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ 
          (import rust-overlay) 
          devshell.overlays.default
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
          targets = [ ];
        };

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; [
          rustToolchain
        ];

        devInputs = with pkgs; [
          # Remove rust-analyzer from here since it's already part of rustToolchain
          cargo-watch
          cargo-edit
          nixpkgs-fmt
        ];

      in rec {
        devShells.default = pkgs.devshell.mkShell {
          name = "hexalith";
          packages = buildInputs ++ nativeBuildInputs ++ devInputs;
          
          env = [
            { 
              name = "RUST_BACKTRACE"; 
              value = "1"; 
            }
          ];

          commands = [
            {
              name = "build";
              category = "development";
              help = "Build the project";
              command = "cargo build";
            }
            {
              name = "run";
              category = "development";
              help = "Run the project";
              command = "cargo run -- \"$@\"";
            }
            {
              name = "logo";
              category = "hexalith";
              help = "Generate a logo with custom options";
              command = ''
                echo "Generating logo..."
                GRID_SIZE="''${1:-6}"
                SHAPES="''${2:-3}"
                OPACITY="''${3:-0.8}"
                FORMAT="''${4:-svg}"
                OUTPUT="''${5:-logo.svg}"
                
                cargo run -- --grid-size "$GRID_SIZE" --shapes "$SHAPES" --opacity "$OPACITY" --format "$FORMAT" --verbose "$OUTPUT"
              '';
            }
            {
              name = "example";
              category = "development";
              help = "Run the simple example";
              command = "cargo run --example simple";
            }
            {
              name = "fix-tests";
              category = "development";
              help = "Fix broken test imports";
              command = ''
                # Fix grid tests
                sed -i.bak 's/use super::geometry::{HexGrid, Point};/use crate::generator::grid::geometry::{HexGrid, Point};/' ./src/generator/grid/tests.rs
                sed -i.bak 's/use super::triangular::TriangularGrid;/use crate::generator::grid::triangular::TriangularGrid;/' ./src/generator/grid/tests.rs
                
                # Fix utils tests
                sed -i.bak 's/use super::\*;/use crate::utils::{uuid_to_seed, default_color_palette};/' ./src/utils/tests.rs
                
                # Remove backup files
                find . -name "*.bak" -delete
                
                echo "Fixed test imports"
              '';
            }
            {
              name = "test";
              category = "development";
              help = "Run tests";
              command = "cargo test";
            }
            {
              name = "fmt";
              category = "development";
              help = "Format code";
              command = "${pkgs.rustfmt}/bin/rustfmt $(find src examples -name '*.rs')";
            }
            {
              name = "check";
              category = "development";
              help = "Check for compilation errors";
              command = "cargo check";
            }
            {
              name = "clippy";
              category = "development";
              help = "Run linter";
              command = "cargo clippy";
            }
            {
              name = "release";
              category = "build";
              help = "Build for release";
              command = "cargo build --release";
            }
          ];

          # Configure devshell welcome and menu
          motd = ''
            🦀 Hexalith - Modern geometric logo generator in Rust
          '';
          
          # Enable the menu
          menu.enable = true;
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "hexlogogen";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          
          nativeBuildInputs = nativeBuildInputs;
          buildInputs = buildInputs;

          meta = with pkgs.lib; {
            description = "Modern geometric logo generator in Rust";
            homepage = "https://github.com/utensils/hexalith";
            license = licenses.mit;
            maintainers = [];
          };
        };
      }
    );
}