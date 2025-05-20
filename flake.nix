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
              name = "rs-build";
              category = "development";
              help = "Build the project";
              command = "${rustToolchain}/bin/cargo build";
            }
            {
              name = "rs-run";
              category = "development";
              help = "Run the project";
              command = "${rustToolchain}/bin/cargo run -- \"$@\"";
            }
            {
              name = "logo";
              category = "hexalith";
              help = "Generate a logo using default settings";
              command = ''
                echo "Generating logo..."
                ${rustToolchain}/bin/cargo run -- --format svg --verbose logo.svg
              '';
            }
            {
              name = "rs-example";
              category = "development";
              help = "Run the simple example";
              command = "${rustToolchain}/bin/cargo run --example simple";
            }
            {
              name = "rs-fix-tests";
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
              name = "rs-test";
              category = "development";
              help = "Run tests";
              command = "${rustToolchain}/bin/cargo test";
            }
            {
              name = "rs-fmt";
              category = "development";
              help = "Format code";
              command = "${pkgs.rustfmt}/bin/rustfmt $(find src examples -name '*.rs')";
            }
            {
              name = "rs-check";
              category = "development";
              help = "Check for compilation errors";
              command = "${rustToolchain}/bin/cargo check";
            }
            {
              name = "rs-clippy";
              category = "development";
              help = "Run linter";
              command = "${rustToolchain}/bin/cargo-clippy";
            }
            {
              name = "rs-release";
              category = "build";
              help = "Build for release";
              command = "${rustToolchain}/bin/cargo build --release";
            }

          ];

          # Configure devshell welcome
          motd = ''
            🦀 Hexalith - Modern geometric logo generator in Rust
          '';
          
          # Show menu automatically on shell entry
          devshell.startup.menu.text = "menu";
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