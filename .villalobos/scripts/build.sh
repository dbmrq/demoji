#!/bin/bash
# Build script for demoji - wraps Cargo build
set -e
cd "$(dirname "$0")/../.."  # Navigate to project root

# Check if Cargo.toml exists (project scaffolded)
if [ ! -f "Cargo.toml" ]; then
    echo "ERROR: Cargo.toml not found. Project not yet scaffolded."
    echo "Run Task 1.1 to initialize the Rust project first."
    exit 1
fi

# Run cargo build
echo "Building demoji..."
cargo build

echo "Build successful!"

