#!/bin/bash

# Simple coverage script for Matrix Rust Project
# Generates coverage report with HTML output

set -e

echo "Running code coverage analysis..."

# Ensure coverage directory exists
mkdir -p coverage

# Generate coverage with HTML output only
cargo tarpaulin --out Html --output-dir coverage --verbose
