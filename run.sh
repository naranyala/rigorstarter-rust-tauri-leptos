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
# Update CSS imports in index.html programmatically
./scripts/generate_css_imports.sh

# Use cargo check for a faster compilation check instead of a full trunk build
if ! cargo check --target wasm32-unknown-unknown; then
    echo "------------------------------------------------------------"
    echo "❌ FRONTEND CHECK FAILED"
    echo "Please fix the Rust compilation errors in the terminal above."
    echo "------------------------------------------------------------"
    exit 1
fi

echo "✅ Code quality and frontend check healthy. Starting development mode..."
npx @tauri-apps/cli dev
