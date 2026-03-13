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
"${RUNNER[@]}" -- --format svg --seed 31415 --theme classic "$OUT_DIR/symmetric-classic-seed-31415.svg"
"${RUNNER[@]}" -- --format png --seed 31415 --theme classic "$OUT_DIR/symmetric-classic-seed-31415.png"
"${RUNNER[@]}" -- --format svg --seed 27182 --theme google --grid-size 5 --shapes 4 "$OUT_DIR/symmetric-google-seed-27182.svg"
"${RUNNER[@]}" -- --format png --seed 27182 --theme google --grid-size 5 --shapes 4 "$OUT_DIR/symmetric-google-seed-27182.png"
"${RUNNER[@]}" -- --format svg --seed 16180 --theme rainbow --grid-size 6 --shapes 5 "$OUT_DIR/symmetric-rainbow-seed-16180.svg"
"${RUNNER[@]}" -- --format png --seed 16180 --theme rainbow --grid-size 6 --shapes 5 "$OUT_DIR/symmetric-rainbow-seed-16180.png"
