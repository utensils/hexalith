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
          webhook
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
              help = "Generate a logo with custom options";
              command = ''
                echo "Generating logo..."
                GRID_SIZE="''${1:-6}"
                SHAPES="''${2:-3}"
                OPACITY="''${3:-0.8}"
                FORMAT="''${4:-svg}"
                OUTPUT="''${5:-logo.svg}"
                
                ${rustToolchain}/bin/cargo run -- --grid-size "$GRID_SIZE" --shapes "$SHAPES" --opacity "$OPACITY" --format "$FORMAT" --verbose "$OUTPUT"
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
            {
              name = "serve-logo";
              category = "hexalith";
              help = "Start a webhook server to serve random logos in the browser";
              command = ''
                # Create hooks.json configuration file
                cat > /tmp/hooks.json << 'EOF'
                [
                  {
                    "id": "logo",
                    "execute-command": "/tmp/generate_logo.sh",
                    "command-working-directory": "/tmp",
                    "pass-arguments-to-command": [],
                    "response-headers": [
                      {
                        "name": "Content-Type",
                        "value": "image/svg+xml"
                      }
                    ],
                    "include-command-output-in-response": true,
                    "response-message": ""
                  }
                ]
                EOF
                
                # Create the logo generation script
                cat > /tmp/generate_logo.sh << 'EOF'
                #!/bin/sh
                # Generate random parameters
                GRID_SIZE=$(( RANDOM % 8 + 3 ))  # Random grid size between 3-10
                SHAPES=$(( RANDOM % 5 + 1 ))     # Random shapes between 1-5
                OPACITY=$(awk -v min=0.5 -v max=1.0 'BEGIN{srand(); print min+rand()*(max-min)}')
                
                # Create a temporary directory for the cargo run
                TEMP_DIR=$(mktemp -d)
                cd "$TEMP_DIR"
                
                # Copy the project to the temporary directory
                cp -r ${toString ./.}/* .
                
                # Generate a random logo with our Rust application
                OUTPUT_FILE="/tmp/logo_output.svg"
                
                # Generate random parameters for a unique logo each time
                GRID_SIZE=$(( RANDOM % 8 + 3 ))  # Random grid size between 3-10
                SHAPES=$(( RANDOM % 5 + 1 ))     # Random shapes between 1-5
                OPACITY=$(awk -v min=0.5 -v max=1.0 'BEGIN{srand(); print min+rand()*(max-min)}')
                
                # Run the logo generator, redirecting build output to /dev/null
                ${rustToolchain}/bin/cargo run --quiet -- --format svg "$OUTPUT_FILE" 2>/dev/null
                
                # Output the SVG content with XML declaration
                if [ -f "$OUTPUT_FILE" ] && [ -s "$OUTPUT_FILE" ]; then
                  # Add XML declaration if it's missing
                  if ! grep -q '<?xml' "$OUTPUT_FILE"; then
                    echo '<?xml version="1.0" encoding="UTF-8"?>'
                  fi
                  cat "$OUTPUT_FILE"
                else
                  # Fallback SVG if generation fails
                  echo '<?xml version="1.0" encoding="UTF-8"?>'
                  echo '<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200">'
                  echo '  <rect width="200" height="200" fill="#f0f0f0"/>'
                  echo '  <text x="20" y="100" font-family="Arial" font-size="14">Logo generation failed</text>'
                  echo '</svg>'
                fi
                EOF
                
                chmod +x /tmp/generate_logo.sh
                
                echo "Starting webhook server on port 9000..."
                echo "Open http://localhost:9000/hooks/logo in your browser and refresh for new logos"
                ${pkgs.webhook}/bin/webhook -hooks /tmp/hooks.json -verbose -port 9000
              '';
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