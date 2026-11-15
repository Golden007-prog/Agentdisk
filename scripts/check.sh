#!/usr/bin/env sh
# Runs the same checks as CI on macOS or Linux.  Usage: ./scripts/check.sh
set -eu
cd "$(dirname "$0")/.."
echo "== Format ==";  cargo fmt --all --check
echo "== Lint ==";    cargo clippy --all-targets -- -D warnings
echo "== Tests ==";   cargo test --all --locked
echo "== Build ==";   cargo build --release --locked -p agentdisk
echo "== Smoke =="
# Cargo puts the binary under CARGO_TARGET_DIR when it is set (relative paths are
# relative to the repo root, where this script runs).
bin="${CARGO_TARGET_DIR:-target}/release/agentdisk"
if [ ! -x "$bin" ]; then
  echo "error: smoke test binary not found at $bin" >&2
  exit 1
fi
# One command per line: under set -e a failure in an a && b chain does not stop the script.
"$bin" --version
"$bin" adapters >/dev/null
"$bin" size . >/dev/null
echo "All checks passed. Binary: $bin"
