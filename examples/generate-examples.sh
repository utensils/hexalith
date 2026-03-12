#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="$ROOT_DIR/examples/generated"

mkdir -p "$OUT_DIR"

if command -v nix >/dev/null 2>&1; then
  RUNNER=(nix develop -c cargo run --)
else
  RUNNER=(cargo run --)
fi

"${RUNNER[@]}" -- --format svg --seed 42 --theme classic "$OUT_DIR/classic-seed-42.svg"
"${RUNNER[@]}" -- --format png --seed 42 --theme classic "$OUT_DIR/classic-seed-42.png"
"${RUNNER[@]}" -- --format svg --seed 2024 --theme google --classic "$OUT_DIR/google-classic-seed-2024.svg"
"${RUNNER[@]}" -- --format png --seed 2024 --theme google --classic "$OUT_DIR/google-classic-seed-2024.png"
"${RUNNER[@]}" -- --format svg --seed 777 --theme rainbow --grid-size 6 --shapes 5 "$OUT_DIR/rainbow-grid6-seed-777.svg"
"${RUNNER[@]}" -- --format png --seed 777 --theme rainbow --grid-size 6 --shapes 5 "$OUT_DIR/rainbow-grid6-seed-777.png"
