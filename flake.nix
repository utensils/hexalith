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

        # Use a specific stable Rust version rather than latest
        rustToolchain = pkgs.rust-bin.stable."1.78.0".default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" "llvm-tools-preview" ];
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
          grcov
          lcov
          nixpkgs-fmt
          # Extra development tools for web server
          nodePackages.prettier
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
              name = "web-run";
              category = "web";
              help = "Run the web interface";
              command = "${rustToolchain}/bin/cargo run --bin hexweb";
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
                ${rustToolchain}/bin/cargo run --bin hexlogogen -- --format svg --verbose logo.svg
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
              name = "rs-coverage";
              category = "development";
              help = "Generate coverage report and open in browser";
              command = ''
                # Clean up previous runs
                rm -rf ./target/coverage/
                mkdir -p ./target/coverage/
                find . -name "*.profraw" -delete

                # Set up coverage environment
                export CARGO_INCREMENTAL=0
                export RUSTFLAGS="-Cinstrument-coverage -Ccodegen-units=1 -Copt-level=0"
                export LLVM_PROFILE_FILE="hexalith-%p-%m.profraw"
                
                # Run tests to generate coverage data
                echo "Running tests to generate coverage data..."
                ${rustToolchain}/bin/cargo test

                # Generate HTML report
                echo "Generating coverage report..."
                # Enable branch tracking with llvm instrumentation data
                ${pkgs.grcov}/bin/grcov . \
                  --binary-path ./target/debug/deps/ \
                  --source-dir . \
                  --output-path ./target/coverage/ \
                  --output-type html \
                  --branch \
                  --ignore "target/*" \
                  --ignore "tests/*" \
                  --ignore-not-existing \
                  --llvm \
                  --excl-br-line "unreachable|if (self.is_empty())" \
                  --excl-start "fn main" \
                  --keep-only "src/*"
                  
                # Also generate a JSON summary for easier parsing
                ${pkgs.grcov}/bin/grcov . \
                  --binary-path ./target/debug/deps/ \
                  --source-dir . \
                  --output-path ./target/coverage/summary.json \
                  --output-type covdir \
                  --branch \
                  --ignore "target/*" \
                  --ignore "tests/*" \
                  --ignore-not-existing \
                  --llvm \
                  --excl-br-line "unreachable|if (self.is_empty())" \
                  --excl-start "fn main" \
                  --keep-only "src/*"

                # Print text summary from the coverage.json file
                echo ""
                echo "Coverage Summary:"
                echo "================="
                percentage=$(cat ./target/coverage/html/coverage.json | grep -o '"message":"[0-9.]*%' | cut -d':' -f2 | tr -d '"')
                echo "Overall coverage: $percentage"
                echo ""

                echo "Coverage report available at ./target/coverage/html/index.html"
                echo "Opening coverage report in browser..."
                open ./target/coverage/html/index.html 2>/dev/null || xdg-open ./target/coverage/html/index.html 2>/dev/null || true
                
                # Clean up
                find . -name "*.profraw" -delete
              '';
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

        # Create simple wrapper scripts that use cargo to run the binaries
        
        # Default CLI application
        apps.default = {
          type = "app";
          program = toString (pkgs.writeShellScript "hexalith-cli" ''
            export PATH="${pkgs.git}/bin:$PATH"
            
            # Always use a clean directory to avoid build conflicts
            REPO_DIR=$(mktemp -d)
            
            # Clean up on exit
            trap "rm -rf $REPO_DIR" EXIT
            
            echo "Cloning hexalith repository to temporary directory..."
            ${pkgs.git}/bin/git clone --depth 1 https://github.com/utensils/hexalith.git "$REPO_DIR" >/dev/null 2>&1
            
            cd "$REPO_DIR"
            
            echo "Generating logo..."
            # Clear any Rust environment variables that could interfere
            unset RUSTFLAGS RUSTDOCFLAGS CARGO_RUSTFLAGS CARGO_UNSTABLE_SPARSE_REGISTRY RUSTC_BOOTSTRAP
            
            # Set clean environment
            export CARGO_TERM_COLOR="always"
            
            # Run using the pinned stable toolchain
            ${rustToolchain}/bin/cargo run --bin hexlogogen -- --format svg --verbose logo.svg "$@"
          '');
        };
        
        # CLI as a named app
        apps.cli = self.apps.${system}.default;
        
        # Web interface as a named app
        apps.web = {
          type = "app";
          program = toString (pkgs.writeShellScript "hexalith-web" ''
            export PATH="${pkgs.git}/bin:$PATH"
            
            # Always use a clean directory to avoid build conflicts
            REPO_DIR=$(mktemp -d)
            
            # Clean up on exit
            trap "rm -rf $REPO_DIR" EXIT
            
            echo "Cloning hexalith repository to temporary directory..."
            ${pkgs.git}/bin/git clone --depth 1 https://github.com/utensils/hexalith.git "$REPO_DIR" >/dev/null 2>&1
            
            cd "$REPO_DIR"
            
            echo "Starting web interface..."
            # Clear any Rust environment variables that could interfere
            unset RUSTFLAGS RUSTDOCFLAGS CARGO_RUSTFLAGS CARGO_UNSTABLE_SPARSE_REGISTRY RUSTC_BOOTSTRAP
            
            # Set clean environment
            export CARGO_TERM_COLOR="always"
            
            # Run using the pinned stable toolchain
            ${rustToolchain}/bin/cargo run --bin hexweb
          '');
        };
      }
    );
}