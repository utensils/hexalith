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
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
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
          rust-analyzer
          cargo-watch
          cargo-edit
          nixpkgs-fmt
        ];

      in rec {
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;
          packages = devInputs;

          shellHook = ''
            echo "🦀 Hexalith development environment"
            echo "Run 'cargo build' to build the project"
          '';
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