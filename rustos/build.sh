#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=== Building RustOS Kernel ==="
cargo build -Zjson-target-spec --package rustos

mkdir -p iso/boot/grub
mkdir -p build

echo "=== Copying Kernel to ISO Tree ==="
cp target/i686-rustos/debug/rustos iso/boot/kernel.elf

echo "=== Generating Bootable ISO with grub-mkrescue ==="
grub-mkrescue -o build/rustos.iso iso

echo "=== Build Complete: build/rustos.iso ==="
