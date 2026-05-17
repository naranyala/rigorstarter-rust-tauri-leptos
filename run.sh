#!/usr/bin/env bash

# Ensure we are in the project root
set -e

echo "Checking code style and quality..."
# Run cargo fmt to ensure consistent formatting
cargo fmt

# Run cargo clippy to check for common mistakes
if ! cargo clippy -- -D warnings; then
    echo "------------------------------------------------------------"
    echo "❌ CLIPPY CHECK FAILED"
    echo "Please fix the linting errors before continuing."
    echo "------------------------------------------------------------"
    exit 1
fi

echo "Checking frontend build health..."
# Run a fast build check without serving
if ! trunk build --quiet; then
    echo "------------------------------------------------------------"
    echo "❌ FRONTEND BUILD FAILED"
    echo "Please fix the Rust compilation errors in the terminal above."
    echo "------------------------------------------------------------"
    exit 1
fi

echo "✅ Code quality and frontend build healthy. Starting development mode..."
rm -rf dist
npx @tauri-apps/cli dev
