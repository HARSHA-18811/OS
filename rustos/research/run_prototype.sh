#!/usr/bin/env bash
# Shell runner for host-side Producer-Consumer prototype
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/rust-prototype"

echo "=== Running Producer-Consumer Research Prototype (Host) ==="
# Determine host target (e.g. x86_64-unknown-linux-gnu or default)
HOST_TARGET=$(rustc -vV | grep "host:" | cut -d' ' -f2)
cargo run --release --target "$HOST_TARGET"

echo ""
echo "=== Prototype Execution Complete! Check research/rust-prototype/results.csv ==="
