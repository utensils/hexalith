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
"${RUNNER[@]}" -- --format svg --seed 9102 --theme google --grid-size 4 --shapes 4 "$OUT_DIR/emblem-google-seed-9102.svg"
"${RUNNER[@]}" -- --format png --seed 9102 --theme google --grid-size 4 --shapes 4 "$OUT_DIR/emblem-google-seed-9102.png"
"${RUNNER[@]}" -- --format svg --seed 8808 --theme blues --grid-size 4 --shapes 4 "$OUT_DIR/emblem-blues-seed-8808.svg"
"${RUNNER[@]}" -- --format png --seed 8808 --theme blues --grid-size 4 --shapes 4 "$OUT_DIR/emblem-blues-seed-8808.png"
"${RUNNER[@]}" -- --format svg --seed 3318 --theme classic --grid-size 4 --shapes 4 "$OUT_DIR/emblem-classic-seed-3318.svg"
"${RUNNER[@]}" -- --format png --seed 3318 --theme classic --grid-size 4 --shapes 4 "$OUT_DIR/emblem-classic-seed-3318.png"
