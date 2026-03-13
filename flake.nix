{
  description = "Hexalith - Modern geometric logo generator in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
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
          extensions = [ "clippy" "rust-analyzer" "rust-src" "rustfmt" ];
        };
      in {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustToolchain
            cargo-edit
            cargo-watch
            nixfmt-rfc-style
          ];

          RUST_BACKTRACE = "1";

          shellHook = ''
            echo "hexalith dev shell: Rust stable (${rustToolchain.pname or "rust"})"
          '';
        };

        apps.default = {
          type = "app";
          program = toString (pkgs.writeShellApplication {
            name = "hexalith";
            runtimeInputs = [ rustToolchain ];
            text = ''
              exec cargo run --bin hexalith -- "$@"
            '';
          });
        };

        apps.cli = self.apps.${system}.default;
        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
