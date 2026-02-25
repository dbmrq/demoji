#!/bin/bash
# Test script for demoji - wraps Cargo test
set -e
cd "$(dirname "$0")/../.."  # Navigate to project root

# Check if Cargo.toml exists (project scaffolded)
if [ ! -f "Cargo.toml" ]; then
    echo "ERROR: Cargo.toml not found. Project not yet scaffolded."
    echo "Run Task 1.1 to initialize the Rust project first."
    exit 1
fi

# Run cargo test
echo "Running tests..."
cargo test

echo "All tests passed!"

