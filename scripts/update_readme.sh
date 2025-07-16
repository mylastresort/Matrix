#!/bin/bash

# Script to update README with coverage data
set -e

echo "Generating coverage data for README update..."

# Generate coverage and capture output
COVERAGE_OUTPUT=$(cargo tarpaulin --out Stdout 2>/dev/null || echo "")

if [ -z "$COVERAGE_OUTPUT" ]; then
    echo "Failed to generate coverage data"
    exit 1
fi

# Extract overall coverage
OVERALL_LINE=$(echo "$COVERAGE_OUTPUT" | grep "% coverage" | tail -1)
OVERALL_COVERAGE=$(echo "$OVERALL_LINE" | sed -n 's/^\([0-9]\+\.[0-9]\+%\) coverage.*/\1/p')
OVERALL_LINES=$(echo "$OVERALL_LINE" | sed -n 's/.* \([0-9]\+\/[0-9]\+\) lines covered.*/\1/p')

echo "Overall coverage: $OVERALL_COVERAGE ($OVERALL_LINES)"

# Prepare coverage percentage for URL (replace % with %25)
COVERAGE_PERCENT=$(echo "$OVERALL_COVERAGE" | sed 's/%/%25/g')

# Update the coverage badge in README.md using sed
sed -i "s|<img src=\"https://img\.shields\.io/badge/coverage-[0-9]\+\.[0-9]\+%25-[a-z]\+\" alt=\"Coverage\">|<img src=\"https://img.shields.io/badge/coverage-${COVERAGE_PERCENT}-orange\" alt=\"Coverage\">|g" README.md

echo "README.md updated successfully"