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

# Restored polished emblem defaults rendered with the expanded theme set.
"${RUNNER[@]}" -- --format png --seed 1201 --theme ink --grid-size 4 --shapes 4 "$OUT_DIR/emblem-ink-seed-1201.png"
"${RUNNER[@]}" -- --format png --seed 2048 --theme horizon --grid-size 4 --shapes 4 "$OUT_DIR/emblem-horizon-seed-2048.png"
"${RUNNER[@]}" -- --format png --seed 2206 --theme ember --grid-size 4 --shapes 4 "$OUT_DIR/emblem-ember-seed-2206.png"
"${RUNNER[@]}" -- --format png --seed 3318 --theme orchid --grid-size 4 --shapes 4 "$OUT_DIR/emblem-orchid-seed-3318.png"
"${RUNNER[@]}" -- --format png --seed 4484 --theme citrus --grid-size 4 --shapes 4 "$OUT_DIR/emblem-citrus-seed-4484.png"
"${RUNNER[@]}" -- --format png --seed 5150 --theme classic --grid-size 4 --shapes 4 "$OUT_DIR/emblem-classic-seed-5150.png"

# Glyph-shaped emblem examples.
"${RUNNER[@]}" -- --format png --seed 6001 --theme ember --letter J "$OUT_DIR/letter-j-ember-seed-6001.png"
"${RUNNER[@]}" -- --format svg --seed 6001 --theme ember --letter J "$OUT_DIR/letter-j-ember-seed-6001.svg"
"${RUNNER[@]}" -- --format png --seed 7123 --theme horizon --letter A "$OUT_DIR/letter-a-horizon-seed-7123.png"
"${RUNNER[@]}" -- --format png --seed 8450 --theme ink --letter M "$OUT_DIR/letter-m-ink-seed-8450.png"
